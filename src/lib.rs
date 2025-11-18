#![allow(dead_code)]
#![allow(unused)]

pub mod lex;
pub mod parse;
pub mod tokens;

use crate::lex::AssemblyLexer;
use crate::tokens::{AssemblyToken, Token};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    fn alert(s: &str);

    // fn console_error(s: &str);
    // fn console_warn(s: &str);

}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[derive(Serialize)]
struct WasmToken {
    element: String,
    token_type: String,
}

#[wasm_bindgen]
pub fn analyze_assembly(source: &str) -> JsValue {
    let tokens: Vec<WasmToken> = AssemblyLexer::new(source)
        .map(|token| WasmToken {
            element: match &token {
                AssemblyToken::Instruction(i) => i.to_string(),
                AssemblyToken::Pseudoinstruction(p) => p.to_string(),
                AssemblyToken::Register(r) => r.to_string(),
                AssemblyToken::Symbol(s) => s.to_string(),
                AssemblyToken::Constant(c) => c.to_string(),
                AssemblyToken::Punctuation(p) => p.to_string(),
                AssemblyToken::Invalid(s) => {
                    println!("Invalid token encountered: {}", s);
                    s.clone()
                },  // Invalid token handling
            },
            token_type: token.category().to_string(),
        })
        .collect();

    serde_wasm_bindgen::to_value(&tokens).unwrap_or(JsValue::NULL)
}
