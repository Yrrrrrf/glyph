#![allow(dead_code)]
//! CPU Instructions with subcategories
use super::*;
use serde::Serialize;

define_tokens!(Instruction {
    // Data Transfer
    Mov => ["MOV"], Push => ["PUSH"], Pop => ["POP"], Xchg => ["XCHG"],
    // Arithmetic
    Add => ["ADD"], Sub => ["SUB"], Mul => ["MUL"], Div => ["DIV"],
    Inc => ["INC"], Dec => ["DEC"], Neg => ["NEG"],
    // Logic
    And => ["AND"], Or => ["OR"], Xor => ["XOR"], Not => ["NOT"],
    // Control Flags
    Clc => ["CLC"], Stc => ["STC"], Cmc => ["CMC"], Cld => ["CLD"], Std => ["STD"],
    // Conditional Jump
    Je => ["JE"], Jne => ["JNE"], Jz => ["JZ"], Jnz => ["JNZ"],
    Jg => ["JG"], Jl => ["JL"], Jge => ["JGE"], Jle => ["JLE"],
    // Control Transfer
    Jmp => ["JMP"], Call => ["CALL"], Ret => ["RET"],
    // Interrupt
    Int => ["INT"], Syscall => ["SYSCALL"],
});

/// Instruction classification for analysis
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum InstructionType {
    DataTransfer,
    Arithmetic,
    Logic,
    ControlFlags,
    ControlTransfer,
    ConditionalJump,
    Interrupt,
}

impl Instruction {
    pub fn instruction_type(&self) -> InstructionType {
        use Instruction::*;
        match self {
            Mov | Push | Pop | Xchg => InstructionType::DataTransfer,
            Add | Sub | Mul | Div | Inc | Dec | Neg => InstructionType::Arithmetic,
            And | Or | Xor | Not => InstructionType::Logic,
            Clc | Stc | Cmc | Cld | Std => InstructionType::ControlFlags,
            Jmp | Call | Ret => InstructionType::ControlTransfer,
            Je | Jne | Jz | Jnz | Jg | Jl | Jge | Jle => InstructionType::ConditionalJump,
            Int | Syscall => InstructionType::Interrupt,
        }
    }
}
