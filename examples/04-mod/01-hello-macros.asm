; =============================================================================
;   3x-01-hello-macros.asm
;   Demonstrates using macros to eliminate all program boilerplate.
;   The code is cleaner and focuses only on the program's unique logic.
; =============================================================================

; --- Include our libraries first ---
%include "macros/program.inc"
%include "lib/std.inc"


program_begin
    ; This is the only part we had to write.
    ; All the code between 'define_program' and the implicit 'end'
    ; is injected into the program's skeleton.
    mov si, msg
    call PrintString
program_end

; --- Define data needed for this specific program ---
section .data
    msg db 'Hello from a macro-defined program!', 0x0D, 0x0A, 0
