use logos::{Lexer, Logos};

// Subcategorías para las instrucciones
#[derive(Debug, Clone, PartialEq)]
pub enum InstructionType {
    DataTransfer,    // MOV, PUSH, POP, XCHG
    Arithmetic,      // ADD, SUB, MUL, DIV, INC, DEC
    Logic,           // AND, OR, XOR, NOT
    ControlFlags,    // CLC, STC, CMC, CLD, STD
    ControlTransfer, // JMP, CALL, RET
    ConditionalJump, // JE, JNE, JZ, JG, JL
    Interrupt,       // INT
}

// Subcategorías para las constantes
#[derive(Debug, Clone, PartialEq)]
pub enum ConstantVariant {
    Decimal,
    Hexadecimal,
    Binary,
    String,
}

// The Token enum - SOLO 6 CATEGORÍAS PRINCIPALES
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]  // Skip whitespace
#[logos(skip r";[^\n]*")]     // Skip comments
pub enum Token<'a> {
    // ==================== INSTRUCCIÓN ====================
    #[token("MOV")]
    #[token("PUSH")]
    #[token("POP")]
    #[token("XCHG")]
    #[token("ADD")]
    #[token("SUB")]
    #[token("MUL")]
    #[token("DIV")]
    #[token("INC")]
    #[token("DEC")]
    #[token("AND")]
    #[token("OR")]
    #[token("XOR")]
    #[token("NOT")]
    #[token("CLC")]
    #[token("STC")]
    #[token("CLD")]
    #[token("STD")]
    #[token("JMP")]
    #[token("CALL")]
    #[token("RET")]
    #[token("JE")]
    #[token("JNE")]
    #[token("JZ")]
    #[token("JG")]
    #[token("JL")]
    #[token("JGE")]
    #[token("JLE")]
    #[token("INT")]
    Instruction(&'a str),

    // ==================== PSEUDOINSTRUCCIÓN ====================
    #[token("SEGMENT")]
    #[token("ENDS")]
    #[token("DB")]
    #[token("DW")]
    #[token("DD")]
    #[token("DUP")]
    #[token("EQU")]
    #[token("END")]
    #[token("PTR")]
    #[token("BYTE")]
    #[token("WORD")]
    #[token("DWORD")]
    Pseudoinstruction(&'a str),

    // ==================== REGISTRO ====================
    #[token("AX")]
    #[token("BX")]
    #[token("CX")]
    #[token("DX")]
    #[token("AL")]
    #[token("AH")]
    #[token("BL")]
    #[token("BH")]
    #[token("SI")]
    #[token("DI")]
    #[token("BP")]
    #[token("SP")]
    Register(&'a str),

    // ==================== SÍMBOLO ====================
    // Label declaration: etiqueta:
    #[regex(r"[_a-zA-Z][a-zA-Z0-9_]*:", |lex| &lex.slice()[..lex.slice().len() - 1])]
    // General identifier (label use, variable name, etc.)
    #[regex(r"[\.a-zA-Z_][a-zA-Z0-9_]*")]
    Symbol(&'a str),

    // ==================== CONSTANTE ====================
    // String: "Hello, World!"
    #[regex(r#""[^"]*""#, |lex| &lex.slice()[1..lex.slice().len() - 1])]
    // todo: Fix the conflic with symbols that end with 'h'
    // todo: Somehow handle the 0x prefix
    // Hexadecimal: 0Fh, 1Ah, FFh
    // #[regex(r"[0-9A-Fa-f]+[Hh]")]
    // Binary: 1010b, 11110000b
    #[regex(r"[01]+[Bb]")]
    // Decimal: 123, 456
    #[regex(r"[0-9]+")]
    Constant(&'a str),

    // ==================== PUNCTUATION ====================
    #[token(",")]
    Comma,

    #[token("[")]
    LeftBracket,

    #[token("]")]
    RightBracket,

    // ==================== INVALID ====================
    Invalid,
}

impl<'a> Token<'a> {
    // Helper method para obtener el tipo de instrucción
    pub fn instruction_type(&self) -> Option<InstructionType> {
        if let Token::Instruction(s) = self {
            match s.to_uppercase().as_str() {
                "MOV" | "PUSH" | "POP" | "XCHG" => Some(InstructionType::DataTransfer),
                "ADD" | "SUB" | "MUL" | "DIV" | "INC" | "DEC" => Some(InstructionType::Arithmetic),
                "AND" | "OR" | "XOR" | "NOT" => Some(InstructionType::Logic),
                "CLC" | "STC" | "CLD" | "STD" => Some(InstructionType::ControlFlags),
                "JMP" | "CALL" | "RET" => Some(InstructionType::ControlTransfer),
                "JE" | "JNE" | "JZ" | "JG" | "JL" | "JGE" | "JLE" => Some(InstructionType::ConditionalJump),
                "INT" => Some(InstructionType::Interrupt),
                _ => None,
            }
        } else {
            None
        }
    }

    // Helper method para obtener el tipo de constante
    pub fn constant_variant(&self) -> Option<ConstantVariant> {
        if let Token::Constant(s) = self {
            if s.starts_with('"') || s.ends_with('"') {
                Some(ConstantVariant::String)
            } else if s.to_uppercase().ends_with('H') {
                Some(ConstantVariant::Hexadecimal)
            } else if s.to_uppercase().ends_with('B') {
                Some(ConstantVariant::Binary)
            } else {
                Some(ConstantVariant::Decimal)
            }
        } else {
            None
        }
    }
}

// Public function to create the lexer
pub fn lexer<'a>(source: &'a str) -> Lexer<'a, Token<'a>> {
    Token::lexer(source)
}