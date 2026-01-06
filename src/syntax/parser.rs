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

    // --- DUP PATTERN ---
    let dup_val = select! {
        Token::Constant(constant::Type::NumberDecimal(v)) => v,
        Token::Constant(constant::Type::NumberHex(v, _)) => v,
    }
    .then_ignore(select! { Token::Pseudoinstruction(s) if s == "DUP" => s })
    .then_ignore(just(Token::Punctuation(PunctuationType::LParen)))
    .then(operand.clone())
    .then_ignore(just(Token::Punctuation(PunctuationType::RParen)))
    .map(|(count, val)| Operand::Dup {
        count,
        value: Box::new(val),
    });

    let variable_value = choice((dup_val.clone(), operand.clone()));

    // --- STATEMENTS ---

    // 1. Instruction
    let instruction = select! {
        Token::Instruction(_, op) => op,
        Token::Symbol(op) => op,
    }
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
        .then(variable_value.clone())
        .map(|((name, dir), val)| Statement::Variable {
            name,
            directive: dir,
            value: val,
        });

    // 4. Data (Anonymous definition)
    // We split this into two explicit cases to ensure DUP is prioritized

    // Case A: DW 100 DUP(0)
    let anonymous_dup = select! { Token::Pseudoinstruction(d) => d }
        .then(dup_val.clone())
        .map(|(dir, val)| Statement::Data {
            directive: dir,
            value: val,
        });

    // Case B: DW 100
    let anonymous_std = select! { Token::Pseudoinstruction(d) => d }
        .then(operand.clone())
        .map(|(dir, val)| Statement::Data {
            directive: dir,
            value: val,
        });

    // Priority: Try DUP first, then standard
    let anonymous_data = choice((anonymous_dup, anonymous_std));

    // 5. Segment
    let segment = select! { Token::Pseudoinstruction(d) => d }.map(|name| {
        if name.to_uppercase() == "ENDS" {
            Statement::SegmentEnd
        } else {
            Statement::Segment { name }
        }
    });

    // 6. End
    let end_stmt = select! { Token::Symbol(s) if s.eq_ignore_ascii_case("END") => s }
        .then(select! { Token::Symbol(l) => l }.or_not())
        .map(|(_, l)| Statement::End { label: l });

    let statement = choice((
        label,
        variable,
        anonymous_data,
        segment,
        end_stmt,
        instruction,
    ))
    .map(LineNode::Statement);

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
