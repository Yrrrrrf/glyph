use chumsky::prelude::*;
use std::str::FromStr;

// --- Self-Contained Definitions ---
// To avoid creating new files, all necessary types are defined here.

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Mnemonic(String),
    Register(String),
    Immediate(String),
    Label(String),
    // Added to handle other parts of the syntax shown in `+page.svelte`
    Directive(String),
    Punctuation(char),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Register(String),
    Immediate(i64),
    Label(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Mnemonic {
    MOV,
    SYSCALL,
    // Add other mnemonics here as needed
    UNKNOWN,
}

// Implement FromStr to easily convert token strings to the Mnemonic enum.
impl FromStr for Mnemonic {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_uppercase().as_str() {
            "MOV" => Self::MOV,
            "SYSCALL" => Self::SYSCALL,
            _ => Self::UNKNOWN,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Instruction {
    pub mnemonic: Mnemonic,
    pub operand1: Option<Operand>,
    pub operand2: Option<Operand>,
}

// --- Corrected Parser Logic ---

// A type alias for the parser's error type for brevity.
type ParseErr<'a> = extra::Err<Simple<'a, Token>>;

/// Parses a mnemonic token into a `Mnemonic` enum.
fn mnemonic_parser<'a>() -> impl Parser<'a, &'a [Token], Mnemonic, ParseErr<'a>> + Clone {
    // Replaced `filter_map` with the correct `select!` macro.
    select! {
        Token::Mnemonic(s) => Mnemonic::from_str(&s).unwrap_or(Mnemonic::UNKNOWN),
    }
    .labelled("mnemonic")
}

/// Parses an operand token into an `Operand` enum.
fn operand_parser<'a>() -> impl Parser<'a, &'a [Token], Operand, ParseErr<'a>> + Clone {
    // Replaced `filter_map` with the correct `select!` macro.
    select! {
        Token::Register(s) => Operand::Register(s),
        Token::Immediate(s) => {
            let val = if let Some(hex_val) = s.strip_prefix("0x") {
                i64::from_str_radix(hex_val, 16).unwrap_or(0)
            } else if let Some(hex_val) = s.strip_suffix(|c| c == 'h' || c == 'H') {
                // Parse hexadecimal with 'h' suffix like "0Ah"
                i64::from_str_radix(hex_val, 16).unwrap_or(0)
            } else {
                s.parse().unwrap_or(0)
            };
            Operand::Immediate(val)
        },
        Token::Label(s) => Operand::Label(s),
    }
    .labelled("operand")
}

/// Parses a full instruction.
pub fn instruction_parser<'a>() -> impl Parser<'a, &'a [Token], Instruction, ParseErr<'a>> + Clone {
    mnemonic_parser()
        .then(operand_parser().repeated().at_most(2).collect::<Vec<_>>())
        .map(|(mnemonic, operands)| {
            let mut ops = operands.into_iter();
            Instruction {
                mnemonic,
                operand1: ops.next(),
                operand2: ops.next(),
            }
        })
        .labelled("instruction")
}
