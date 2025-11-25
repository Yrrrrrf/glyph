// src/lib.rs
#![allow(unused)]
#![allow(dead_code)]

use chumsky::prelude::*;
use serde::Serialize;
use wasm_bindgen::prelude::*;

mod ast;
mod semantics;
mod syntax;

use ast::Statement;
// Removed Flavor import, using validator directly
use semantics::validator::validate;
use syntax::{lexer::lexer, parser::parser, tokens::Token};

#[derive(Serialize)]
pub struct JsSymbolRecord {
    pub name: String,
    pub type_: String,
    pub data_type: String,
    pub value: u64,
    pub segment: String,
}

#[derive(Serialize)]
pub struct JsLineAnalysis {
    pub line_number: usize,
    pub is_correct: bool,
    pub error_message: Option<String>,
    pub instruction: String,
}

#[derive(Serialize)]
pub struct JsToken {
    pub element: String,
    pub category: String,
    pub detail: String,
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Serialize)]
pub struct JsCompilerResult {
    pub success: bool,
    pub tokens: Option<Vec<JsToken>>,
    pub errors: Vec<String>,
    pub program: Option<ast::Program>,
    pub symbol_table: Vec<JsSymbolRecord>,
    pub line_analysis: Vec<JsLineAnalysis>,
}

fn calculate_line(source: &str, offset: usize) -> usize {
    if offset == 0 {
        return 1;
    }
    let slice = &source[..offset.min(source.len())];
    let count = slice.lines().count();
    if slice.ends_with('\n') {
        count + 1
    } else {
        count
    }
    .max(1)
}

fn generate_line_analysis(
    source: &str,
    errors: &[String],
    error_spans: &[(usize, usize)],
) -> Vec<JsLineAnalysis> {
    let mut lines = Vec::new();
    for (i, raw_line) in source.lines().enumerate() {
        let line_num = i + 1;
        let mut error_msg = None;
        let mut is_correct = true;

        for (idx, (start, _end)) in error_spans.iter().enumerate() {
            if calculate_line(source, *start) == line_num {
                is_correct = false;
                error_msg = Some(errors[idx].clone());
                break; // Take the first error that hits this line (usually the root cause)
            }
        }

        lines.push(JsLineAnalysis {
            line_number: line_num,
            is_correct,
            error_message: error_msg,
            instruction: raw_line.to_string(),
        });
    }
    lines
}

fn generate_symbol_table(program: &ast::Program) -> Vec<JsSymbolRecord> {
    let mut table = Vec::new();
    let mut current_segment = "Unknown".to_string();

    for stmt in program {
        match stmt {
            Statement::Segment { name } => {
                current_segment = name.clone();
            }
            Statement::Label(name) => {
                table.push(JsSymbolRecord {
                    name: name.clone(),
                    type_: "Label".to_string(),
                    data_type: "None".to_string(),
                    value: 0,
                    segment: current_segment.clone(),
                });
            }
            Statement::Variable {
                name,
                directive,
                value: _,
            } => {
                table.push(JsSymbolRecord {
                    name: name.clone(),
                    type_: "Variable".to_string(),
                    data_type: directive.to_uppercase(),
                    value: 0,
                    segment: current_segment.clone(),
                });
            }
            Statement::Constant { name, value: _ } => {
                table.push(JsSymbolRecord {
                    name: name.clone(),
                    type_: "Constant".to_string(),
                    data_type: "Word".to_string(),
                    value: 0,
                    segment: current_segment.clone(),
                });
            }
            _ => {}
        }
    }
    table
}

#[wasm_bindgen]
pub fn analyze_full_program(source: &str) -> JsValue {
    let len = source.len();
    let (tokens_result, lex_errs) = lexer().parse(source).into_output_errors();

    let mut all_errors_msg = Vec::new();
    let mut all_error_spans = Vec::new();

    // 1. LEXER ERRORS
    for err in lex_errs {
        // Clean up the error message for the UI
        let msg = format!("[LEX] Unexpected input: {:?}", err.found().unwrap_or(&' '));
        all_errors_msg.push(msg);
        all_error_spans.push((err.span().start, err.span().end));
    }

    let js_tokens = if let Some(tokens) = &tokens_result {
        Some(
            tokens
                .iter()
                .map(|(token, span)| {
                    let line = calculate_line(source, span.start);
                    let raw_element = &source[span.start..span.end];
                    JsToken {
                        element: raw_element.to_string(),
                        category: token.category(),
                        detail: token.description(),
                        line,
                        start: span.start,
                        end: span.end,
                    }
                })
                .collect(),
        )
    } else {
        None
    };

    // If Lexer failed catastrophically, stop here
    if tokens_result.is_none() {
        let lines = generate_line_analysis(source, &all_errors_msg, &all_error_spans);
        return serde_wasm_bindgen::to_value(&JsCompilerResult {
            success: false,
            tokens: None,
            errors: all_errors_msg,
            program: None,
            symbol_table: vec![],
            line_analysis: lines,
        })
        .unwrap();
    }

    let tokens = tokens_result.unwrap();
    let token_stream = chumsky::input::Stream::from_iter(tokens.into_iter())
        .map(SimpleSpan::from(len..len), |(t, s)| (t, s));

    let (ast, parse_errs) = parser().parse(token_stream).into_output_errors();

    // 2. PARSER ERRORS
    for err in parse_errs {
        let msg = format!("[PAR] Invalid syntax or missing token");
        all_errors_msg.push(msg);
        all_error_spans.push((err.span().start, err.span().end));
    }

    let program = ast.clone();
    let mut symbol_table = Vec::new();

    if let Some(prog) = &program {
        symbol_table = generate_symbol_table(prog);

        let semantic_errs = validate(prog);

        // 3. SEMANTIC ERRORS
        for err in semantic_errs {
            let msg = format!("[SEM] {}", err.message);
            // TODO: Semantic errors in validator.rs should ideally return spans.
            // For now, we attach them to line 0 or rely on the UI to show global errors if no line matches.
            // But if `err.line` is implemented in validator, we use that.
            // Since our validator currently returns line:0, these might not attach perfectly to lines
            // unless we upgrade validator.rs to track spans.
            // For this requests context, we simply add them.
            all_errors_msg.push(msg);
            all_error_spans.push((0, 0));
        }
    }

    let line_analysis = generate_line_analysis(source, &all_errors_msg, &all_error_spans);
    let success = all_errors_msg.is_empty();

    serde_wasm_bindgen::to_value(&JsCompilerResult {
        success,
        tokens: js_tokens,
        errors: all_errors_msg,
        program,
        symbol_table,
        line_analysis,
    })
    .unwrap()
}

#[wasm_bindgen]
pub fn analyze_assembly(source: &str) -> JsValue {
    analyze_full_program(source)
}
