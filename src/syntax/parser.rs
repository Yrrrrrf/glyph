// src/syntax/parser.rs
use crate::ast::{LineNode, Operand, Program, Statement};
use crate::syntax::tokens::{PunctuationType, Token, constant};
use chumsky::input::ValueInput;
use chumsky::prelude::*;

pub fn parser<'a, I>() -> impl Parser<'a, I, Program, extra::Err<Rich<'a, Token>>>
where
    I: ValueInput<'a, Token = Token, Span = SimpleSpan>,
{
    // --- OPERANDS ---
    let imm = select! {
        Token::Constant(constant::Type::NumberDecimal(v)) => Operand::Immediate(v, v.to_string()),
        Token::Constant(constant::Type::NumberHex(v, raw)) => Operand::Immediate(v, raw),
        Token::Constant(constant::Type::NumberBinary(v, raw)) => Operand::Immediate(v, raw),
        Token::Constant(constant::Type::Char(c)) => Operand::Immediate(c as u64, format!("'{}'", c)),
        Token::Constant(constant::Type::String(s)) => Operand::StringLiteral(s),
    };

    let reg = select! { Token::Register(r) => Operand::Register(r) };
    let lbl = select! { Token::Symbol(s) => Operand::Label(s) };

    // Update Bracket Matching to use PunctuationType
    let mem_bracket = just(Token::Punctuation(PunctuationType::LBracket))
        .ignore_then(select! { Token::Register(r) => r })
        .then_ignore(just(Token::Punctuation(PunctuationType::RBracket)))
        .map(|r| Operand::Memory {
            base: r,
            offset: None,
        });

    let operand = choice((imm, reg, mem_bracket, lbl));

    // --- STATEMENTS ---

    // 1. Instruction
    let instruction = select! { Token::Instruction(_, op) => op }
        .then(
            operand
                .clone()
                .separated_by(just(Token::Punctuation(PunctuationType::Comma)))
                .collect(),
        )
        .map(|(op, ops)| Statement::Instruction {
            mnemonic: op,
            operands: ops,
        });

    // 2. Label
    let label = select! { Token::Symbol(name) => name }
        .then_ignore(just(Token::Punctuation(PunctuationType::Colon)))
        .map(Statement::Label);

    // 3. Variable
    let variable = select! { Token::Symbol(name) => name }
        .then(select! { Token::Pseudoinstruction(d) => d })
        .then(operand.clone())
        .map(|((name, dir), val)| Statement::Variable {
            name,
            directive: dir,
            value: val,
        });

    // 4. Data
    let data = select! { Token::Pseudoinstruction(d) => d }
        .then(operand.clone())
        .map(|(dir, val)| Statement::Data {
            directive: dir,
            value: val,
        });

    // 5. Segment
    let segment = select! { Token::Pseudoinstruction(d) => d }.map(|name| {
        if name.to_uppercase() == "ENDS" {
            Statement::SegmentEnd
        } else {
            Statement::Segment { name }
        }
    });

    let statement = choice((segment, label, variable, data, instruction)).map(LineNode::Statement);

    // --- LINE PARSER WITH RECOVERY ---
    let line = choice((
        // Case 1: Valid Statement + Newline/EOF
        statement.then_ignore(just(Token::Newline).or(end().to(Token::Newline))),
        // Case 2: Empty Line (just Newline)
        just(Token::Newline).to(LineNode::Empty),
    ))
    .map_with(|node, e| {
        let span: SimpleSpan = e.span();
        crate::ast::Spanned {
            node,
            span: (span.start, span.end),
        }
    })
    .recover_with(via_parser(
        any()
            .and_is(just(Token::Newline).not())
            .repeated()
            .then(just(Token::Newline))
            .map_with(|_, e| {
                let span: SimpleSpan = e.span();
                crate::ast::Spanned {
                    node: LineNode::Error("Syntax Error".to_string()),
                    span: (span.start, span.end),
                }
            }),
    ));

    line.repeated().collect()
}
