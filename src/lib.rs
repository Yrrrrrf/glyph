// src/lib.rs
#![allow(dead_code)]
#![allow(unused)]

pub mod analysis;
pub mod lex;
pub mod parse;
pub mod tokens;

use crate::analysis::Analyzer;
use crate::lex::AssemblyLexer;
use crate::tokens::{AssemblyToken, Token};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[derive(Serialize)]
pub struct TokenOutput {
    pub element: String,
    pub category: String,
    pub detail: String,
    pub line: usize,
}

#[wasm_bindgen]
pub fn analyze_assembly(source: &str) -> JsValue {
    let results: Vec<TokenOutput> = AssemblyLexer::new(source)
        .map(|(token, line)| TokenOutput {
            element: token.to_string(),
            category: token.category().to_string(),
            detail: token.detailed_type(),
            line,
        })
        .collect();

    serde_wasm_bindgen::to_value(&results).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn analyze_full_program(source: &str) -> JsValue {
    // 1. Lexer
    let tokens_with_lines: Vec<_> = AssemblyLexer::new(source).collect();

    // 2. Analyzer (Fase 2)
    let mut analyzer = Analyzer::new();
    let result = analyzer.analyze(tokens_with_lines);

    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}
