use wasm_bindgen::prelude::*;

#[wasm_bindgen(x)]
pub fn foo() {}

#[wasm_bindgen]
extern "wasm-bindgen" {
    #[wasm_bindgen(y)]
    fn bar();

    #[wasm_bindgen {  }]
    fn bar();
}

fn main() {}
