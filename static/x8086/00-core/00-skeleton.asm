; =============================================================================
;   0x-00-skeleton.asm
;   A minimal, fully-commented boilerplate for an 8086 .com program.
;
;   This program does nothing except load and then immediately exit cleanly.
; =============================================================================

; --- Assembler Directives (Commands for NASM) ---

org 0x100           ; DIRECTIVE: "Origin". Tells NASM to assume this code will be
                    ; loaded at memory address 0x100. This is a mandatory
                    ; requirement for all .com files.

section .text       ; DIRECTIVE: Declares the start of the "text" section,
                    ; which by convention contains all the executable code. This
                    ; is a fundamental best practice for organizing a program.

global _start       ; DIRECTIVE: Makes the "_start" label visible to tools outside
                    ; this file (like a linker). While redundant for simple .com
                    ; files, it is an essential habit for all other programming.


; --- Program Entry Point ---

_start:             ; LABEL: Marks the official entry point of the program.
                    ; Execution begins at the instruction immediately following
                    ; this label.


; --- CPU Instructions (The actual program logic) ---

    ; To terminate the program, we must call the DOS operating system.
    ; This is a two-step process:
    ; 1. Tell DOS *what* we want to do.
    ; 2. Call DOS to do it.

    mov ah, 0x4C      ; MNEMONIC: "Move". This instruction copies the value 0x4C
                    ; into the 'ah' register. The value 0x4C is the specific
                    ; function number for "Terminate Program".

    int 0x21          ; MNEMONIC: "Interrupt". This instruction pauses the program
                    ; and calls the operating system. The value 0x21 is the
                    ; interrupt number for general DOS services. Because 'ah'
                    ; now holds 0x4C, DOS knows to execute the "Terminate
                    ; Program" service, and the program ends cleanly.
