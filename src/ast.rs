#![allow(unused)]
#![allow(dead_code)]

// src/ast.rs
// use crate::syntax::tokens::Token;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Operand {
    Register(String),
    Immediate(u64, String),
    Memory { base: String, offset: Option<i64> },
    Label(String),
    StringLiteral(String), // For DB "hello"
}

#[derive(Debug, Clone, Serialize)]
pub enum Statement {
    // CODE
    Instruction {
        mnemonic: String,
        operands: Vec<Operand>,
    },
    Label(String),

    // DATA / STRUCTURE
    // e.g. ".stack segment" or "data segment"
    Segment {
        name: String,
    },
    SegmentEnd, // "ends"

    // e.g. "var1 db 10"
    Variable {
        name: String,
        directive: String,
        value: Operand,
    },
    // e.g. "dw 128" (no name)
    Data {
        directive: String,
        value: Operand,
    },

    // Unhandled but parsed directive
    Directive {
        name: String,
        args: Vec<Operand>,
    },
}

pub type Program = Vec<Statement>;
