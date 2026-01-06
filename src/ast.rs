// src/ast.rs
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Operand {
    Register(String),
    Immediate(u64, String),
    Memory { base: String, offset: Option<i64> },
    Label(String),
    StringLiteral(String),
    // NEW VARIANTS
    Dup { count: u64, value: Box<Operand> },
    Uninitialized,
}

#[derive(Debug, Clone, Serialize)]
pub enum Statement {
    Instruction {
        mnemonic: String,
        operands: Vec<Operand>,
    },
    Label(String),
    Segment {
        name: String,
    },
    End {
        label: Option<String>,
    },
    SegmentEnd,
    Variable {
        name: String,
        directive: String, // "DB" or "DW"
        value: Operand,
    },
    // NEW VARIANT
    Constant {
        name: String,
        value: Operand,
    },
    Data {
        directive: String,
        value: Operand,
    },
    Directive {
        name: String,
        args: Vec<Operand>,
    },
    Unknown,
}

#[derive(Debug, Clone, Serialize)]
pub struct Spanned<T> {
    pub node: T,
    pub span: (usize, usize),
}

#[derive(Debug, Clone, Serialize)]
pub enum LineNode {
    Statement(Statement),
    Empty,
    Error(String),
}

pub type Program = Vec<Spanned<LineNode>>;
