use wasm_bindgen::prelude::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

#[wasm_bindgen] // this is an attribute
extern {
    pub fn alert(s: &str);
}

// pub means function is offered as a library, no usage requirement
// within the project.
#[wasm_bindgen]
pub fn greet(name: &str) { 
        alert(&format!("Hello, {}!", name));
}
