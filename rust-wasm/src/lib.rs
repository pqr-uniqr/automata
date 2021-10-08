use pitch_detection::{McLeodDetector, PitchDetector};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

// originally based on https://rustwasm.github.io/wasm-bindgen/examples/web-audio.html

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[wasm_bindgen] // this is an attribute
extern "C" {
    pub fn alert(s: &str);
}

// pub means function is offered as a library, no usage requirement
// within the project.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct WasmPitchDetector {
    sample_rate: usize,
    fft_size: usize,
    detector: McLeodDetector<f32>,
}

#[wasm_bindgen]
impl WasmPitchDetector {
    pub fn new(sample_rate: usize, fft_size: usize) -> WasmPitchDetector {
        let fft_pad = fft_size / 2;

        WasmPitchDetector {
            sample_rate,
            fft_size,
            detector: McLeodDetector::<f32>::new(fft_size, fft_pad),
        }
    }

    pub fn detect_pitch(&mut self, audio_samples: Vec<f32>) -> f32 {
        if audio_samples.len() < self.fft_size {
            panic!("Insufficient samples passed to detect_pitch(). Expected an array containing {} elements but got {}", self.fft_size, audio_samples.len());
        }

        // Include only notes that exceed a power threshold which relates to the
        // amplitude of frequencies in the signal. Use the suggested default
        // value of 5.0 from the library.
        const POWER_THRESHOLD: f32 = 5.0;

        // The clarity measure describes how coherent the sound of a note is. For
        // example, the background sound in a crowded room would typically be would
        // have low clarity and a ringing tuning fork would have high clarity.
        // This threshold is used to accept detect notes that are clear enough
        // (valid values are in the range 0-1).
        const CLARITY_THRESHOLD: f32 = 0.6;

        let optional_pitch = self.detector.get_pitch(
            &audio_samples,
            self.sample_rate,
            POWER_THRESHOLD,
            CLARITY_THRESHOLD,
        );

        match optional_pitch {
            Some(pitch) => pitch.frequency,
            None => 0.0,
        }
    }
}

/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

#[wasm_bindgen]
pub struct FmOsc {
    ctx: AudioContext,
    /// The primary oscillator.  This will be the fundamental frequency
    primary: web_sys::OscillatorNode,

    /// Overall gain (volume) control
    gain: web_sys::GainNode,

    /// Amount of frequency modulation
    fm_gain: web_sys::GainNode,

    /// The oscillator that will modulate the primary oscillator's frequency
    fm_osc: web_sys::OscillatorNode,

    /// The ratio between the primary frequency and the fm_osc frequency.
    ///
    /// Generally fractional values like 1/2 or 1/4 sound best
    fm_freq_ratio: f32,

    fm_gain_ratio: f32,
}

impl Drop for FmOsc {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl FmOsc {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<FmOsc, JsValue> {
        // instantiate audioContext, web audio's top level, entry point interface
        let ctx = web_sys::AudioContext::new()?;

        // create oscillators and knobs
        let primary = ctx.create_oscillator()?;
        let fm_osc = ctx.create_oscillator()?;
        let gain = ctx.create_gain()?;
        let fm_gain = ctx.create_gain()?;

        // set oscillator wave types and frequencies
        primary.set_type(OscillatorType::Sine);
        primary.frequency().set_value(440.0); // A4 note
        gain.gain().set_value(0.0); // starts muted
        fm_gain.gain().set_value(0.0); // no initial frequency modulation
        fm_osc.set_type(OscillatorType::Sine);
        fm_osc.frequency().set_value(0.0);

        // Connect the nodes up!

        // The primary oscillator is routed through the gain node, so that
        // it can control the overall output volume.
        primary.connect_with_audio_node(&gain)?;

        // Then connect the gain node to the AudioContext destination (aka
        // your speakers).
        gain.connect_with_audio_node(&ctx.destination())?;

        // The FM oscillator is connected to its own gain node, so it can
        // control the amount of modulation.
        fm_osc.connect_with_audio_node(&fm_gain)?;

        // Connect the FM oscillator to the frequency parameter of the main
        // oscillator, so that the FM node can modulate its frequency.
        fm_gain.connect_with_audio_param(&primary.frequency())?;

        // Start the oscillators!
        primary.start()?;
        fm_osc.start()?;

        Ok(FmOsc {
            ctx,
            primary,
            gain,
            fm_gain,
            fm_osc,
            fm_freq_ratio: 0.0,
            fm_gain_ratio: 0.0,
        })
    }

    /// Sets the gain for this oscillator, between 0.0 and 1.0.
    #[wasm_bindgen]
    pub fn set_gain(&self, mut gain: f32) {
        if gain > 1.0 {
            gain = 1.0;
        }
        if gain < 0.0 {
            gain = 0.0;
        }
        self.gain.gain().set_value(gain);
    }

    #[wasm_bindgen]
    pub fn set_primary_frequency(&self, freq: f32) {
        self.primary.frequency().set_value(freq);

        // The frequency of the FM oscillator depends on the frequency of the
        // primary oscillator, so we update the frequency of both in this method.
        self.fm_osc.frequency().set_value(self.fm_freq_ratio * freq);
        self.fm_gain.gain().set_value(self.fm_gain_ratio * freq);
    }

    #[wasm_bindgen]
    pub fn set_note(&self, note: u8) {
        let freq = midi_to_freq(note);
        self.set_primary_frequency(freq);
    }

    /// This should be between 0 and 1, though higher values are accepted.
    #[wasm_bindgen]
    pub fn set_fm_amount(&mut self, amt: f32) {
        self.fm_gain_ratio = amt;

        self.fm_gain
            .gain()
            .set_value(self.fm_gain_ratio * self.primary.frequency().value());
    }

    /// This should be between 0 and 1, though higher values are accepted.
    #[wasm_bindgen]
    pub fn set_fm_frequency(&mut self, amt: f32) {
        self.fm_freq_ratio = amt;
        self.fm_osc
            .frequency()
            .set_value(self.fm_freq_ratio * self.primary.frequency().value());
    }
}

// https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub struct Destroyer {
    ctx: AudioContext,

    // TODO consider using AudioWorkletNode - from spec:
    // "If sample-accurate playback of network- or disk-backed assets is required, an implementer
    // should use AudioWorkletNode to implement playback."
    audio_src: web_sys::AudioBufferSourceNode,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn process_audio(var: f32) {}

#[wasm_bindgen]
impl Destroyer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Destroyer, JsValue> {
        log("ctor de la Destroyer");

        // ctor() audioContext, top lvl entr pt of Web Audio
        let ctx = web_sys::AudioContext::new()?;
        // TODO hw set opt ctor args?
        // ? y detune, playback r/o? dynmc ctrl impsbl?
        // -> dem setby ctor args
        let audio_src = ctx.create_buffer_source()?;
        // ? hw fil buf?
        // -> copyToChannel(Float32Array source,
        //              unsigned long channelNumber,
        //              optional unsigned long bufferOffSet)
        let buf = ctx.create_buffer(
            /* unsigned long numChannel*/ 2, /* unsigned long length */ 1,
            /* float smpl rate */ 3300.0,
        );

        /* FIXME buf not yet used */

        // retriv <audio>, mk HTMLMediaElementSource
        let wndw = web_sys::window().expect("global window missing");
        let doc = wndw.document().expect("document missing");

        let script_processor_node = ctx.create_script_processor().unwrap();
        log(&format!(
            "buffer size: {:?}",
            script_processor_node.buffer_size()
        )); // 1024
        script_processor_node.connect_with_audio_node(&ctx.destination())?;
        // script_processor_node.set_onaudioprocess(); // Option<&Function>

        let tracks = vec![
            "smplz/tv_angel_guitar_0.flac",
            "smplz/tv_angel_drums_0.flac",
        ];
        for track in tracks.iter() {
            let media_element = doc
                .create_element("audio")?
                .dyn_into::<web_sys::HtmlMediaElement>()
                .unwrap();
            media_element.set_attribute("src", track);
            media_element.set_attribute("loop", "true");
            media_element.set_loop(true);

            let media_node = ctx
                .create_media_element_source(&media_element)
                .expect("media element not found");

            // media_node.connect_with_audio_node(script_processor_node)?;

            media_element.play()?;
        }

        // how to get a buffer audio source out of media element
        //
        // https://github.com/WebAudio/web-audio-api/issues/1872
        // https://stackoverflow.com/questions/11292076 - create a
        // https://www.w3.org/2011/audio/wiki/Spec_Differences#Reading_Data_from_a_Media_Element
        //

        let val = doc
            .get_element_by_id("paragraphId")
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap();
        web_sys::console::log_2(&"URL: %s".into(), &JsValue::from_str(&val.inner_text()));

        /*
        // ? hw load snd fyl 2 Float32Array? encoding suppted?
        // aif no, flac ya
        let floatArr
        buf.copy_to_channel(floatArr, 1, 0);

        // TODO <- audioBuffer
        audio_src.buffer().set_value();
        */

        // TODO configure audio buffer src node

        Ok(Destroyer { ctx, audio_src })
    }

    #[wasm_bindgen]
    pub fn power_to_kill(&self) {
        log("power_to_kill");
    }
}
