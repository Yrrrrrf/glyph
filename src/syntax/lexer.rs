// src/syntax/lexer.rs
use crate::syntax::tokens::pseudoinstruction;
use crate::syntax::tokens::register;
use crate::syntax::tokens::{PunctuationType, Token, classify_instruction, constant};
use chumsky::prelude::*;

type LexerError<'src> = extra::Err<Rich<'src, char>>;

fn validate_constants<'src>() -> impl Parser<'src, &'src str, Token, LexerError<'src>> {
    // 1.1 Hex Validator
    let validate_hex = || {
        text::digits(16)
            .to_slice()
            .then_ignore(just('h').or(just('H')))
            .try_map(|s: &str, span| {
                if s.chars().next().map_or(false, |c| c.is_ascii_alphabetic()) {
                    Err(Rich::custom(
                        span,
                        "Hex literal must start with a decimal digit (0-9)",
                    ))
                } else {
                    Ok(s)
                }
            })
            .map(|s: &str| {
                let chars: Vec<char> = s.chars().collect();
                if chars.len() > 1 && chars[0] == '0' && chars[1].is_ascii_digit() {
                    Token::Constant(constant::Type::NumberHex(
                        u64::from_str_radix(s, 16).unwrap_or(0),
                        format!("{}h", s),
                    ))
                } else {
                    Token::Error(format!("Necessary leading zero"))
                }
            })
    };

    // 1.2 Binary
    let validate_bin = || {
        text::digits(2)
            .to_slice()
            .then_ignore(just('b').or(just('B')))
            .map(|s: &str| {
                if s.len() != 8 && s.len() != 16 {
                    Token::Error(format!("Invalid length: {}, expected 8 or 16", s.len()))
                } else {
                    Token::Constant(constant::Type::NumberBinary(
                        u64::from_str_radix(s, 2).unwrap_or(0),
                        format!("{}b", s),
                    ))
                }
            })
    };

    // 1.3 Decimal
    let validate_dec = || {
        text::int(10)
            .map(|s: &str| Token::Constant(constant::Type::NumberDecimal(s.parse().unwrap_or(0))))
    };

    // 1.4 String
    let validate_string = || {
        let content = none_of("\"\r\n").repeated().collect::<String>();
        just('"')
            .ignore_then(content)
            .then(just('"').map(|_| true).or(empty().map(|_| false)))
            .map(|(s, closed)| {
                if closed {
                    Token::Constant(constant::Type::String(s))
                } else {
                    Token::Error(format!("String missing closing quote"))
                }
            })
    };

    // 1.5 Char
    let validate_char = || {
        let content = none_of("\'\r\n").repeated().collect::<String>();
        just('\'')
            .ignore_then(content)
            .then(just('\'').map(|_| true).or(empty().map(|_| false)))
            .map(|(s, closed)| {
                if closed {
                    Token::Constant(constant::Type::String(s))
                } else {
                    Token::Error(format!("Char literal missing closing quote"))
                }
            })
    };

    choice((
        validate_hex(),
        validate_bin(),
        validate_dec(),
        validate_string(),
        validate_char(),
    ))
}

fn validate_compounds<'src>() -> impl Parser<'src, &'src str, Token, LexerError<'src>> {
    let case_ignore = |kwd: &'static str| {
        just('.')
            .or_not()
            .then(text::ascii::ident())
            .to_slice()
            .try_map(move |s: &str, span| {
                if s.eq_ignore_ascii_case(kwd) {
                    Ok(s)
                } else {
                    Err(Rich::custom(span, "Case mismatch"))
                }
            })
    };

    let mk_compound = move |first: &'static str, second: &'static str| {
        case_ignore(first)
            .then(text::whitespace())
            .then(case_ignore(second))
            .to(Token::Pseudoinstruction(format!("{} {}", first, second)))
    };

    let arrays = just('[')
        .then(none_of("]\r\n").repeated().collect::<String>())
        .then(just(']'))
        .map(|((_, content), _)| Token::Symbol(format!("[{}]", content)));

    let dups = text::ascii::ident()
        .try_map(|s: &str, span| {
            if s.eq_ignore_ascii_case("dup") {
                Ok(s)
            } else {
                Err(Rich::custom(span, "not dup"))
            }
        })
        .then(text::whitespace().or_not())
        .then(just('('))
        .then(none_of(")\r\n").repeated().collect::<String>())
        .then(just(')'))
        .map(|((((_, _), _), content), _)| Token::Pseudoinstruction(format!("dup({})", content)));

    choice((
        mk_compound(".STACK", "SEGMENT"),
        mk_compound(".DATA", "SEGMENT"),
        mk_compound(".CODE", "SEGMENT"),
        mk_compound("BYTE", "PTR"),
        mk_compound("WORD", "PTR"),
        mk_compound("DWORD", "PTR"),
        arrays,
        dups,
    ))
}

// --- UPDATED IDENTIFIER PARSER ---
fn validate_identifiers<'src>() -> impl Parser<'src, &'src str, Token, LexerError<'src>> {
    text::ascii::ident()
        .or(just('.').then(text::ascii::ident()).to_slice())
        .map(|s: &str| {
            let upper = s.to_uppercase();

            // 1. Check if it's an Assigned Instruction
            if let Some(instr_type) = classify_instruction(&upper) {
                return Token::Instruction(instr_type, upper);
            }

            // 2. Check Register
            if register::is_valid(&upper) {
                return Token::Register(upper);
            }

            // 3. Check Pseudo
            if pseudoinstruction::is_reserved(&upper) {
                return Token::Pseudoinstruction(upper);
            }

            // 4. Default to Symbol
            Token::Symbol(s.to_string())
        })
}

// --- UPDATED PUNCTUATION PARSER ---
fn validate_punctuation<'src>() -> impl Parser<'src, &'src str, Token, LexerError<'src>> {
    choice((
        just(',').to(Token::Punctuation(PunctuationType::Comma)),
        just(':').to(Token::Punctuation(PunctuationType::Colon)),
        just('[').to(Token::Punctuation(PunctuationType::LBracket)),
        just(']').to(Token::Punctuation(PunctuationType::RBracket)),
        just('(').to(Token::Punctuation(PunctuationType::LParen)),
        just(')').to(Token::Punctuation(PunctuationType::RParen)),
        just('+').to(Token::Punctuation(PunctuationType::Plus)),
        just('-').to(Token::Punctuation(PunctuationType::Minus)),
        just('.').to(Token::Punctuation(PunctuationType::Dot)),
    ))
}

pub fn lexer<'src>() -> impl Parser<'src, &'src str, Vec<(Token, SimpleSpan)>, LexerError<'src>> {
    let token_type = choice((
        validate_compounds(),
        validate_constants(),
        validate_identifiers(), // This now handles Instruction Logic internally
        validate_punctuation(),
        text::newline().to(Token::Newline),
    ));

    // ... (rest of function: comments, whitespace, etc. remains same) ...
    let comment = just(';').then(any().and_is(text::newline().not()).repeated());

    // Whitespace excluding newline
    let whitespace = one_of(" \t").repeated().at_least(1);

    let ignored = choice((whitespace.ignored(), comment.ignored())).repeated();

    ignored
        .ignore_then(
            token_type
                .map_with(|t, e| (t, e.span()))
                .recover_with(via_parser(
                    any().map_with(|c: char, e| (Token::Error(c.to_string()), e.span())),
                )),
        )
        .repeated()
        .collect()
}
