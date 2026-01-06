// src/semantics/validator.rs
use crate::ast::{LineNode, Operand, Program, Statement};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct CompilerError {
    pub message: String,
    pub line: usize,
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable,
    Label,
    Constant,
}

// FIX: Add PartialEq here
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Byte, // 8-bit
    Word, // 16-bit
    None, // Label
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub type_: SymbolType,
    pub data_type: DataType,
    pub defined: bool,
    pub offset: Option<u64>,
}

// FIX: Updated signature (1 argument)
pub fn validate(ast: &Program) -> (Vec<CompilerError>, HashMap<String, SymbolInfo>) {
    let mut errors = Vec::new();
    let mut symbol_table: HashMap<String, SymbolInfo> = HashMap::new();

    // PASS 1: Symbol Collection
    for spanned in ast {
        if let LineNode::Statement(stmt) = &spanned.node {
            match stmt {
                Statement::Variable {
                    name,
                    directive,
                    value,
                } => {
                    let dir = directive.to_uppercase();
                    let dtype = if dir == "DB" {
                        DataType::Byte
                    } else {
                        DataType::Word
                    };

                    symbol_table.insert(
                        name.clone(),
                        SymbolInfo {
                            type_: SymbolType::Variable,
                            data_type: dtype,
                            defined: true,
                            offset: None,
                        },
                    );

                    validate_variable_value(name, &dir, value, &mut errors);
                }
                Statement::Label(name) => {
                    symbol_table.insert(
                        name.clone(),
                        SymbolInfo {
                            type_: SymbolType::Label,
                            data_type: DataType::None,
                            defined: true,
                            offset: None,
                        },
                    );
                }
                Statement::Constant { name, value: _ } => {
                    symbol_table.insert(
                        name.clone(),
                        SymbolInfo {
                            type_: SymbolType::Constant,
                            data_type: DataType::Word,
                            defined: true,
                            offset: None,
                        },
                    );
                }
                _ => {}
            }
        }
    }

    // PASS 2: Instruction Validation
    for spanned in ast {
        if let LineNode::Statement(stmt) = &spanned.node {
            if let Statement::Instruction { mnemonic, operands } = stmt {
                validate_instruction(mnemonic, operands, &symbol_table, &mut errors);
            }
        }
    }

    (errors, symbol_table)
}

fn validate_variable_value(
    _name: &str,
    directive: &str,
    value: &Operand,
    errors: &mut Vec<CompilerError>,
) {
    let max_val = if directive == "DB" { 255 } else { 65535 };

    match value {
        Operand::Immediate(val, _) => {
            if *val > max_val {
                errors.push(CompilerError {
                    message: format!(
                        "Size Error: '{}' is too large for {}. Max: {}",
                        val, directive, max_val
                    ),
                    line: 0,
                });
            }
        }
        Operand::StringLiteral(s) => {
            if directive == "DW" && s.len() > 2 {
                errors.push(CompilerError {
                    message: format!("Error: DW cannot contain long string '{}'", s),
                    line: 0,
                });
            }
        }
        _ => {}
    }
}

fn validate_instruction(
    mnemonic: &str,
    operands: &[Operand],
    symbols: &HashMap<String, SymbolInfo>,
    errors: &mut Vec<CompilerError>,
) {
    let mnem = mnemonic.to_uppercase();

    // 1. Check Label Existence for Jumps
    if is_jump(&mnem) {
        if let Some(Operand::Label(name)) = operands.first() {
            if !symbols.contains_key(name) {
                errors.push(CompilerError {
                    message: format!("Error: Label '{}' is not defined.", name),
                    line: 0,
                });
            }
        }
    }

    // 2. Check Operand Size Mismatch
    if operands.len() == 2 {
        let op1_size = get_operand_size(&operands[0], symbols);
        let op2_size = get_operand_size(&operands[1], symbols);

        if let (Some(s1), Some(s2)) = (op1_size, op2_size) {
            if s1 != s2 {
                errors.push(CompilerError {
                    message: format!(
                        "Type Mismatch: Operands have different sizes ({:?} vs {:?}) in '{}'",
                        s1, s2, mnem
                    ),
                    line: 0,
                });
            }
        }
    }
}

fn get_operand_size(op: &Operand, symbols: &HashMap<String, SymbolInfo>) -> Option<DataType> {
    match op {
        Operand::Register(r) => {
            let r = r.to_uppercase();
            if r.ends_with('X')
                || ["SI", "DI", "SP", "BP", "CS", "DS", "SS", "ES"].contains(&r.as_str())
            {
                Some(DataType::Word)
            } else {
                Some(DataType::Byte)
            }
        }
        Operand::Label(name) | Operand::Memory { base: name, .. } => {
            if let Some(info) = symbols.get(name) {
                Some(info.data_type.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn is_jump(mnem: &str) -> bool {
    matches!(mnem, "JMP" | "JE" | "JZ" | "JNZ" | "JC" | "JNC" | "LOOP")
}
