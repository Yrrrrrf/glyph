// src/lib.rs
#![allow(unused)]
#![allow(dead_code)]

use chumsky::prelude::*;
use serde::Serialize;
use std::cmp::Ordering;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

mod ast;
mod semantics;
mod syntax;

use ast::{LineNode, Statement};
use semantics::encoder::{pass_one, pass_two};
use semantics::validator::validate;
use syntax::{lexer::lexer, parser::parser, tokens::Token};

#[derive(Serialize)]
pub struct JsSymbolRecord {
    pub name: String,
    pub type_: String,
    pub data_type: String,
    pub value: u64,
    pub segment: String,
    pub line: usize,
}

#[derive(Serialize)]
pub struct JsLineAnalysis {
    pub line_number: usize,
    pub is_correct: bool,
    pub error_message: Option<String>,
    pub instruction: String,
    pub address: Option<String>,
    pub machine_code: Option<String>,
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
    semantic_errors: &HashMap<usize, String>,
    stmt_info: &HashMap<usize, (String, String)>,
) -> Vec<JsLineAnalysis> {
    let mut lines = Vec::new();
    for (i, raw_line) in source.lines().enumerate() {
        let line_num = i + 1;
        let mut error_msg = None;
        let mut is_correct = true;

        // 1. Check Parser/Lexer Errors (Span-based)
        for (idx, (start, _end)) in error_spans.iter().enumerate() {
            if calculate_line(source, *start) == line_num {
                is_correct = false;
                error_msg = Some(errors[idx].clone());
                break;
            }
        }

        // 2. Check Semantic Errors (Line-based)
        // Overwrites parser error if present, or adds if clean so far
        if let Some(msg) = semantic_errors.get(&line_num) {
            is_correct = false;
            error_msg = Some(msg.clone());
        }

        let (addr, code) = if let Some((a, c)) = stmt_info.get(&line_num) {
            if is_correct {
                (Some(a.clone()), Some(c.clone()))
            } else {
                (Some(a.clone()), None) // Keep address but hide code
            }
        } else {
            (None, None)
        };

        lines.push(JsLineAnalysis {
            line_number: line_num,
            is_correct,
            error_message: error_msg,
            instruction: raw_line.to_string(),
            address: addr,
            machine_code: code,
        });
    }
    lines
}

fn get_line_content(source: &str, offset: usize) -> &str {
    let start = source[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
    let end = source[offset..]
        .find('\n')
        .map(|i| offset + i)
        .unwrap_or(source.len());
    &source[start..end]
}

#[wasm_bindgen]
pub fn analyze_full_program(source: &str) -> JsValue {
    let result = analyze_full_program_struct(source);
    serde_wasm_bindgen::to_value(&result).unwrap()
}

pub fn analyze_full_program_struct(source: &str) -> JsCompilerResult {
    let len = source.len();
    let (tokens_result, lex_errs) = lexer().parse(source).into_output_errors();

    let mut all_errors_msg = Vec::new();
    let mut all_error_spans = Vec::new();

    for err in lex_errs {
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

    if tokens_result.is_none() {
        let lines = generate_line_analysis(
            source,
            &all_errors_msg,
            &all_error_spans,
            &HashMap::new(),
            &HashMap::new(),
        );
        return JsCompilerResult {
            success: false,
            tokens: None,
            errors: all_errors_msg,
            program: None,
            symbol_table: vec![],
            line_analysis: lines,
        };
    }

    let tokens = tokens_result.unwrap();
    let token_stream = chumsky::input::Stream::from_iter(tokens.into_iter())
        .map(SimpleSpan::from(len..len), |(t, s)| (t, s));

    let (ast, parse_errs) = parser().parse(token_stream).into_output_errors();

    for err in parse_errs {
        // Forensic analysis on the full line
        let line_content = get_line_content(source, err.span().start);
        let diag = semantics::diagnostics::diagnose_syntax_error(line_content);

        let msg = if diag != "Sintaxis inválida o token faltante" {
            format!("[PAR] {}", diag)
        } else {
            format!("[PAR] Invalid syntax or missing token")
        };

        all_errors_msg.push(msg);
        all_error_spans.push((err.span().start, err.span().end));
    }

    let program = ast.clone();

    if let Some(prog) = &program {
        for spanned in prog {
            if let LineNode::Error(_) = &spanned.node {
                // Forensic analysis on the full line (using span start is safe)
                let line_content = get_line_content(source, spanned.span.0);
                let specific_msg = semantics::diagnostics::diagnose_syntax_error(line_content);
                all_errors_msg.push(format!("[PAR] {}", specific_msg));
                all_error_spans.push(spanned.span);
            }
        }
    }
    let mut js_symbol_table = Vec::new();
    let mut stmt_info_map = HashMap::new();
    let mut semantic_error_map = HashMap::new();

    if let Some(prog) = &program {
        let (semantic_errs, mut symbol_info_map) = validate(prog);

        for err in semantic_errs {
            let msg = format!("[SEM] {}", err.message);
            // We store it in the map for line attribution
            // Note: If multiple errors on one line, last one wins or we append?
            // Let's overwrite for now or join.
            if let Some(existing) = semantic_error_map.get_mut(&err.line) {
                *existing = format!("{}; {}", existing, msg);
            } else {
                semantic_error_map.insert(err.line, msg.clone());
            }

            // Also add to main errors list for global status
            all_errors_msg.push(format!("Line {}: {}", err.line, err.message));
        }

        let address_map = pass_one(prog, &mut symbol_info_map);
        let machine_code_map = pass_two(prog, &address_map);

        for (name, info) in symbol_info_map {
            js_symbol_table.push(JsSymbolRecord {
                name: name,
                type_: format!("{:?}", info.type_),
                data_type: format!("{:?}", info.data_type),
                value: info.offset.unwrap_or(0),
                segment: info.segment,
                line: info.line_defined,
            });
        }
        js_symbol_table.sort_by(|a, b| a.name.cmp(&b.name));

        for (idx, spanned) in prog.iter().enumerate() {
            let line = calculate_line(source, spanned.span.0);

            let addr_str = if let Some(addr) = address_map.get(&idx) {
                format!("{:04X}", addr)
            } else {
                String::new()
            };

            let code_str = machine_code_map.get(&idx).cloned().unwrap_or_default();

            if !addr_str.is_empty() || !code_str.is_empty() {
                stmt_info_map.insert(line, (addr_str, code_str));
            }
        }
    }

    let line_analysis = generate_line_analysis(
        source,
        &all_errors_msg,
        &all_error_spans,
        &semantic_error_map,
        &stmt_info_map,
    );

    JsCompilerResult {
        success: all_errors_msg.is_empty(),
        tokens: js_tokens,
        errors: all_errors_msg,
        program,
        symbol_table: js_symbol_table,
        line_analysis: line_analysis,
    }
}
