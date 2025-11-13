#![allow(dead_code)]
#![allow(unused)]
// #![allow()]

pub mod lex;
pub mod parse;
pub mod tokens;

use crate::lex::AssemblyLexer;
use wasm_bindgen::prelude::*;
// use crate::tokens::AssemblyToken;

#[wasm_bindgen]
extern "C" {
    // Import JavaScript's console.log function
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn analyze_assembly(source: &str) -> JsValue {
    let tokens: Vec<_> = AssemblyLexer::new(source).collect();
    // Convert to a format Svelte can use
    serde_wasm_bindgen::to_value(&tokens).unwrap_or(wasm_bindgen::JsValue::NULL)
}
