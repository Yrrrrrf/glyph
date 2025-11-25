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

// --- 2. HELPERS (FIX FOR LEXER) ---
// These were missing in the previous refactor, causing the "unresolved module" error.
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
    StringManipulation,
    ProcessorControl,
    Unknown,
}

impl fmt::Display for InstructionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DataTransfer => write!(f, "Transferencia de Datos"),
            Self::Arithmetic => write!(f, "Aritmética"),
            Self::Logic => write!(f, "Lógica"),
            Self::ControlTransfer => write!(f, "Transferencia de Control"),
            Self::FlagControl => write!(f, "Control de Banderas"),
            Self::ConditionalJump => write!(f, "Salto Condicional"),
            Self::Interrupt => write!(f, "Interrupción"),
            Self::StringManipulation => write!(f, "Manipulación de Cadenas"),
            Self::ProcessorControl => write!(f, "Control del Procesador"),
            Self::Unknown => write!(f, "Instrucción"),
        }
    }
}

pub fn classify_instruction(mnemonic: &str) -> Option<InstructionType> {
    match mnemonic {
        "POP" | "LES" | "LDS" | "MOV" | "XCHG" => Some(InstructionType::DataTransfer),
        "DEC" | "IDIV" | "IMUL" | "ADC" | "CMP" | "AAD" | "ADD" | "SUB" | "INC" => {
            Some(InstructionType::Arithmetic)
        }
        "AAA" => Some(InstructionType::Arithmetic),
        "STC" | "CLC" => Some(InstructionType::FlagControl),
        "JAE" | "JC" | "JGE" | "JNB" | "JNG" | "JNO" | "JZ" | "JNZ" => {
            Some(InstructionType::ConditionalJump)
        }
        "INTO" | "INT" => Some(InstructionType::Interrupt),
        "SCASW" => Some(InstructionType::StringManipulation),
        "HLT" => Some(InstructionType::ProcessorControl),
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
            Self::Comma => write!(f, "Separador"),
            Self::Colon => write!(f, "Definidor"),
            Self::LBracket | Self::RBracket => write!(f, "Memoria"),
            Self::LParen | Self::RParen => write!(f, "Agrupación"),
            Self::Plus | Self::Minus => write!(f, "Operador"),
            Self::Dot => write!(f, "Acceso"),
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
        }
    }

    pub fn description(&self) -> String {
        match self {
            // Token::Instruction(type_, _) => type_.to_string(),
            Token::Instruction(type_, _) => format!("{}", type_),
            // Token::Pseudoinstruction(_) => "Directiva".to_string(),
            Token::Pseudoinstruction(_) => "Directiva".to_string(),
            Token::Register(_) => "Registro CPU".to_string(),
            Token::Punctuation(p) => format!("{}", p),
            Token::Symbol(_) => "Identificador".to_string(),

            Token::Constant(c) => match c {
                constant::Type::String(_) => "String".to_string(),
                constant::Type::NumberDecimal(_) => "Decimal".to_string(),
                constant::Type::NumberHex(_, _) => "Hexadecimal".to_string(),
                constant::Type::NumberBinary(_, _) => "Binario".to_string(),
                constant::Type::Char(_) => "Char".to_string(),
            },
            Token::Error(e) => format!("{}", e),
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
        }
    }
}
