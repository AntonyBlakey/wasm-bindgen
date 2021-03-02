use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/defined-in-js.js")]
extern "wasm-bindgen" {
    type MyClass;

    #[wasm_bindgen(constructor)]
    fn new() -> MyClass;

    #[wasm_bindgen(method, getter)]
    fn number(this: &MyClass) -> u32;
    #[wasm_bindgen(method, setter)]
    fn set_number(this: &MyClass, number: u32) -> MyClass;
    #[wasm_bindgen(method)]
    fn render(this: &MyClass) -> String;
}

// lifted from the `console_log` example
#[wasm_bindgen]
extern "wasm-bindgen" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(inline_js = "export function add(a, b) { return a + b; }")]
extern "wasm-bindgen" {
    fn add(a: u32, b: u32) -> u32;
}
#[wasm_bindgen(inline_js = "export function test() {}")]
extern "wasm-bindgen" {
    fn test();
}

#[wasm_bindgen]
pub fn greet(name: String) {
    log(&format!("Hello from {}!", name)); // should output "Hello from Rust!"

    let x = MyClass::new();
    assert_eq!(x.number(), add(40, 2));
    test();
    log(&x.render());
}
