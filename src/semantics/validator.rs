#![allow(unused)]
#![allow(dead_code)]

use crate::ast::{Operand, Program, Statement};

#[derive(PartialEq, Debug)]
pub enum Flavor {
    Masm,
    Nasm,
}

#[derive(Debug)]
pub struct CompilerError {
    pub message: String,
    pub line: usize, // Simplified for example
}

// pub fn validate(ast: &Program, flavor: Flavor, strict: bool) -> Vec<CompilerError> {
//     let mut errors = Vec::new();

//     for stmt in ast {
//         match stmt {
//             Statement::Variable { value, .. } => {
//                 if let Operand::Immediate(_, raw) = value {
//                     validate_number(raw, strict, &mut errors);
//                 }
//             }
//             Statement::Instruction { operands, .. } => {
//                 for op in operands {
//                     if let Operand::Immediate(_, raw) = op {
//                         validate_number(raw, strict, &mut errors);
//                     }
//                     // Hybrid Check: NASM requires [] for memory
//                     if flavor == Flavor::Nasm {
//                         // todo: check if operand is used as memory without brackets
//                         if let Operand::Label(name) = op {
//                             // In NASM, 'mov ax, var' is valid but loads address.
//                             // If user meant memory, they need [var].
//                             // We might warn here or strictly enforce brackets depending on interpretation.
//                         }
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }
//     errors
// }

pub fn validate(ast: &Program, flavor: Flavor, strict: bool) -> Vec<CompilerError> {
    let mut errors = Vec::new();

    for stmt in ast {
        match stmt {
            Statement::Variable { value, .. } | Statement::Data { value, .. } => {
                if let Operand::Immediate(_, raw) = value {
                    validate_number(raw, strict, &mut errors);
                }
            }
            Statement::Instruction { operands, .. } => {
                for op in operands {
                    if let Operand::Immediate(_, raw) = op {
                        validate_number(raw, strict, &mut errors);
                    }
                }
            }
            _ => {}
        }
    }
    errors
}

fn validate_number(raw: &str, strict: bool, errors: &mut Vec<CompilerError>) {
    if !strict {
        return;
    }

    // fn some_condition() -> bool { false } // Placeholder

    // RULE: Binary must be 8 or 16 bits
    if raw.ends_with('b') || raw.ends_with('B') {
        let digits = &raw[..raw.len() - 1];
        if digits.len() != 8 && digits.len() != 16 {
            errors.push(CompilerError {
                message: format!(
                    "Strict Mode: Binary literal '{}' must be exactly 8 or 16 bits",
                    raw
                ),
                line: 0, // Need to propagate spans from AST to fix this
            });
        }
    }

    // RULE: Hex cannot start with 0 (wait, usually valid hex MUST start with 0 if next char is letter in MASM)
    // Teacher said "Hex can't start with 0"? Or maybe "Hex must start with digit"?
    // Usually the rule is: 0FFh (valid), FFh (invalid in MASM because looks like ident).
    // If teacher meant "Don't use 0x", check for '0x' prefix.
    if raw.starts_with("0x") {
        errors.push(CompilerError {
            message: "Strict Mode: C-style '0x' hex literals are not allowed. Use 'h' suffix."
                .to_string(),
            line: 0,
        });
    }
}
