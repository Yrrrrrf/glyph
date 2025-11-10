use logos::{Lexer, Logos};

// The Token enum, which is the core of a Logos lexer.
// It defines all possible tokens in your assembly language.
#[derive(Logos, Debug, Clone, PartialEq)]
pub enum Token<'a> {
    // Skip whitespace and comments. The regex for comments handles everything
    // from a semicolon to the end of the line.
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r";[^\n]*", logos::skip)]
    Error, // Logos requires a default error variant.

    // Keywords and Directives are matched first.
    #[token("section")]
    #[token("global")]
    #[token("db")]
    Directive(&'a str),

    #[token("mov")]
    #[token("syscall")]
    Mnemonic(&'a str),

    #[token("rax")]
    #[token("rdi")]
    #[token("rsi")]
    #[token("rdx")]
    Register(&'a str),

    // A string literal, e.g., 'Hello, World!'.
    // The callback function strips the surrounding quotes.
    #[regex(r#"'[^']*'"#, |lex| &lex.slice()[1..lex.slice().len() - 1])]
    String(&'a str),

    // An integer literal.
    #[regex("[0-9]+")]
    Integer(&'a str),
    
    // A label, which can end with a colon. The callback strips the colon.
    // This has a lower priority than keywords, so `section` won't be matched as a label.
    #[regex(r"[_a-zA-Z][a-zA-Z0-9_]*:", |lex| &lex.slice()[..lex.slice().len() - 1])]
    LabelDeclaration(&'a str),

    // General identifiers, which can be section names (like .data) or label uses (like msg).
    #[regex(r"[\.a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier(&'a str),
    
    // Punctuation.
    #[token(",")]
    Comma,
}

// This is the public function that creates the lexer.
pub fn lexer<'a>(source: &'a str) -> Lexer<'a, Token<'a>> {
    Token::lexer(source)
}