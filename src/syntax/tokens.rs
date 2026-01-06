// src/syntax/tokens.rs
use serde::Serialize;
use std::fmt;

// --- 1. CONSTANTS ---
pub mod constant {
    use serde::Serialize;
    #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
    pub enum Type {
        String(String),
        NumberDecimal(u64),
        NumberHex(u64, String),
        NumberBinary(u64, String),
        Char(char),
    }
}

// --- 2. HELPERS ---
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

// --- 3. INSTRUCTIONS ---
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum InstructionType {
    DataTransfer,
    Arithmetic,
    Logic,
    ControlTransfer,
    FlagControl,
    ConditionalJump,
    Interrupt,
    ProcessorControl,
    Unknown,
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataTransfer => write!(f, "Data Transfer"),
            Self::Arithmetic => write!(f, "Arithmetic"),
            Self::Logic => write!(f, "Logic"),
            Self::ControlTransfer => write!(f, "Control Transfer"),
            Self::FlagControl => write!(f, "Flag Control"),
            Self::ConditionalJump => write!(f, "Conditional Jump"),
            Self::Interrupt => write!(f, "Interrupt"),
            Self::ProcessorControl => write!(f, "Processor Control"),
            Self::Unknown => write!(f, "Instruction"),
        }
    }
}

pub fn classify_instruction(mnemonic: &str) -> Option<InstructionType> {
    match mnemonic {
        "POP" | "LES" | "LDS" | "MOV" | "XCHG" => Some(InstructionType::DataTransfer),
        "DEC" | "IDIV" | "IMUL" | "ADC" | "CMP" | "AAD" => Some(InstructionType::Arithmetic),
        "AAA" => Some(InstructionType::Arithmetic),
        "STC" | "CLC" => Some(InstructionType::FlagControl),
        "JAE" | "JC" | "JGE" | "JNB" | "JNG" | "JNO" => Some(InstructionType::ConditionalJump),
        "HLT" | "SCASW" | "INTO" | "INT" => Some(InstructionType::Interrupt),
        _ => None,
    }
}

// --- 4. PUNCTUATION ---
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum PunctuationType {
    Comma,
    Colon,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Plus,
    Minus,
    Dot,
}

impl fmt::Display for PunctuationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Comma => write!(f, "Separator"),
            Self::Colon => write!(f, "Definition"), // or "Label Definition"
            Self::LBracket | Self::RBracket => write!(f, "Memory Access"),
            Self::LParen | Self::RParen => write!(f, "Grouping"),
            Self::Plus | Self::Minus => write!(f, "Operator"),
            Self::Dot => write!(f, "Access"),
        }
    }
}

// --- 5. MAIN TOKEN ENUM ---
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub enum Token {
    Instruction(InstructionType, String),
    Pseudoinstruction(String),
    Register(String),
    Constant(constant::Type),
    Symbol(String),
    Punctuation(PunctuationType),
    Error(String),
    Newline,
}

impl Token {
    pub fn category(&self) -> String {
        match self {
            Token::Instruction(_, _) => "Instruction".to_string(),
            Token::Pseudoinstruction(_) => "Directive".to_string(),
            Token::Register(_) => "Register".to_string(),
            Token::Constant(_) => "Constant".to_string(),
            Token::Symbol(_) => "Symbol".to_string(),
            Token::Punctuation(_) => "Punctuation".to_string(),
            Token::Error(_) => "Error".to_string(),
            Token::Newline => "Control".to_string(),
        }
    }

    pub fn description(&self) -> String {
        match self {
            Token::Instruction(type_, _) => format!("{}", type_),
            Token::Pseudoinstruction(_) => "Directive".to_string(),
            Token::Register(_) => "Register".to_string(),
            Token::Punctuation(p) => format!("{}", p),
            Token::Symbol(_) => "Identifier".to_string(),

            Token::Constant(c) => match c {
                constant::Type::String(_) => "String".to_string(),
                constant::Type::NumberDecimal(_) => "Decimal".to_string(),
                constant::Type::NumberHex(_, _) => "Hexadecimal".to_string(),
                constant::Type::NumberBinary(_, _) => "Binary".to_string(),
                constant::Type::Char(_) => "Char".to_string(),
            },
            Token::Error(e) => format!("{}", e),
            Token::Newline => "Newline".to_string(),
        }
    }
}

// Display for Chumsky Errors
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Instruction(_, s) => write!(f, "{}", s),
            Token::Pseudoinstruction(s) => write!(f, "{}", s),
            Token::Register(s) => write!(f, "{}", s),
            Token::Symbol(s) => write!(f, "{}", s),
            Token::Punctuation(p) => write!(f, "{:?}", p),
            Token::Constant(c) => write!(f, "{:?}", c),
            Token::Error(s) => write!(f, "Error({})", s),
            Token::Newline => write!(f, "\\n"),
        }
    }
}
