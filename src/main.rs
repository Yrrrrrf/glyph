// src/main.rs
#![allow(unused)]

mod ast;
mod semantics;
mod syntax;

use chumsky::prelude::*;
use std::collections::HashMap;
use std::env;
use std::fs;

// Import your modules
use semantics::encoder::{pass_one, pass_two};
use semantics::validator::validate;
use syntax::{lexer::lexer, parser::parser, tokens::Token};

fn main() {
    // 1. Read File
    let args: Vec<String> = env::args().collect();
    let filename = args
        .get(1)
        .expect("Please provide a filename: cargo run test.asm");
    let source = fs::read_to_string(filename).expect("Could not read file");

    println!("\n=== ASSEMBLING: {} ===\n", filename);

    // 2. LEXER
    let len = source.len();
    let (tokens_result, lex_errs) = lexer().parse(&source).into_output_errors();

    if !lex_errs.is_empty() {
        println!("❌ LEXER ERRORS:");
        for err in lex_errs {
            println!("  {:?}", err);
        }
        return;
    }

    let tokens = tokens_result.unwrap();

    // 3. PARSER
    let token_stream = chumsky::input::Stream::from_iter(tokens.into_iter())
        .map(SimpleSpan::from(len..len), |(t, s)| (t, s));

    let (ast_opt, parse_errs) = parser().parse(token_stream).into_output_errors();

    if !parse_errs.is_empty() {
        println!("❌ PARSER ERRORS:");
        for err in parse_errs {
            println!("  {:?}", err);
        }
        return;
    }

    let program = ast_opt.unwrap();

    // 4. VALIDATOR (Populate Symbol Table)
    // Note: This returns the symbol table with 'offset: None' initially
    let (semantic_errs, mut symbol_table) = validate(&program);

    if !semantic_errs.is_empty() {
        println!("⚠️ SEMANTIC ERRORS:");
        for err in semantic_errs {
            println!("  Line {}: {}", err.line, err.message);
        }
        // We continue even with semantic errors to test addressing,
        // unless you want to stop here.
    }

    // 5. PASS 1: ADDRESS DETERMINATION
    // This calculates addresses and updates the symbol_table offsets
    let address_map = pass_one(&program, &mut symbol_table);

    // 6. PASS 2: MACHINE CODE ENCODING
    let machine_code_map = pass_two(&program, &address_map);

    // ==========================================
    // OUTPUT: LISTING FILE VISUALIZATION
    // ==========================================
    println!("=== LISTING OUTPUT ===");
    println!(
        "{:<6} | {:<8} | {:<16} | {}",
        "Line", "Address", "Machine Code", "Source"
    );
    println!("{}", "-".repeat(80));

    // Split source into lines to print side-by-side
    for (line_idx, source_line) in source.lines().enumerate() {
        // We need to map source lines to AST nodes.
        // Since 'address_map' is indexed by AST Node Index, not Line Number,
        // we have to be careful.
        //
        // Ideally, we loop through the AST (program) and find which line it belongs to.
        // But for a simple visualization, let's try to lookup if a statement exists on this line.

        // Find which AST node starts on this line
        let node_idx = program.iter().position(|spanned| {
            let start_byte = spanned.span.0;
            get_line_number(&source, start_byte) == line_idx + 1
        });

        let addr_str = if let Some(idx) = node_idx {
            if let Some(addr) = address_map.get(&idx) {
                format!("{:04X}", addr)
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let code_str = if let Some(idx) = node_idx {
            if let Some(code) = machine_code_map.get(&idx) {
                code.clone()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        println!(
            "{:<6} | \x1b[34m{:<8}\x1b[0m | \x1b[32m{:<16}\x1b[0m | {}",
            line_idx + 1,
            addr_str,
            code_str,
            source_line
        );
    }

    // ==========================================
    // OUTPUT: SYMBOL TABLE
    // ==========================================
    println!("\n=== SYMBOL TABLE ===");
    println!(
        "{:<20} | {:<10} | {:<10} | {:<10}",
        "Name", "Type", "DataType", "Offset"
    );
    println!("{}", "-".repeat(60));

    // Sort symbols for cleaner output
    let mut sorted_symbols: Vec<_> = symbol_table.iter().collect();
    sorted_symbols.sort_by_key(|(name, _)| *name);

    for (name, info) in sorted_symbols {
        let offset_str = if let Some(off) = info.offset {
            format!("{:04X}h", off)
        } else {
            "----".to_string()
        };

        println!(
            "{:<20} | {:<10?} | {:<10?} | \x1b[33m{:<10}\x1b[0m",
            name, info.type_, info.data_type, offset_str
        );
    }
}

// Helper to convert byte offset to line number
fn get_line_number(source: &str, offset: usize) -> usize {
    let slice = &source[..offset.min(source.len())];
    let count = slice.lines().count();
    if slice.ends_with('\n') {
        count + 1
    } else {
        count
    }
    .max(1)
}
