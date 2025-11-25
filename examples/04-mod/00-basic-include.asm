; =============================================================================
;   3x-00-import.asm
;   Demonstrates importing and calling a procedure from an external file.
; =============================================================================

org 0x100           ; Origin for a .com program

section .text
global _start

_start:
    ; --- Step 1: Call the procedure ---
    ; This procedure is NOT defined in this file. NASM will find it in the
    ; included file below and assemble it as if it were here all along.
    call PrintExternalMessage

    ; --- Step 2: Exit the program gracefully ---
    mov ah, 0x4C
    int 0x21

; =============================================================================
;   -- PREPROCESSOR DIRECTIVE --
;   This is the magic line. NASM will pause, read the entire contents of
;   'fn/print-external-message.inc', and paste them right here before continuing to assemble.
; =============================================================================
%include "lib/io.inc"
