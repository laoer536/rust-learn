mod utils;
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
  pub fn alert(s: &str);
}
#[wasm_bindgen]
pub fn say(name: &str) {
  alert(&format!("Hello, {}!", name));
}
