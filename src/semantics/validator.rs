// src/semantics/validator.rs
use crate::ast::{LineNode, Operand, Program, Statement};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilerError {
    pub message: String,
    pub line: usize,
    pub is_correct: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolType {
    Variable,
    Label,
    Constant,
}

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
    pub segment: String,
    pub offset: Option<u64>,
    pub line_defined: usize,
}

const ALLOWED_INSTRUCTIONS: &[&str] = &[
    "CMC", "CMPSB", "NOP", "POPA", "AAD", "AAM", "MUL", "INC", "IDIV", "INT", "AND", "LEA", "OR",
    "XOR", "JNAE", "JNE", "JNLE", "LOOPE", "JA", "JC", // , "MOV", "ADD", "SUB", "DEC", "RET",
];

const JUMP_INSTRUCTIONS: &[&str] = &["JNAE", "JNE", "JNLE", "LOOPE", "JA", "JC"];

pub fn validate(ast: &Program) -> (Vec<CompilerError>, HashMap<String, SymbolInfo>) {
    let mut errors = Vec::new();
    let mut symbol_table: HashMap<String, SymbolInfo> = HashMap::new();

    let mut current_segment = "NONE".to_string();

    // PASS 1: Symbol Collection & Segment Tracking
    for (line_idx, spanned) in ast.iter().enumerate() {
        let line_num = line_idx + 1;

        if let LineNode::Statement(stmt) = &spanned.node {
            match stmt {
                Statement::Segment { name } => {
                    if name.to_uppercase().contains("STACK") {
                        current_segment = "STACK".to_string();
                    } else if name.to_uppercase().contains("DATA") {
                        current_segment = "DATA".to_string();
                    } else if name.to_uppercase().contains("CODE") {
                        current_segment = "CODE".to_string();
                    }
                }
                Statement::SegmentEnd => {
                    current_segment = "NONE".to_string();
                }
                Statement::End { .. } => {}
                Statement::Variable {
                    name, directive, ..
                } => {
                    if current_segment == "DATA" {
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
                                segment: "DATA".to_string(),
                                offset: None,
                                line_defined: line_num,
                            },
                        );
                    }
                }
                Statement::Label(name) => {
                    if current_segment == "CODE" {
                        symbol_table.insert(
                            name.clone(),
                            SymbolInfo {
                                type_: SymbolType::Label,
                                data_type: DataType::None,
                                defined: true,
                                segment: "CODE".to_string(),
                                offset: None,
                                line_defined: line_num,
                            },
                        );
                    }
                }
                Statement::Constant { name, .. } => {
                    symbol_table.insert(
                        name.clone(),
                        SymbolInfo {
                            type_: SymbolType::Constant,
                            data_type: DataType::Word,
                            defined: true,
                            segment: current_segment.clone(),
                            offset: None,
                            line_defined: line_num,
                        },
                    );
                }
                _ => {}
            }
        }
    }

    // PASS 2: Detailed Validation
    current_segment = "NONE".to_string(); // Reset

    for (line_idx, spanned) in ast.iter().enumerate() {
        let line_num = line_idx + 1;

        if let LineNode::Statement(stmt) = &spanned.node {
            match stmt {
                Statement::Segment { name } => {
                    let upper = name.to_uppercase();
                    if upper.contains("STACK") {
                        current_segment = "STACK".to_string();
                    } else if upper.contains("DATA") {
                        current_segment = "DATA".to_string();
                    } else if upper.contains("CODE") {
                        current_segment = "CODE".to_string();
                    }
                }
                Statement::SegmentEnd => {
                    current_segment = "NONE".to_string();
                }
                Statement::End { .. } => {}

                // --- VARIABLE DECLARATION VALIDATION ---
                Statement::Variable {
                    name: _,
                    directive,
                    value,
                } => {
                    // Check Hex
                    let mut op_to_check = value;
                    if let Operand::Dup { value: inner, .. } = value {
                        op_to_check = inner;
                    }
                    if let Operand::Immediate(_, raw) = op_to_check {
                        if raw.to_lowercase().ends_with('h') {
                            if let Some(first) = raw.chars().next() {
                                if !first.is_ascii_digit() {
                                    errors.push(CompilerError {
                                        message: format!(
                                            "Constante Hex inválida '{}' (falta 0 inicial)",
                                            raw
                                        ),
                                        line: line_num,
                                        is_correct: false,
                                    });
                                }
                            }
                        }
                    }

                    let dir = directive.to_uppercase();

                    if current_segment == "CODE" {
                        errors.push(CompilerError {
                            message: "Declaración de datos no permitida en segmento de código."
                                .to_string(),
                            line: line_num,
                            is_correct: false,
                        });
                    } else if current_segment == "STACK" {
                        if dir != "DW" {
                            errors.push(CompilerError {
                                message: format!("'{}' no permitido en segmento de pila.", dir),
                                line: line_num,
                                is_correct: false,
                            });
                        }
                    } else if current_segment == "DATA" {
                        if let Operand::Label(s) = value {
                            if dir == "DB" {
                                errors.push(CompilerError {
                                    message: format!(
                                        "Texto sin comillas. Use: {} {} 'texto'",
                                        "variable", dir
                                    ),
                                    line: line_num,
                                    is_correct: false,
                                });
                            }
                        }
                    } else {
                        errors.push(CompilerError {
                            message: "Línea fuera de segmento".to_string(),
                            line: line_num,
                            is_correct: false,
                        });
                    }
                }

                Statement::Data { directive, value } => {
                    let dir = directive.to_uppercase();
                    if current_segment == "CODE" {
                        errors.push(CompilerError {
                            message: format!("'{}' no permitido en segmento de código.", dir),
                            line: line_num,
                            is_correct: false,
                        });
                    } else if current_segment == "STACK" {
                        if dir != "DW" {
                            errors.push(CompilerError {
                                message: format!("'{}' no permitido en segmento de pila.", dir),
                                line: line_num,
                                is_correct: false,
                            });
                        }
                    }
                    if let Operand::Label(_) = value {
                        if dir == "DB" {
                            errors.push(CompilerError {
                                message: "Texto sin comillas. Use: DB 'texto'".to_string(),
                                line: line_num,
                                is_correct: false,
                            });
                        }
                    }
                }

                Statement::Instruction { mnemonic, operands } => {
                    let mnem = mnemonic.to_uppercase();

                    if current_segment != "CODE" {
                        errors.push(CompilerError {
                            message: format!(
                                "Instrucción '{}' no permitida en segmento de {}",
                                mnem,
                                if current_segment == "NONE" {
                                    "fuera de segmento"
                                } else if current_segment == "DATA" {
                                    "datos"
                                } else {
                                    "pila"
                                }
                            ),
                            line: line_num,
                            is_correct: false,
                        });
                    } else {
                        if !ALLOWED_INSTRUCTIONS.contains(&mnem.as_str()) {
                            errors.push(CompilerError {
                                message: format!("'{}' no es una instrucción válida", mnem),
                                line: line_num,
                                is_correct: false,
                            });
                        } else {
                            if JUMP_INSTRUCTIONS.contains(&mnem.as_str()) {
                                if let Some(Operand::Label(lbl)) = operands.first() {
                                    if !symbol_table.contains_key(lbl) {
                                        errors.push(CompilerError {
                                            message: format!(
                                                "Etiqueta '{}' no definida previamente",
                                                lbl
                                            ),
                                            line: line_num,
                                            is_correct: false,
                                        });
                                    }
                                }
                            }

                            for op in operands {
                                if let Operand::Immediate(_, raw) = op {
                                    if raw.to_lowercase().ends_with('h') {
                                        if let Some(first) = raw.chars().next() {
                                            if !first.is_ascii_digit() {
                                                errors.push(CompilerError {
                                                     message: format!("Constante Hex inválida '{}' (falta 0 inicial)", raw),
                                                     line: line_num,
                                                     is_correct: false,
                                                 });
                                            }
                                        }
                                    }
                                }
                                if let Operand::Label(name) = op {
                                    if !JUMP_INSTRUCTIONS.contains(&mnem.as_str()) {
                                        if !symbol_table.contains_key(name) {
                                            errors.push(CompilerError {
                                                message: format!(
                                                    "Elemento no identificado: '{}'",
                                                    name
                                                ),
                                                line: line_num,
                                                is_correct: false,
                                            });
                                        }
                                    }
                                }
                                if let Operand::Memory { base, .. } = op {
                                    if !symbol_table.contains_key(base) && !is_register(base) {}
                                }
                            }
                        }
                    }
                }

                Statement::Label(_) => {
                    if current_segment == "DATA" {
                        errors.push(CompilerError {
                            message: "Etiquetas de código no permitidas en segmento de datos"
                                .to_string(),
                            line: line_num,
                            is_correct: false,
                        });
                    }
                }

                _ => {}
            }
        }
    }

    (errors, symbol_table)
}

fn is_register(s: &str) -> bool {
    let r = s.to_uppercase();
    matches!(
        r.as_str(),
        "AX" | "BX"
            | "CX"
            | "DX"
            | "SI"
            | "DI"
            | "SP"
            | "BP"
            | "AL"
            | "AH"
            | "BL"
            | "BH"
            | "CL"
            | "CH"
            | "DL"
            | "DH"
            | "CS"
            | "DS"
            | "SS"
            | "ES"
    )
}
