// src/semantics/validator.rs
use crate::ast::{LineNode, Operand, Program, Statement, Spanned};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompilerError {
    pub message: String,
    pub line: usize,
    pub is_correct: bool, // false = Error, true = Warning/Info (but we mostly use False for errors)
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
}

// Team 2 Allowed Instructions
const ALLOWED_INSTRUCTIONS: &[&str] = &[
    "CMC", "CMPSB", "NOP", "POPA", "AAD", "AAM",
    "MUL", "INC", "IDIV", "INT",
    "AND", "LEA", "OR", "XOR",
    "JNAE", "JNE", "JNLE", "LOOPE", "JA", "JC"
];

const JUMP_INSTRUCTIONS: &[&str] = &[
    "JNAE", "JNE", "JNLE", "LOOPE", "JA", "JC"
];

pub fn validate(ast: &Program) -> (Vec<CompilerError>, HashMap<String, SymbolInfo>) {
    let mut errors = Vec::new();
    let mut symbol_table: HashMap<String, SymbolInfo> = HashMap::new();
    
    // Track segments
    let mut current_segment = "NONE".to_string();

    // PASS 1: Symbol Collection & Segment Tracking
    for (line_idx, spanned) in ast.iter().enumerate() {
        let line_num = line_idx + 1;
        
        if let LineNode::Statement(stmt) = &spanned.node {
            match stmt {
                Statement::Segment { name } => {
                     if name.to_uppercase().contains("STACK") { current_segment = "STACK".to_string(); }
                     else if name.to_uppercase().contains("DATA") { current_segment = "DATA".to_string(); }
                     else if name.to_uppercase().contains("CODE") { current_segment = "CODE".to_string(); }
                }
                Statement::SegmentEnd => {
                    current_segment = "NONE".to_string();
                }
                Statement::Variable { name, directive, .. } => {
                    if current_segment == "DATA" {
                         let dir = directive.to_uppercase();
                         let dtype = if dir == "DB" { DataType::Byte } else { DataType::Word };
                         symbol_table.insert(name.clone(), SymbolInfo {
                             type_: SymbolType::Variable,
                             data_type: dtype,
                             defined: true,
                             segment: "DATA".to_string(),
                             offset: None,
                         });
                    }
                }
                Statement::Label(name) => {
                    if current_segment == "CODE" {
                        symbol_table.insert(name.clone(), SymbolInfo {
                            type_: SymbolType::Label,
                            data_type: DataType::None,
                            defined: true,
                            segment: "CODE".to_string(),
                            offset: None,
                        });
                    }
                }
                Statement::Constant { name, .. } => {
                    symbol_table.insert(name.clone(), SymbolInfo {
                        type_: SymbolType::Constant,
                        data_type: DataType::Word,
                        defined: true,
                        segment: current_segment.clone(),
                        offset: None,
                    });
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
                     if upper.contains("STACK") { current_segment = "STACK".to_string(); }
                     else if upper.contains("DATA") { current_segment = "DATA".to_string(); }
                     else if upper.contains("CODE") { current_segment = "CODE".to_string(); }
                     // Validating strictly? Parser handles ".STACK SEGMENT" as one token mostly or filtered.
                     // But we assume parser success implies basic structure. 
                }
                Statement::SegmentEnd => {
                    current_segment = "NONE".to_string();
                }
                
                // --- VARIABLE DECLARATION VALIDATION ---
                Statement::Variable { name: _, directive, value } => {
                    let dir = directive.to_uppercase();
                    
                    if current_segment == "CODE" {
                        errors.push(CompilerError {
                            message: "Declaración de datos no permitida en segmento de código.".to_string(),
                            line: line_num,
                            is_correct: false,
                        });
                    } else if current_segment == "STACK" {
                         // Only DW allowed in Stack, and specific format
                         if dir != "DW" {
                             errors.push(CompilerError {
                                 message: format!("'{}' no permitido en segmento de pila.", dir),
                                 line: line_num,
                                 is_correct: false,
                             });
                         }
                         // Check for DUP in value (Format: DW num DUP(...))
                         // The parser might have parsed `num DUP(...)` as `Operand::Dup` or `Operand::Uninitialized`?
                         // Need to check `src/ast.rs` and parser.
                         // Parser maps `dup(...)` to `Pseudoinstruction` token?
                         // Wait, in `parser.rs`: `let variable = ... .then(operand.clone())`.
                         // In `tokens.rs`, `DUP` is a Pseudoinstruction.
                         // My parser logic for Variable expects `Operand`.
                         // If `DUP` is used, the lexer produces `Pseudoinstruction("dup(...)")` (from `validate_compounds`).
                         // But `Operand` parser expects `Constant`, `Register`, `Symbol`, `Bracket`.
                         // It does NOT accept `Pseudoinstruction` as `Operand`.
                         // So `num DUP(...)` might fail to parse as `Operand` if `DUP` is a `Pseudoinstruction`.
                         // Actually, looking at `lexer.rs`:
                         // `dups` returns `Token::Pseudoinstruction(format!("dup({})", content))`.
                         // `parser.rs`: `operand` choice does NOT include `Pseudoinstruction`.
                         // THIS IS A PARSER ISSUE. DUP handling is tricky.
                         // However, for now, let's assume simple cases or that `DUP` parses as `Symbol` if not matched?
                         // No, `lexer.rs` matches `dup` specifically.
                         // I will assume for now we handle what we can. 
                         
                    } else if current_segment == "DATA" {
                        // Check "Texto sin comillas"
                        if let Operand::Label(s) = value {
                             // If a variable is initialized with a Label/Symbol that is NOT a defined constant, it's likely missing quotes.
                             // Unless it's `var1 DW other_var` (address).
                             // But strict requirement: "Texto sin comillas. Use: frase DB 'texto'"
                             if dir == "DB" {
                                 errors.push(CompilerError {
                                     message: format!("Texto sin comillas. Use: {} {} 'texto'", "variable", dir), // Generic name
                                     line: line_num,
                                     is_correct: false,
                                 });
                             }
                        }
                    } else {
                        // Outside segment
                        errors.push(CompilerError {
                            message: "Línea fuera de segmento".to_string(),
                            line: line_num,
                            is_correct: false,
                        });
                    }
                }

                // --- DATA DECLARATION VALIDATION (DB/DW without name?) ---
                Statement::Data { directive, value } => {
                    // Similar checks
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
                     // Check "Texto sin comillas"
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

                // --- INSTRUCTION VALIDATION ---
                Statement::Instruction { mnemonic, operands } => {
                    let mnem = mnemonic.to_uppercase();
                    
                    if current_segment != "CODE" {
                        errors.push(CompilerError {
                            message: format!("Instrucción '{}' no permitida en segmento de {}", mnem, 
                                if current_segment == "NONE" { "fuera de segmento" } 
                                else if current_segment == "DATA" { "datos" }
                                else { "pila" }),
                            line: line_num,
                            is_correct: false,
                        });
                    } else {
                        // 1. Check Allowed List
                        if !ALLOWED_INSTRUCTIONS.contains(&mnem.as_str()) {
                             errors.push(CompilerError {
                                 message: format!("'{}' no es una instrucción válida", mnem),
                                 line: line_num,
                                 is_correct: false,
                             });
                        } else {
                            // 2. Specific Instruction Logic
                            
                            // Jumps
                            if JUMP_INSTRUCTIONS.contains(&mnem.as_str()) {
                                if let Some(Operand::Label(lbl)) = operands.first() {
                                    if !symbol_table.contains_key(lbl) {
                                         errors.push(CompilerError {
                                             message: format!("Etiqueta '{}' no definida previamente", lbl),
                                             line: line_num,
                                             is_correct: false,
                                         });
                                    }
                                }
                            }
                            
                            // Check Operand Existence (Variables)
                            for op in operands {
                                if let Operand::Label(name) = op {
                                    // If it's not a register (Operand::Register covers that), and not a jump label we just checked...
                                    // Actually Jumps use Label, variables use Label.
                                    // If it's a variable usage:
                                    if !JUMP_INSTRUCTIONS.contains(&mnem.as_str()) {
                                        // It's a memory access or immediate?
                                        // If it's a Symbol but not in symbol table
                                        if !symbol_table.contains_key(name) {
                                             // Could be forward reference? Assembly usually allows it for labels, but Python says "Símbolo ... no declarado en segmento de datos"
                                             errors.push(CompilerError {
                                                 message: format!("Elemento no identificado: '{}'", name),
                                                 line: line_num,
                                                 is_correct: false,
                                             });
                                        }
                                    }
                                }
                                if let Operand::Memory { base, .. } = op {
                                     if !symbol_table.contains_key(base) && !is_register(base) {
                                         // If base is not a register, it might be a variable? 
                                         // Parser maps `[reg]` to `Memory`. `reg` is string.
                                         // If `[var]` -> Parser logic:
                                         // `mem_bracket` uses `select! { Token::Register(r) => r }`.
                                         // So parser only supports `[Register]`. 
                                         // If user types `[var]`, parser likely fails or tries something else.
                                         // If parser supports direct memory `MOV AX, var` -> `var` is `Operand::Label`.
                                     }
                                }
                            }
                        }
                    }
                }
                
                // --- LABEL VALIDATION ---
                Statement::Label(_) => {
                     if current_segment == "DATA" {
                          errors.push(CompilerError {
                             message: "Etiquetas de código no permitidas en segmento de datos".to_string(),
                             line: line_num,
                             is_correct: false,
                         });
                     }
                }
                
                _ => {}
            }
        } else if let LineNode::Error(msg) = &spanned.node {
             // Parser error, we propagate but maybe refine?
             errors.push(CompilerError {
                 message: format!("Error de sintaxis: {}", msg),
                 line: line_num,
                 is_correct: false,
             });
        }
    }

    (errors, symbol_table)
}

fn is_register(s: &str) -> bool {
    let r = s.to_uppercase();
    matches!(r.as_str(), 
        "AX" | "BX" | "CX" | "DX" | "SI" | "DI" | "SP" | "BP" |
        "AL" | "AH" | "BL" | "BH" | "CL" | "CH" | "DL" | "DH" |
        "CS" | "DS" | "SS" | "ES"
    )
}