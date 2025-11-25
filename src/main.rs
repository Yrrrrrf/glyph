// src/main.rs
#![allow(dead_code, unused)]

mod ast;
mod semantics;
mod syntax;

use chumsky::prelude::*;
use std::env;
use std::fs;

use syntax::{lexer::lexer, tokens::Token};

fn get_filename() -> String {
    let args: Vec<String> = env::args().collect();
    args.get(1)
        .map(|s| s.to_string())
        .unwrap_or_else(|| "test.asm".to_string())
}

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Error reading '{}': {}", filename, e);
        std::process::exit(1);
    })
}

fn get_line_number(source: &str, offset: usize) -> usize {
    let slice = &source[..offset];
    let count = slice.lines().count();
    if slice.ends_with('\n') {
        count + 1
    } else {
        count
    }
    .max(1)
}

fn print_lexer_output(source: &str, tokens: &[(Token, SimpleSpan)]) {
    println!("=== LEXER OUTPUT ===");
    println!(
        "{:<4} | {:<48} | {:<24}",
        "Line", "Category", "Value (Source Slice)"
    );
    println!("{}", "-".repeat(90));

    for (token, span) in tokens {
        let line = get_line_number(source, span.start);

        // 1. Get exact text from source using span
        let value_str = &source[span.start..span.end];

        // 2. Get Metadata from Token methods
        let category = token.category();
        let description = token.description();

        // 3. Print
        if let Token::Error(_) = token {
            println!("{line:<4} | \x1b[31m{description:<48}\x1b[0m | {value_str:<24}");
        } else {
            println!("{line:<4} | {category:<16}{description:<32} | {value_str:<24}",);
        }
    }
    println!();
}

fn main() {
    let filename = get_filename();
    let code = read_file(&filename);

    println!("=== Processing: {} ===\n", filename);

    let (tokens, lex_errs) = lexer().parse(&code).into_output_errors();

    if !lex_errs.is_empty() {
        println!("⚠️ LEXER ENCOUNTERED ERRORS:");
        for err in lex_errs {
            let span = err.span();
            let line = get_line_number(&code, span.start);
            let found_text = err
                .found()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "end of file".to_string());

            println!(
                "  [Line {:>3}] Error: Found '{}', expected one of: {:?}",
                line,
                found_text,
                err.expected().collect::<Vec<_>>()
            );
        }
        println!();
    }

    if let Some(tokens) = tokens {
        print_lexer_output(&code, &tokens);
    } else {
        println!("❌ Fatal Lexer Error: No tokens produced.");
    }
}
