//! Assembly token definitions using logic_tracer-style macros
#![allow(dead_code)]
#![allow(unused)]

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
    Invalid(String), // NEW: For unrecognizable tokens
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
            Self::Invalid(_) => "invalid",
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
            Self::Invalid(s) => Token::Label(format!("__INVALID_{}", s)),
        }
    }

    pub fn detailed_type(&self) -> String {
        match self {
            Self::Instruction(i) => format!("instruction-{}", i.to_string().to_lowercase()),
            Self::Pseudoinstruction(p) => format!("directive-{}", p.to_string().to_lowercase()),
            Self::Register(r) => format!("register-{}", r.to_string().to_lowercase()),
            Self::Symbol(_) => "symbol".to_string(),
            Self::Constant(c) => {
                let variant = match c.variant() {
                    ConstantVariant::Decimal => "decimal",
                    ConstantVariant::Hexadecimal => "hexadecimal",
                    ConstantVariant::Binary => "binary",
                    ConstantVariant::String => "string",
                };
                format!("constant-{}", variant)
            }
            Self::Punctuation(p) => format!("punctuation-{}", p.to_string()),
            Self::Invalid(_) => "invalid".to_string(),
        }
    }
}

impl std::fmt::Display for AssemblyToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            AssemblyToken::Instruction(i) => i.to_string(),
            AssemblyToken::Pseudoinstruction(p) => p.to_string(),
            AssemblyToken::Register(r) => r.to_string(),
            AssemblyToken::Symbol(s) => s.to_string(),
            AssemblyToken::Constant(c) => c.to_string(),
            AssemblyToken::Punctuation(p) => p.to_string(),
            AssemblyToken::Invalid(s) => s.clone(),
        };
        write!(f, "{}", s)
    }
}

/// Helper macro to define categorized token enums
/// Usage: define_categorized_tokens!(Instruction, InstructionType {
///     DataTransfer => [MOV, PUSH, POP, XCHG],
///     Arithmetic => [ADD, SUB, MUL, DIV],
/// });
#[macro_export]
macro_rules! define_categorized_tokens {
    (
        $enum_name:ident, $type_enum:ident {
            $(
                $category:ident => [$($variant:ident),+ $(,)?]
            ),+ $(,)?
        }
    ) => {
        #[derive(Debug, Clone, PartialEq, Serialize)]
        pub enum $type_enum {
            $($category),+
        }

        #[derive(Debug, Clone, PartialEq, Serialize)]
        pub enum $enum_name {
            $(
                $($variant),+
            ),+
        }

        impl crate::tokens::Token for $enum_name {
            fn from_str(s: &str) -> Option<Self> {
                let upper = s.to_uppercase();
                match upper.as_str() {
                    $(
                        $(stringify!($variant) => Some(Self::$variant)),+
                    ),+,
                    _ => None
                }
            }

            fn to_string(&self) -> String {
                match self {
                    $(
                        $(Self::$variant => String::from(stringify!($variant))),+
                    ),+
                }
            }
        }

        impl $enum_name {
            pub fn instruction_type(&self) -> $type_enum {
                match self {
                    $(
                        $(Self::$variant => $type_enum::$category),+
                    ),+
                }
            }
        }
    };
}
