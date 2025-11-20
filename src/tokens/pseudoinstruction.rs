// src/tokens/pseudoinstruction.rs
use super::*;
use serde::Serialize;

define_tokens!(Pseudoinstruction {
    // Added Org here
    Org => ["ORG"],
    Segment => ["SEGMENT"], Ends => ["ENDS"], Db => ["DB"], Dw => ["DW"],
    Dd => ["DD"], Dup => ["DUP"], Equ => ["EQU"], End => ["END"],
    Ptr => ["PTR"], Byte => ["BYTE"], Word => ["WORD"], Dword => ["DWORD"],
    Data => [".DATA"], Code => [".CODE"], Text => [".TEXT"], Bss => [".BSS"],
    // Global => ["GLOBAL", "GLOBL"], Section => ["SECTION"], Macro => ["MACRO"],
    Global => ["GLOBAL"], Section => ["SECTION"], Macro => ["MACRO"],
});