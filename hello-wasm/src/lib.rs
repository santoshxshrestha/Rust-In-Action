use wasm_bindgen::prelude::*;

#[wasm_bindgen]
unsafe extern "C" {
    pub fn alert(s: &str);
    pub fn console_log(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn log_message(message: &str) {
    console_log(message);
}
