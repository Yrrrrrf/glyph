//! Assembler directives
use super::*;
use serde::Serialize;

define_tokens!(Pseudoinstruction {
    Segment => ["SEGMENT"], Ends => ["ENDS"], Db => ["DB"], Dw => ["DW"],
    Dd => ["DD"], Dup => ["DUP"], Equ => ["EQU"], End => ["END"],
    Ptr => ["PTR"], Byte => ["BYTE"], Word => ["WORD"], Dword => ["DWORD"],
    // Common modern directives
    Data => [".DATA"], Code => [".CODE"], Text => [".TEXT"],
    Global => ["GLOBAL"], Section => ["SECTION"], Macro => ["MACRO"],
});
