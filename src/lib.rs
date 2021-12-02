use mail_parser::Message;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn convert(input: &str) -> String {
    let message = Message::parse(input.as_bytes());
    serde_json::to_string_pretty(&message).unwrap_or_default()
}
