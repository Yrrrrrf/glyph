// src/lib.rs
#![allow(unused)]
#![allow(dead_code)]

use chumsky::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod ast;
mod semantics;
mod syntax;

use semantics::validator::{Flavor, validate};
use syntax::{parser::parser, tokens::Token};

use crate::syntax::lexer;

#[derive(Serialize)]
pub struct JsCompilerResult {
    pub success: bool,
    pub tokens: Option<Vec<JsToken>>,
    pub errors: Vec<String>,
    pub program: Option<ast::Program>,
}

#[derive(Serialize)]
pub struct JsToken {
    pub element: String,  // The Exact Slice from Source Code
    pub category: String, // E.g. "Instruction"
    pub detail: String,   // E.g. "Aritmética"
    pub line: usize,
    pub start: usize, 
    pub end: usize,
}

#[wasm_bindgen]
pub fn analyze_full_program(source: &str) -> JsValue {
    let len = source.len();
    let (tokens_result, lex_errs) = lexer::lexer().parse(source).into_output_errors();

    let mut js_errors = Vec::new();
    for err in lex_errs {
        js_errors.push(format!("Lexing Error: {:?}", err));
    }

    // --- ENHANCED TOKEN GENERATION ---
    let js_tokens = if let Some(tokens) = &tokens_result {
        Some(
            tokens
                .iter()
                .map(|(token, span)| {
                    // 1. Get Line Number (Optimized? Not yet, but functional)
                    let line = source[..span.start].lines().count().max(1);

                    // 2. Extract Raw Content using Span
                    // This gives the exact text the user typed (e.g. "mOv", "0afH")
                    let raw_element = &source[span.start..span.end];

                    // 3. Use Token Methods for Metadata
                    let category = token.category();
                    let detail = token.description();

                    JsToken {
                        element: raw_element.to_string(),
                        category,
                        detail,
                        line,
                        start: span.start,
                        end: span.end,
                    }
                })
                .collect::<Vec<JsToken>>(),
        )
    } else {
        None
    };

    if tokens_result.is_none() {
        return serde_wasm_bindgen::to_value(&JsCompilerResult {
            success: false,
            tokens: None,
            errors: js_errors,
            program: None,
        }).unwrap();
    }

    let tokens = tokens_result.unwrap();

    // ... (Parsing logic remains, but we need to update parser.rs first) ...
    // Note: If you want to skip parsing while testing just the Lexer, you can comment this out temporarily.
    // For now, let's assume Parser updates are applied (Step 4).
    
    let token_stream = chumsky::input::Stream::from_iter(tokens.into_iter())
        .map(SimpleSpan::from(len..len), |(t, s)| (t, s));

    let (ast, parse_errs) = parser().parse(token_stream).into_output_errors();

    for err in parse_errs {
        js_errors.push(format!("Parse Error: {:?}", err));
    }

    // ... Validator ...
    let program = ast.clone();
    
    // Quick Semantic Check (Simplified)
    if let Some(prog) = &program {
         let flavor = Flavor::Masm;
         let semantic_errs = validate(prog, flavor, true);
         for err in semantic_errs {
             js_errors.push(format!("Semantic Error: {}", err.message));
         }
    }

    let success = js_errors.is_empty();

    serde_wasm_bindgen::to_value(&JsCompilerResult {
        success,
        tokens: js_tokens,
        errors: js_errors,
        program,
    }).unwrap()
}

#[wasm_bindgen]
pub fn analyze_assembly(source: &str) -> JsValue {
    analyze_full_program(source)
}