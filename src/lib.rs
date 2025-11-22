#![allow(unused)]
#![allow(dead_code)]

// src/lib.rs
use chumsky::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod ast;
mod semantics;
mod syntax;

use semantics::validator::{Flavor, validate};
use syntax::{lexer::lexer, parser::parser};

#[derive(Serialize)]
pub struct JsCompilerResult {
    pub success: bool,
    pub tokens: Option<Vec<JsToken>>, // For debugging/visualization
    pub errors: Vec<String>,
    pub program: Option<ast::Program>, // AST (simplified for JS if needed)
}

#[derive(Serialize)]
pub struct JsToken {
    pub element: String,
    pub category: String,
    pub detail: String,
    pub line: usize,
}

#[wasm_bindgen]
pub fn analyze_full_program(source: &str) -> JsValue {
    // 1. Lexing
    let len = source.len();
    let (tokens_result, lex_errs) = lexer().parse(source).into_output_errors();

    let mut js_errors = Vec::new();

    // Convert lexer errors
    for err in lex_errs {
        js_errors.push(format!("Lexing Error: {:?}", err));
    }

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

    // Convert tokens to JS format for visualization
    let js_tokens: Vec<JsToken> = tokens
        .iter()
        .map(|(t, span)| {
            // Basic line estimation (can be improved by tracking newlines in spans)
            let line = source[..span.start].lines().count();
            JsToken {
                element: t.to_string(),
                category: format!("{:?}", t), // Rough category
                detail: t.to_string(),
                line,
            }
        })
        .collect();

    // 2. Parsing
    // We use .map() instead of .split_token_span() to avoid lifetime/trait inference issues with Stream
    let token_stream = chumsky::input::Stream::from_iter(tokens.into_iter())
        .map(SimpleSpan::from(len..len), |(t, s)| (t, s));

    let (ast, parse_errs) = parser().parse(token_stream).into_output_errors();

    for err in parse_errs {
        // We must collect() the expected tokens to print them
        js_errors.push(format!(
            "Parse Error: Expected {:?}, found {:?}",
            err.expected().collect::<Vec<_>>(),
            err.found()
        ));
    }

    if ast.is_none() {
        return serde_wasm_bindgen::to_value(&JsCompilerResult {
            success: false,
            tokens: Some(js_tokens),
            errors: js_errors,
            program: None,
        })
        .unwrap();
    }

    let program = ast.unwrap();

    // 3. Validation
    // We'll hardcode Hybrid preferences or pass them via args later
    let strict_mode = true;
    let flavor = Flavor::Masm;

    let semantic_errs = validate(&program, flavor, strict_mode);
    for err in semantic_errs {
        js_errors.push(format!("Semantic Error: {}", err.message));
    }

    let success = js_errors.is_empty();

    serde_wasm_bindgen::to_value(&JsCompilerResult {
        success,
        tokens: Some(js_tokens),
        errors: js_errors,
        // We can't easily serialize the full AST enum to JS without custom serde impls
        // for every node, so we might just return null here or implement a specific view struct.
        // For now, we return success status.
        program: Some(program),
    })
    .unwrap()
}

// Keep this for legacy compatibility if needed, or remove
#[wasm_bindgen]
pub fn analyze_assembly(source: &str) -> JsValue {
    analyze_full_program(source)
}
