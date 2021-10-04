import("./pkg/wasm.js").then((rust_module /* rust_module */) => {
	rust_module.default().then((_) => {

        let infinite_destructive_empowerment = null;

		const destroy_button = document.getElementById("destroy");
		const audioElement = document.querySelector('audio');
		destroy_button.addEventListener("click", event => {
			// pass it into the audio context
			// const track = audioContext.createMediaElementSource(audioElement);

			if (infinite_destructive_empowerment === null) {
                infinite_destructive_empowerment = new rust_module.Destroyer();
                console.log(infinite_destructive_empowerment);
                infinite_destructive_empowerment.power_to_kill();
			} else {
				infinite_destructive_empowerment.free();
				infinite_destructive_empowerment = null;
			}
		});

        /*
		let fm = null;

		const play_button = document.getElementById("play");
		play_button.addEventListener("click", event => {
			if (fm === null) {
				fm = new rust_module.FmOsc();
				fm.set_note(50);
				fm.set_fm_frequency(0);
				fm.set_fm_amount(0);
				fm.set_gain(0.8);
			} else {
				fm.free();
				fm = null;
			}
		});

		const primary_slider = document.getElementById("primary_input");
		primary_slider.addEventListener("input", event => {
			if (fm) {
				fm.set_note(parseInt(event.target.value));
			}
		});

		const fm_freq = document.getElementById("fm_freq");
		fm_freq.addEventListener("input", event => {
			if (fm) {
				fm.set_fm_frequency(parseFloat(event.target.value));
			}
		});

		const fm_amount = document.getElementById("fm_amount");
		fm_amount.addEventListener("input", event => {
			if (fm) {
				fm.set_fm_amount(parseFloat(event.target.value));
			}
		});

        console.log('rust-wasm done');
        */

	})
});
