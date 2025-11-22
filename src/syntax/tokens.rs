// src/syntax/tokens.rs
use serde::Serialize;
use std::fmt;

// --- MODULES DEFINITION ---

pub mod constant {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
    pub enum Type {
        String(String),
        NumberDecimal(u64),
        NumberHex(u64, String), // Value, Raw string (e.g. "0AFh")
        NumberBinary(u64, String),
        Char(char),
    }
}

pub mod instruction {
    // The set of instructions assigned to your team (EQ9)
    pub fn is_assigned(mnemonic: &str) -> bool {
        matches!(
            mnemonic,
            "AAA"
                | "AAD"
                | "HLT"
                | "INTO"
                | "SCASW"
                | "STC"
                | "DEC"
                | "IDIV"
                | "IMUL"
                | "POP"
                | "ADC"
                | "CMP"
                | "LES"
                | "LDS"
                | "JAE"
                | "JC"
                | "JGE"
                | "JNB"
                | "JNG"
                | "JNO"
        )
    }
}

pub mod pseudoinstruction {
    pub fn is_reserved(s: &str) -> bool {
        matches!(
            s,
            "DB" | "DW"
                | "DD"
                | "EQU"
                | "ORG"
                | "OFFSET"
                | "ENDS"
                | "SEGMENT"
                | ".CODE"
                | ".DATA"
                | ".STACK"
                | ".MODEL"
        )
    }
}

pub mod register {
    pub fn is_valid(s: &str) -> bool {
        matches!(
            s,
            "AX" | "BX"
                | "CX"
                | "DX"
                | "AL"
                | "AH"
                | "BL"
                | "BH"
                | "CL"
                | "CH"
                | "DL"
                | "DH"
                | "SI"
                | "DI"
                | "SP"
                | "BP"
                | "CS"
                | "DS"
                | "SS"
                | "ES"
        )
    }
}

// --- MAIN TOKEN ENUM ---

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Token {
    // Categories from the rubric
    Instruction(String),       // Only AAA, AAD, etc.
    Pseudoinstruction(String), // .stack segment, db, dw...
    Register(String),          // AX, BX...
    Constant(constant::Type),  // Numbers, Strings
    Symbol(String),            // Labels AND unassigned instructions (MOV, INT...)

    // Punctuation (Separators)
    Comma,
    Colon,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Plus,
    Minus,
    Dot,

    Error(String),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Instruction(s) => write!(f, "{}", s),
            Token::Pseudoinstruction(s) => write!(f, "{}", s),
            Token::Register(s) => write!(f, "{}", s),
            Token::Symbol(s) => write!(f, "{}", s),

            Token::Constant(c) => match c {
                constant::Type::String(s) => write!(f, "\"{}\"", s),
                constant::Type::NumberDecimal(v) => write!(f, "{}", v),
                constant::Type::NumberHex(_, raw) => write!(f, "{}", raw),
                constant::Type::NumberBinary(_, raw) => write!(f, "{}", raw),
                constant::Type::Char(c) => write!(f, "'{}'", c),
            },

            // todo: Remove all of this...
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::LBracket => write!(f, "["),
            Token::RBracket => write!(f, "]"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Dot => write!(f, "."),
            Token::Error(s) => write!(f, "Error({})", s),
        }
    }
}
