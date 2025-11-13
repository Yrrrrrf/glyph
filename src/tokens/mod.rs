//! Assembly token definitions using logic_tracer-style macros

use std::fmt::Debug;

use serde::Serialize;

/// Core trait that all tokens must implement
pub trait Token: Debug {
    fn from_str(s: &str) -> Option<Self>
    where
        Self: Sized;
    fn to_string(&self) -> String;
}

/// Helper macro to define token enums with multiple string representations
/// Handles case-insensitivity automatically for assembly keywords
#[macro_export]
macro_rules! define_tokens {
    ($name:ident { $($variant:ident => [$($str:literal),+]),+ $(,)? }) => {
        #[derive(Debug, Clone, PartialEq, Serialize)]
        pub enum $name {
            $($variant),+
        }

        impl Token for $name {
            fn from_str(s: &str) -> Option<Self> {
                let upper = s.to_uppercase();
                match upper.as_str() {
                    $($($str)|+ => Some(Self::$variant)),+,
                    _ => None
                }
            }

            fn to_string(&self) -> String {
                match self {
                    $(Self::$variant => String::from($($str),+)),+
                }
            }
        }
    };
}

// Re-export token modules
pub mod constant;
pub mod instruction;
pub mod pseudoinstruction;
pub mod punctuation;
pub mod register;
pub mod symbol;

use crate::tokens::{
    constant::{Constant, ConstantVariant},
    instruction::Instruction,
    pseudoinstruction::Pseudoinstruction,
    punctuation::Punctuation,
    register::Register,
    symbol::Symbol,
};

/// All possible token types in assembly
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AssemblyToken {
    Instruction(Instruction),
    Pseudoinstruction(Pseudoinstruction),
    Register(Register),
    Symbol(Symbol),
    Constant(Constant),
    Punctuation(Punctuation),
}

impl AssemblyToken {
    /// Get the token category name for display
    pub fn category(&self) -> &'static str {
        match self {
            Self::Instruction(_) => "instruction",
            Self::Pseudoinstruction(_) => "directive",
            Self::Register(_) => "register",
            Self::Symbol(_) => "symbol",
            Self::Constant(c) => match c.variant() {
                ConstantVariant::Decimal => "decimal",
                ConstantVariant::Hexadecimal => "hexadecimal",
                ConstantVariant::Binary => "binary",
                ConstantVariant::String => "string",
            },
            Self::Punctuation(_) => "punctuation",
        }
    }

    /// Convert to parse::Token for chumsky
    pub fn to_parse_token(&self) -> crate::parse::Token {
        use crate::parse::Token;
        match self {
            Self::Instruction(i) => Token::Mnemonic(i.to_string()),
            Self::Pseudoinstruction(p) => Token::Directive(p.to_string()),
            Self::Register(r) => Token::Register(r.to_string()),
            Self::Symbol(s) => Token::Label(s.to_string()),
            Self::Constant(c) => Token::Immediate(c.to_string()),
            Self::Punctuation(p) => {
                let s = p.to_string();
                Token::Punctuation(s.chars().next().unwrap())
            }
        }
    }
}
