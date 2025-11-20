// src/main.rs
mod lex;
mod parse;
mod tokens;

use chumsky::Parser;
use std::env;
use std::fs;
use tokens::AssemblyToken;

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

fn print_lexer_output(tokens: &[(AssemblyToken, usize)]) {
    println!("=== LEXER OUTPUT ===");
    for (token, line) in tokens {
        // Generate type-category string
        let type_category = match token {
            AssemblyToken::Instruction(i) => {
                format!("instruction-{:?}", i.instruction_type())
            }
            _ => token.category().to_string(),
        };
        
        // Print: Line | type-category | value
        println!("Line {:>3} | {:>20} | {}", line, type_category, token);
    }
}

fn print_parser_output(tokens: &[(AssemblyToken, usize)]) {
    println!("\n=== PARSER OUTPUT ===");
    // Extract just the tokens for the parser, ignoring line numbers
    let parse_tokens: Vec<parse::Token> = tokens.iter().map(|(t, _)| t.to_parse_token()).collect();

    // CHANGED: Use program_parser() instead of instruction_parser()
    let result = parse::program_parser().parse(&parse_tokens);
    
    match result.output() {
        Some(ast) => {
            println!("Successfully parsed {} statements:\n", ast.len());
            for stmt in ast {
                println!("{:?}", stmt);
            }
        },
        None => println!("Parsing failed completely."),
    }

    if result.has_errors() {
        println!("\nErrors encountered:");
        for err in result.errors() {
            println!("{:?}", err);
        }
    }
}


fn main() {
    let filename = get_filename();
    let code = read_file(&filename);

    println!("=== Processing: {} ===\n", filename);

    let tokens: Vec<_> = lex::lexer(&code).collect();

    print_lexer_output(&tokens);
    print_parser_output(&tokens);
}
