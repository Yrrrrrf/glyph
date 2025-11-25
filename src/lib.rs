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
use syntax::{
    lexer::lexer,
    parser::parser,
    tokens::{Token, constant}, // Import constant and Token
};

#[derive(Serialize)]
pub struct JsCompilerResult {
    pub success: bool,
    pub tokens: Option<Vec<JsToken>>,
    pub errors: Vec<String>,
    pub program: Option<ast::Program>,
}

#[derive(Serialize)]
pub struct JsToken {
    pub element: String,  // e.g., "128"
    pub category: String, // e.g., "Constant (Dec)"
    pub detail: String,   // e.g., "Constant(NumberDecimal(128))"
    pub line: usize,
}

// Helper function to match main.rs formatting logic
fn get_token_details(token: &Token) -> (String, String) {
    match token {
        Token::Instruction(s) => ("Instruction".to_string(), s.clone()),
        Token::Pseudoinstruction(s) => ("Pseudoinstruction".to_string(), s.clone()),
        Token::Register(s) => ("Register".to_string(), s.clone()),
        Token::Symbol(s) => ("Symbol".to_string(), s.clone()),
        Token::Constant(c) => match c {
            constant::Type::String(s) => ("Constant (String)".to_string(), format!("\"{}\"", s)),
            constant::Type::NumberDecimal(v) => ("Constant (Dec)".to_string(), v.to_string()),
            constant::Type::NumberHex(_, raw) => ("Constant (Hex)".to_string(), raw.clone()),
            constant::Type::NumberBinary(_, raw) => ("Constant (Bin)".to_string(), raw.clone()),
            constant::Type::Char(c) => ("Constant (Char)".to_string(), format!("'{}'", c)),
        },
        Token::Error(s) => ("Elemento inválido".to_string(), s.clone()),
        _ => ("Separator".to_string(), token.to_string()),
    }
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

    // Prepare tokens for JS, even if there are errors (if tokens exist)
    let js_tokens = if let Some(tokens) = &tokens_result {
        Some(
            tokens
                .iter()
                .map(|(t, span)| {
                    // Calculate line number
                    let line = source[..span.start].lines().count().max(1);

                    // Get friendly category and clean value
                    let (category, element) = get_token_details(t);

                    JsToken {
                        element,
                        category,
                        // Use Rust's Debug formatter {:?} to get the Enum detail
                        detail: format!("{:?}", t),
                        line,
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

    // 2. Parsing
    let token_stream = chumsky::input::Stream::from_iter(tokens.into_iter())
        .map(SimpleSpan::from(len..len), |(t, s)| (t, s));

    let (ast, parse_errs) = parser().parse(token_stream).into_output_errors();

    for err in parse_errs {
        js_errors.push(format!(
            "Parse Error: Expected {:?}, found {:?}",
            err.expected().collect::<Vec<_>>(),
            err.found()
        ));
    }

    if ast.is_none() {
        return serde_wasm_bindgen::to_value(&JsCompilerResult {
            success: false,
            tokens: js_tokens,
            errors: js_errors,
            program: None,
        })
        .unwrap();
    }

    let program = ast.unwrap();

    // 3. Validation
    let strict_mode = true;
    let flavor = Flavor::Masm;

    let semantic_errs = validate(&program, flavor, strict_mode);
    for err in semantic_errs {
        js_errors.push(format!("Semantic Error: {}", err.message));
    }

    let success = js_errors.is_empty();

    serde_wasm_bindgen::to_value(&JsCompilerResult {
        success,
        tokens: js_tokens,
        errors: js_errors,
        program: Some(program),
    })
    .unwrap()
}

// Keep this for legacy compatibility if needed
#[wasm_bindgen]
pub fn analyze_assembly(source: &str) -> JsValue {
    analyze_full_program(source)
}
