// src/tokens/pseudoinstruction.rs
use super::*;
use serde::Serialize;

define_tokens!(Pseudoinstruction {
    // Existing
    Org => ["ORG"],
    Segment => ["SEGMENT"], Ends => ["ENDS"], Db => ["DB"], Dw => ["DW"],
    Dd => ["DD"], Dup => ["DUP"], Equ => ["EQU"], End => ["END"],
    // Single word PTRs (if they appear alone)
    Ptr => ["PTR"], 
    Byte => ["BYTE"], Word => ["WORD"], Dword => ["DWORD"],
    
    // Standard Sections
    Data => [".DATA"], Code => [".CODE"], Text => [".TEXT"], Bss => [".BSS"],
    // Global => ["GLOBAL", "GLOBL"], Section => ["SECTION"], Macro => ["MACRO"],
    Global => ["GLOBAL"] , Section => ["SECTION"], Macro => ["MACRO"],

    // --- NEW COMPOUND TYPES FOR ACADEMIC REQUIREMENT ---
    // These allow the tokenizer to accept "BYTE PTR" as a single valid token
    BytePtr => ["BYTE PTR"],
    WordPtr => ["WORD PTR"],
    DwordPtr => ["DWORD PTR"],
    DataSegment => [".DATA SEGMENT"],
    CodeSegment => [".CODE SEGMENT"],
    StackSegment => [".STACK SEGMENT"],
});