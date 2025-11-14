// #![allow(dead_code)]
// #![allow(unused)]

mod lex;
mod parse;
mod tokens;

use chumsky::Parser;

fn main() {
    let code = r#"
    ; Sample assembly
    section .data
        msg db "Hello", 0
    .code
    start:
        MOV AX, 10h
        ADD AX, 1
        INT 21h
    "#;

    println!("=== NEW LEXER OUTPUT ===");
    let tokens: Vec<_> = lex::lexer(code).collect();

    for token in &tokens {
        println!("{:>12} | {:?}", token.category(), token);
    }

    println!("\n=== CONVERTED FOR PARSER ===");
    let parse_tokens: Vec<parse::Token> = tokens.iter().map(|t| t.to_parse_token()).collect();

    // println!("{:#?}", parse_tokens);

    // Parse as before
    let result = parse::instruction_parser().parse(&parse_tokens);
    println!("\n=== PARSE RESULT ===\n{:#?}", result);
}
