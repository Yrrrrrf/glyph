// src/parse.rs
#![allow(dead_code)]

use chumsky::prelude::*;

// --- AST Definitions ---

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Mnemonic(String),
    Directive(String),
    Register(String),
    Immediate(String),
    Label(String),
    Punctuation(char),
    String(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operand {
    Register(String),
    Immediate(i64),
    StringLiteral(String),
    Label(String),
    Memory(Box<Operand>), 
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub mnemonic: String,
    pub operands: Vec<Operand>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Directive {
    pub name: String,
    pub args: Vec<Operand>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Label(String),
    Instruction(Instruction),
    Directive(Directive),
    // New: Represents "var1 DB 10" or "stack SEGMENT"
    DataDefinition { label: String, directive: Directive },
    Empty, 
}

type ParseErr<'a> = extra::Err<Simple<'a, Token>>;

// --- Parser Logic ---

fn base_operand_parser<'a>() -> impl Parser<'a, &'a [Token], Operand, ParseErr<'a>> + Clone {
    select! {
        Token::Register(s) => Operand::Register(s),
        Token::Immediate(s) => {
            let val = if let Some(hex_val) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
                i64::from_str_radix(hex_val, 16).unwrap_or(0)
            } else if let Some(hex_val) = s.strip_suffix(|c| c == 'h' || c == 'H') {
                i64::from_str_radix(hex_val, 16).unwrap_or(0)
            } else if let Some(bin_val) = s.strip_suffix(|c| c == 'b' || c == 'B') {
                i64::from_str_radix(bin_val, 2).unwrap_or(0)
            } else if let Some(dec_val) = s.strip_suffix(|c| c == 'd' || c == 'D') {
                 s[..s.len()-1].parse().unwrap_or(0)
            } else {
                s.parse().unwrap_or(0)
            };
            Operand::Immediate(val)
        },
        Token::Label(s) => Operand::Label(s),
        Token::String(s) => Operand::StringLiteral(s),
    }
    .labelled("operand")
}

fn operand_parser<'a>() -> impl Parser<'a, &'a [Token], Operand, ParseErr<'a>> + Clone {
    recursive(|operand| {
        let memory_access = just(Token::Punctuation('['))
            .ignore_then(operand)
            .then_ignore(just(Token::Punctuation(']')))
            .map(|op| Operand::Memory(Box::new(op)));

        memory_access.or(base_operand_parser())
    })
}

fn operand_list_parser<'a>() -> impl Parser<'a, &'a [Token], Vec<Operand>, ParseErr<'a>> + Clone {
    operand_parser()
        .separated_by(just(Token::Punctuation(',')))
        .collect()
}

fn instruction_parser<'a>() -> impl Parser<'a, &'a [Token], Statement, ParseErr<'a>> + Clone {
    select! { Token::Mnemonic(s) => s }
        .then(operand_list_parser().or_not())
        .map(|(mnemonic, ops)| {
            Statement::Instruction(Instruction {
                mnemonic,
                operands: ops.unwrap_or_default(),
            })
        })
        .labelled("instruction")
}

// Parses just the directive part: "DB 10" or "SEGMENT"
fn raw_directive_parser<'a>() -> impl Parser<'a, &'a [Token], Directive, ParseErr<'a>> + Clone {
    let dot_arg = just(Token::Punctuation('.'))
        .ignore_then(select! { Token::Label(s) => s })
        .map(|s| Operand::Label(format!(".{}", s)));

    let args_parser = operand_parser().or(dot_arg)
        .separated_by(just(Token::Punctuation(',')))
        .collect();

    select! { Token::Directive(s) => s }
        .then(args_parser)
        .map(|(name, args)| Directive { name, args })
}

// Parses standalone directives: "ORG 100h" or ".CODE"
fn standalone_directive_parser<'a>() -> impl Parser<'a, &'a [Token], Statement, ParseErr<'a>> + Clone {
    raw_directive_parser().map(Statement::Directive).labelled("directive")
}

// Parses defined variables/segments: "var1 DB 10" or "stack SEGMENT"
fn data_definition_parser<'a>() -> impl Parser<'a, &'a [Token], Statement, ParseErr<'a>> + Clone {
    select! { Token::Label(s) => s }
        .then(raw_directive_parser())
        .map(|(label, directive)| Statement::DataDefinition { label, directive })
        .labelled("data_def")
}

fn label_parser<'a>() -> impl Parser<'a, &'a [Token], Statement, ParseErr<'a>> + Clone {
    let normal_label = select! { Token::Label(s) => s };
    
    let local_label = just(Token::Punctuation('.'))
        .ignore_then(select! { Token::Label(s) => s })
        .map(|s| format!(".{}", s));

    normal_label.or(local_label)
        .then_ignore(just(Token::Punctuation(':')))
        .map(Statement::Label)
        .labelled("label")
}

pub fn program_parser<'a>() -> impl Parser<'a, &'a [Token], Vec<Statement>, ParseErr<'a>> + Clone {
    choice((
        label_parser(),
        data_definition_parser(), // Check "Label Directive" before "Instruction"
        instruction_parser(),
        standalone_directive_parser(),
    ))
    .recover_with(skip_then_retry_until(
        any().ignored(), 
        choice((
            select! { Token::Mnemonic(_) => () },
            select! { Token::Directive(_) => () },
            // Careful recovery around labels
            select! { Token::Label(_) => () }.then(just(Token::Punctuation(':'))).ignored(),
        ))
    ))
    .repeated()
    .collect()
}