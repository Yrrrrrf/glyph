; =============================================================================
;   2x-00-read-param.asm
;   Reads and prints the first (and only) command-line argument passed to the .com file.
;
;   Usage: run-src src/03-cli/00-read-param.asm YourName
;   Output: YourName
;
;   Note: emu2 places command-line args in the standard DOS PSP format.
; =============================================================================
org 0x100           ; .com program origin

section .text
global _start

_start:
    ; Point SI to the start of the command-line buffer (PSP + 0x81)
    mov si, 0x81

    ; Get the command-line length (at 0x80)
    mov cl, [0x80]
    xor ch, ch          ; Clear CH to make CX a proper length
    cmp cx, 0
    je exit             ; If no args, exit silently

    ; Optional: skip leading spaces
.skip_spaces:
    cmp byte [si], ' '
    jne .print_loop
    inc si
    loop .skip_spaces
    jmp exit            ; If all spaces, exit

.print_loop:
    ; Stop at carriage return (0x0D) or if we've printed all chars
    cmp byte [si], 0x0D
    je .newline
    cmp cx, 0
    je .newline

    ; Print current character
    mov al, [si]
    mov ah, 0x0E        ; BIOS teletype output
    mov bx, 0x0007      ; Page 0, white on black
    int 0x10

    inc si
    loop .print_loop

.newline:
    ; Print a newline for clean output
    mov al, 0x0D
    mov ah, 0x0E
    int 0x10
    mov al, 0x0A
    mov ah, 0x0E
    int 0x10

exit:
    ; Terminate via DOS
    mov ah, 0x4C
    int 0x21
