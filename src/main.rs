// Make the lex and parse modules available to the main binary.
mod lex;
mod parse;

// Import necessary items from the chumsky parser library.
use chumsky::Parser;

// --- Main execution block ---
fn main() {
    // Sample assembly code to test the lexer and parser.
    let code = "mov rax, 1";

    println!("--- Testing Lexer ---");
    println!("Input code:\n{}\n", code);

    // 1. Lex the source code into a stream of `lex::Token`s.
    let lex_tokens: Vec<_> = lex::lexer(code).filter_map(Result::ok).collect();
    println!("Lexer output:\n{:#?}\n", lex_tokens);

    // 2. Convert `lex::Token`s to `parse::Token`s for the parser.
    let parse_tokens = convert_tokens(&lex_tokens);
    println!("--- Testing Parser ---");
    println!("Tokens converted for parser:\n{:#?}\n", parse_tokens);

    // 3. Use the instruction_parser to parse the token stream.
    // The parser expects a slice `&[Token]`.
    let parse_result = parse::instruction_parser().parse(&parse_tokens).unwrap();

    // 4. Print the final parsed Abstract Syntax Tree (AST).
    println!("Parser output (AST):\n{:#?}", parse_result);

    // --- Verification ---
    // We can assert the output to confirm it's what we expect.
    assert_eq!(
        parse_result,
        parse::Instruction {
            mnemonic: parse::Mnemonic::MOV,
            operand1: Some(parse::Operand::Register("rax".to_string())),
            operand2: Some(parse::Operand::Immediate(1)),
        }
    );
    println!("\n✅ Successfully parsed the instruction!");
}

/// Converts tokens from the `logos` lexer (`lex::Token`) into the format
/// expected by the `chumsky` parser (`parse::Token`).
fn convert_tokens<'a>(lex_tokens: &'a [lex::Token<'a>]) -> Vec<parse::Token<'a>> {
    lex_tokens
        .iter()
        .map(|token| match token {
            lex::Token::Mnemonic(s) => parse::Token::Mnemonic(s),
            lex::Token::Register(s) => parse::Token::Register(s),
            lex::Token::Integer(s) => parse::Token::Immediate(s),
            lex::Token::Identifier(s) => parse::Token::Label(s),
            lex::Token::LabelDeclaration(s) => parse::Token::Label(s),
            lex::Token::Directive(s) => parse::Token::Directive(s),
            lex::Token::String(s) => parse::Token::String(s),
            lex::Token::Comma => parse::Token::Punctuation(','),
            // The `Error` token is ignored for this test.
            lex::Token::Error => todo!(), // Or handle it gracefully
        })
        .collect()
}