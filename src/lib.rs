use wasm_bindgen::prelude::*;

// First example "Hello world" string
#[wasm_bindgen]
pub fn helloworld() -> String {
    String::from("Hello world from Rust!")
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn example() {
    log("Log from Rust");
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
