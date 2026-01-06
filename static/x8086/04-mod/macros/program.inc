; =============================================================================
;   macros.inc
;   A library of NASM macros to reduce boilerplate for .com programs.
; =============================================================================

; --- MACRO: program_exit ---
; Description: Generates the standard DOS exit code.
%macro program_exit 0
    mov ah, 0x4C
    int 0x21
%endmacro


; --- MACRO: program_begin ---
; Description: Generates the skeleton for a standard .com program.
%macro program_begin 0
    org 0x100
    section .text
    global _start
_start:
%endmacro

; --- MACRO: program_end ---
; Description: Ends the program by calling the exit procedure.
%macro program_end 0
    program_exit
%endmacro