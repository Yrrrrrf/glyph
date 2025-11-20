#![allow(dead_code)]
#![allow(non_camel_case_types)]
//! CPU Instructions with automatic categorization
use crate::define_categorized_tokens;

use super::*;

define_categorized_tokens!(Instruction, InstructionType {
    DataTransfer => [MOV, PUSH, POP, XCHG],
    Arithmetic => [ADD, SUB, MUL, DIV, INC, DEC, NEG],
    Logic => [AND, OR, XOR, NOT],
    ControlFlags => [CLC, STC, CMC, CLD, STD],
    ConditionalJump => [JE, JNE, JZ, JNZ, JG, JL, JGE, JLE],
    ControlTransfer => [JMP, CALL, RET],
    Interrupt => [INT, SYSCALL],
});
