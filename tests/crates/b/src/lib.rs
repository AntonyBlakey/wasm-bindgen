extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "tests/wasm/duplicate_deps.js")]
extern "wasm-bindgen" {
    fn foo(x: u32);
}

pub fn test() {
    foo(10);
}
