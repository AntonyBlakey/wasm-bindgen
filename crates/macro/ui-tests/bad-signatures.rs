use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo() -> &u32 {}

#[wasm_bindgen]
extern "wasm-bindgen" {
    fn foo(Foo(x): Foo);

    fn foo() -> &u32;
}

fn main() {}
