use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "wasm-bindgen" {
	#[wasm_bindgen]
	pub fn foo() -> Result<JsValue, JsValue>;
}

fn main() {}
