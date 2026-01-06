use crate::ast::{LineNode, Operand, Program, Statement};
use crate::semantics::validator::SymbolInfo;
use std::collections::HashMap;

// PHASE 3: Determine Sizes & Addresses
pub fn pass_one(
    program: &Program,
    symbol_table: &mut HashMap<String, SymbolInfo>,
) -> HashMap<usize, u64> {
    // Returns a map of Statement Index -> Address

    let mut address_map = HashMap::new();
    let mut location_counter: u64 = 0x0250; // Requirement: Start at 0250h

    for (index, spanned) in program.iter().enumerate() {
        // Store current address for this statement
        // Note: For Empty/Error lines, we might still store an address (the current LC)
        // or skip it. If we skip it, the frontend map lookup might fail if it expects 1-to-1.
        // But the frontend iterates lines and checks if address_map has key.

        // We will store address for ALL lines, so the frontend can show the address even for errors/comments
        address_map.insert(index, location_counter);

        if let LineNode::Statement(stmt) = &spanned.node {
            match stmt {
                Statement::Segment { name: _ } => {
                    location_counter = 0x0000; // Reset for new segment (or align)
                    // If we want a specific org for code segment, we might check name
                }
                Statement::Label(name) => {
                    // Update Symbol Table with the calculated address
                    if let Some(sym) = symbol_table.get_mut(name) {
                        sym.offset = Some(location_counter);
                    }
                }
                // Inside the match stmt loop:
                Statement::Variable {
                    name,
                    directive,
                    value,
                } => {
                    // START FIX
                    if let Some(sym) = symbol_table.get_mut(name) {
                        sym.offset = Some(location_counter);
                    }
                    // END FIX
                    let size = get_variable_size(directive, value);
                    location_counter += size;
                }
                Statement::Data { directive, value } => {
                    // Note: Data statements usually don't have names in your AST unless
                    // wrapped in Variable, but if they do, update here too.
                    let size = get_variable_size(directive, value);
                    location_counter += size;
                }
                Statement::Instruction { mnemonic, operands } => {
                    // ESTIMATE instruction size.
                    let size = estimate_instruction_size(mnemonic, operands);
                    location_counter += size;
                }
                _ => {}
            }
        }
    }
    address_map
}

fn get_variable_size(directive: &str, value: &Operand) -> u64 {
    match directive.to_uppercase().as_str() {
        "DB" => match value {
            Operand::StringLiteral(s) => s.len() as u64,
            _ => 1,
        },
        "DW" => 2,
        "DD" => 4,
        _ => 0,
    }
}

fn estimate_instruction_size(mnemonic: &str, operands: &[Operand]) -> u64 {
    // Simplified 8086 sizing:
    // Basic: 2 bytes
    // Immediate 16-bit: +2 bytes or +1 byte if 8-bit
    // Memory displacement: +2 bytes

    // This is a rough estimator. For accurate sizing, we need to know exact opcode.
    // Pass 2 will be the authority, but Pass 1 needs to be close for labels.

    let base_size = match mnemonic.to_uppercase().as_str() {
        "RET" | "NOP" | "INT" => 1, // Single byte usually, INT is 2 but handled below
        _ => 2,
    };

    let mut extra = 0;
    if mnemonic.to_uppercase() == "INT" {
        return 2;
    }

    for op in operands {
        match op {
            Operand::Immediate(val, _) => {
                if *val > 255 {
                    extra += 2;
                } else {
                    extra += 1;
                }
            }
            Operand::Memory { .. } => extra += 2,
            _ => {}
        }
    }

    // Clamp/Fix common cases
    if mnemonic.to_uppercase() == "MOV" {
        // MOV reg, reg = 2
        // MOV reg, imm16 = 3 or 4
        // MOV reg, imm8 = 2 or 3
        return 2 + extra.min(2); // Heuristic
    }

    base_size + extra
}

// PHASE 4: Generate Machine Code
pub fn pass_two(program: &Program, _address_map: &HashMap<usize, u64>) -> HashMap<usize, String> {
    // Returns map of Statement Index -> Hex String

    let mut encoding_map = HashMap::new();

    for (index, spanned) in program.iter().enumerate() {
        if let LineNode::Statement(stmt) = &spanned.node {
            match stmt {
                Statement::Instruction { mnemonic, operands } => {
                    let bytes = encode_instruction(mnemonic, operands);
                    if !bytes.is_empty() {
                        let hex_string = bytes
                            .iter()
                            .map(|b| format!("{:02X}", b))
                            .collect::<Vec<String>>()
                            .join(" ");
                        encoding_map.insert(index, hex_string);
                    }
                }
                Statement::Variable {
                    value, directive, ..
                } => {
                    let hex = encode_data(directive, value);
                    encoding_map.insert(index, hex);
                }
                Statement::Data {
                    value, directive, ..
                } => {
                    let hex = encode_data(directive, value);
                    encoding_map.insert(index, hex);
                }
                _ => {}
            }
        }
    }
    encoding_map
}

fn encode_data(directive: &str, value: &Operand) -> String {
    match value {
        Operand::Immediate(val, _) => {
            if directive.to_uppercase() == "DW" {
                // Little Endian
                format!("{:02X} {:02X}", val & 0xFF, (val >> 8) & 0xFF)
            } else {
                format!("{:02X}", val & 0xFF)
            }
        }
        Operand::StringLiteral(s) => s
            .bytes()
            .map(|b| format!("{:02X}", b))
            .collect::<Vec<String>>()
            .join(" "),
        _ => "".to_string(),
    }
}

fn encode_instruction(mnemonic: &str, operands: &[Operand]) -> Vec<u8> {
    let mnem = mnemonic.to_uppercase();

    // Note: This is a partial implementation of 8086 encoding
    match mnem.as_str() {
        "MOV" => {
            if operands.len() == 2 {
                match (&operands[0], &operands[1]) {
                    // MOV Reg, Reg
                    (Operand::Register(dest), Operand::Register(src)) => {
                        // Table for simplified Register encoding
                        // AX=000, CX=001, DX=010, BX=011, SP=100, BP=101, SI=110, DI=111
                        // Opcode 89 /r (MOV r/m16, r16) -> ModR/M byte
                        // Mod=11 (Reg mode)
                        // Reg=Src, R/M=Dest
                        let d = reg_code(dest);
                        let s = reg_code(src);
                        let mod_rm = 0xC0 | (s << 3) | d;
                        vec![0x89, mod_rm]
                    }
                    // MOV Reg, Imm
                    (Operand::Register(dest), Operand::Immediate(val, _)) => {
                        // B8+reg for 16-bit
                        let reg = reg_code(dest);
                        // Check if 8-bit or 16-bit register not fully supported here, assuming 16-bit (AX, BX etc)
                        if is_16bit_reg(dest) {
                            vec![0xB8 + reg, (val & 0xFF) as u8, (val >> 8) as u8]
                        } else {
                            // 8-bit mov: B0+reg
                            vec![0xB0 + reg, (val & 0xFF) as u8]
                        }
                    }
                    _ => vec![],
                }
            } else {
                vec![]
            }
        }
        "INT" => {
            if let Some(Operand::Immediate(val, _)) = operands.first() {
                vec![0xCD, *val as u8]
            } else {
                vec![]
            }
        }
        "NOP" => vec![0x90],
        "RET" => vec![0xC3],
        "ADD" => {
            if operands.len() == 2 {
                // ADD Reg, Reg -> 01 /r
                match (&operands[0], &operands[1]) {
                    (Operand::Register(dest), Operand::Register(src)) => {
                        let d = reg_code(dest);
                        let s = reg_code(src);
                        let mod_rm = 0xC0 | (s << 3) | d;
                        vec![0x01, mod_rm]
                    }
                    _ => vec![],
                }
            } else {
                vec![]
            }
        }
        "SUB" => {
            if operands.len() == 2 {
                match (&operands[0], &operands[1]) {
                    (Operand::Register(dest), Operand::Register(src)) => {
                        let d = reg_code(dest);
                        let s = reg_code(src);
                        let mod_rm = 0xC0 | (s << 3) | d;
                        vec![0x29, mod_rm]
                    }
                    _ => vec![],
                }
            } else {
                vec![]
            }
        }
        "INC" => {
            if let Some(Operand::Register(reg)) = operands.first() {
                if is_16bit_reg(reg) {
                    vec![0x40 + reg_code(reg)]
                } else {
                    vec![0xFE, 0xC0 + reg_code(reg)] // INC r8
                }
            } else {
                vec![]
            }
        }
        "DEC" => {
            if let Some(Operand::Register(reg)) = operands.first() {
                if is_16bit_reg(reg) {
                    vec![0x48 + reg_code(reg)]
                } else {
                    vec![0xFE, 0xC8 + reg_code(reg)] // DEC r8
                }
            } else {
                vec![]
            }
        }
        _ => vec![],
    }
}

fn reg_code(reg: &str) -> u8 {
    match reg.to_uppercase().as_str() {
        "AL" | "AX" => 0,
        "CL" | "CX" => 1,
        "DL" | "DX" => 2,
        "BL" | "BX" => 3,
        "AH" | "SP" => 4,
        "CH" | "BP" => 5,
        "DH" | "SI" => 6,
        "BH" | "DI" => 7,
        _ => 0,
    }
}

fn is_16bit_reg(reg: &str) -> bool {
    matches!(
        reg.to_uppercase().as_str(),
        "AX" | "CX" | "DX" | "BX" | "SP" | "BP" | "SI" | "DI"
    )
}
