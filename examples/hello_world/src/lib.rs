use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "wasm-bindgen" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
