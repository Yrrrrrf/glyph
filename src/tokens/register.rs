//! CPU Registers
use super::*;
use serde::Serialize;

define_tokens!(Register {
    // 16-bit General Purpose
    Ax => ["AX"], Bx => ["BX"], Cx => ["CX"], Dx => ["DX"],
    // 8-bit
    Al => ["AL"], Ah => ["AH"], Bl => ["BL"], Bh => ["BH"],
    Cl => ["CL"], Ch => ["CH"], Dl => ["DL"], Dh => ["DH"],
    // Index/Pointer
    Si => ["SI"], Di => ["DI"], Bp => ["BP"], Sp => ["SP"],
    // Segment
    Cs => ["CS"], Ds => ["DS"], Es => ["ES"], Ss => ["SS"],
    // 32-bit Extended (partial list for brevity)
    Eax => ["EAX"], Ebx => ["EBX"], Ecx => ["ECX"], Edx => ["EDX"],
    // 64-bit (partial)
    Rax => ["RAX"], Rbx => ["RBX"],
});
