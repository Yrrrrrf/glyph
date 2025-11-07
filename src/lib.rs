use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {

    // Import JavaScript's console.log function
    // #[wasm_bindgen(js_namespace = console)]
    // fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
