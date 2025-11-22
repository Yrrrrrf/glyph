// src/syntax/lexer.rs
use crate::syntax::tokens::{Token, constant, instruction, pseudoinstruction, register};
use chumsky::prelude::*;

type LexerError<'src> = extra::Err<Rich<'src, char>>;

/// 1. VALIDATE CONSTANTS
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
                    Token::Error(format!("{}h (Invalid: Unnecessary leading zero)", s))
                } else {
                    Token::Constant(constant::Type::NumberHex(
                        u64::from_str_radix(s, 16).unwrap_or(0),
                        format!("{}h", s),
                    ))
                }
            })
    };

    // 1.2 Binary Validator
    let validate_bin = || {
        text::digits(2)
            .to_slice()
            .then_ignore(just('b').or(just('B')))
            .map(|s: &str| {
                if s.len() != 8 && s.len() != 16 {
                    Token::Error(format!(
                        "{}b (Invalid length: {}, expected 8 or 16)",
                        s,
                        s.len()
                    ))
                } else {
                    Token::Constant(constant::Type::NumberBinary(
                        u64::from_str_radix(s, 2).unwrap_or(0),
                        format!("{}b", s),
                    ))
                }
            })
    };

    // 1.3 Decimal Validator
    let validate_dec = || {
        text::int(10)
            .map(|s: &str| Token::Constant(constant::Type::NumberDecimal(s.parse().unwrap_or(0))))
    };

    // 1.4 String Validator
    let validate_string = || {
        let content = none_of("\"\r\n").repeated().collect::<String>();
        just('"')
            .ignore_then(content)
            .then(just('"').map(|_| true).or(empty().map(|_| false)))
            .map(|(s, closed)| {
                if closed {
                    Token::Constant(constant::Type::String(s))
                } else {
                    Token::Error(format!("\"{} (String missing closing quote)", s))
                }
            })
    };

    // 1.5 Char Validator
    let validate_char = || {
        let content = none_of("\'\r\n").repeated().collect::<String>();
        just('\'')
            .ignore_then(content)
            .then(just('\'').map(|_| true).or(empty().map(|_| false)))
            .map(|(s, closed)| {
                if closed {
                    Token::Constant(constant::Type::String(s))
                } else {
                    Token::Error(format!("'{} (Char literal missing closing quote)", s))
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

/// 2. VALIDATE COMPOUNDS (Case-Insensitive)
/// 2. VALIDATE COMPOUNDS (Case-Insensitive)
fn validate_compounds<'src>() -> impl Parser<'src, &'src str, Token, LexerError<'src>> {
    // Helper to match a specific word case-insensitively (e.g. .STACK or BYTE)
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

    // 1. Segments & Pointers (.STACK SEGMENT, BYTE PTR, etc.)
    let mk_compound = move |first: &'static str, second: &'static str| {
        case_ignore(first)
            .then(text::whitespace())
            .then(case_ignore(second))
            .to(Token::Pseudoinstruction(format!("{} {}", first, second)))
    };

    // 2. Arrays: [xxx]
    // Captures everything inside brackets as one single Symbol token
    let arrays = just('[')
        .then(none_of("]\r\n").repeated().collect::<String>())
        .then(just(']'))
        .map(|((_, content), _)| Token::Symbol(format!("[{}]", content)));

    // 3. DUP: dup(xxx)
    // Captures dup(...) as one single Pseudoinstruction token
    let dups = text::ascii::ident()
        .try_map(|s: &str, span| {
            if s.eq_ignore_ascii_case("dup") {
                Ok(s)
            } else {
                Err(Rich::custom(span, "not dup"))
            }
        })
        .then(text::whitespace().or_not()) // matches "dup(" or "dup ("
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
        arrays, // <--- New
        dups,   // <--- New
    ))
}

/// 3. VALIDATE IDENTIFIERS
fn validate_identifiers<'src>() -> impl Parser<'src, &'src str, Token, LexerError<'src>> {
    text::ascii::ident()
        .or(just('.').then(text::ascii::ident()).to_slice())
        .map(|s: &str| {
            let upper = s.to_uppercase();

            if instruction::is_assigned(&upper) {
                Token::Instruction(upper)
            } else if register::is_valid(&upper) {
                Token::Register(upper)
            } else if pseudoinstruction::is_reserved(&upper) {
                Token::Pseudoinstruction(upper)
            } else {
                Token::Symbol(s.to_string())
            }
        })
}

/// 4. VALIDATE PUNCTUATION
fn validate_punctuation<'src>() -> impl Parser<'src, &'src str, Token, LexerError<'src>> {
    choice((
        just(',').to(Token::Comma),
        just(':').to(Token::Colon),
        just('[').to(Token::LBracket),
        just(']').to(Token::RBracket),
        just('(').to(Token::LParen),
        just(')').to(Token::RParen),
        just('+').to(Token::Plus),
        just('-').to(Token::Minus),
        just('.').to(Token::Dot),
    ))
}

/// MAIN LEXER FUNCTION
pub fn lexer<'src>() -> impl Parser<'src, &'src str, Vec<(Token, SimpleSpan)>, LexerError<'src>> {
    let token_type = choice((
        validate_compounds(),
        validate_constants(),
        validate_identifiers(),
        validate_punctuation(),
    ));

    let comment = just(';').then(any().and_is(text::newline().not()).repeated());

    let whitespace = any()
        .filter(|c: &char| c.is_whitespace())
        .repeated()
        .at_least(1);

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
