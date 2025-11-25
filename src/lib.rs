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
use syntax::{lexer::lexer, parser::parser, tokens::Token};

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

// Helper function to calculate line number correctly
fn calculate_line(source: &str, offset: usize) -> usize {
    if offset == 0 {
        return 1;
    }
    let slice = &source[..offset];
    let count = slice.lines().count();
    if slice.ends_with('\n') {
        count + 1
    } else {
        count
    }
    .max(1)
}

#[wasm_bindgen]
pub fn analyze_full_program(source: &str) -> JsValue {
    let len = source.len();
    let (tokens_result, lex_errs) = lexer().parse(source).into_output_errors();

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
                    // 1. Get Exact Line Number using the robust logic
                    let line = calculate_line(source, span.start);

                    // 2. Extract Raw Content using Span
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
        })
        .unwrap();
    }

    let tokens = tokens_result.unwrap();

    // Check Parser
    let token_stream = chumsky::input::Stream::from_iter(tokens.into_iter())
        .map(SimpleSpan::from(len..len), |(t, s)| (t, s));

    let (ast, parse_errs) = parser().parse(token_stream).into_output_errors();

    for err in parse_errs {
        js_errors.push(format!("Parse Error: {:?}", err));
    }

    let program = ast.clone();

    // Semantic Check
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
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn analyze_assembly(source: &str) -> JsValue {
    analyze_full_program(source)
}
