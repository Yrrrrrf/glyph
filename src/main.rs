// src/main.rs
#![allow(dead_code, unused)]

mod ast;
mod semantics;
mod syntax;

use chumsky::prelude::*;
use std::env;
use std::fs;

use semantics::validator::{Flavor, validate};
use syntax::tokens::{Token, constant};
use syntax::{lexer::lexer, parser::parser};

// ... (get_filename and read_file match your previous version) ...

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

// FIX: Correct line number calculation
fn get_line_number(source: &str, offset: usize) -> usize {
    let slice = &source[..offset];
    let count = slice.lines().count();
    // If the previous slice ends strictly with \n, .lines() doesn't count the new empty line
    // we are currently on.
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
        "{:<10} | {:<22} | {:<30} | {}",
        "Line", "Category", "Value", "Detail"
    );
    println!("{}", "-".repeat(100));

    for (token, span) in tokens {
        let line = get_line_number(source, span.start);

        let (category, value_str) = match token {
            Token::Instruction(s) => ("Instruction", s.clone()),
            Token::Pseudoinstruction(s) => ("Pseudoinstruction", s.clone()),
            Token::Register(s) => ("Register", s.clone()),
            Token::Symbol(s) => ("Symbol", s.clone()),
            Token::Constant(c) => match c {
                constant::Type::String(s) => ("Constant (String)", format!("\"{}\"", s)),
                constant::Type::NumberDecimal(v) => ("Constant (Dec)", v.to_string()),
                constant::Type::NumberHex(_, raw) => ("Constant (Hex)", raw.clone()),
                constant::Type::NumberBinary(_, raw) => ("Constant (Bin)", raw.clone()),
                constant::Type::Char(c) => ("Constant (Char)", format!("'{}'", c)),
            },
            Token::Error(s) => ("Elemento inválido", s.clone()),
            _ => ("Separator", token.to_string()),
        };

        if category == "Elemento inválido" {
            println!(
                "{:<10} | \x1b[31m{:<22}\x1b[0m | {:<30} | {:?}",
                line, category, value_str, token
            );
        } else {
            println!(
                "{:<10} | {:<22} | {:<30} | {:?}",
                line, category, value_str, token
            );
        }
    }
    println!();
}

fn main() {
    let filename = get_filename();
    let code = read_file(&filename);
    let len = code.len();

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

    if tokens.is_none() {
        println!("❌ Fatal Lexer Error: No tokens produced.");
        return;
    }
    let tokens = tokens.unwrap();

    print_lexer_output(&code, &tokens);

    // ... (Parser logic remains the same) ...
    // (If you are not using parser output yet for phase 1, you can leave it or comment it out)
}
