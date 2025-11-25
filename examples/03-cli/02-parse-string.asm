; =============================================================================
;   2x-02-parse-str.asm
;   Parses the first command-line argument as a string.
;
;   - Skips leading spaces.
;   - Extracts characters until a space or the end of the line.
;   - Prints the parsed string.
;
;   Usage: run-src src/03-cli/02-parse-string.asm hello
;   Output: Parsed: hello
;
;   Note: Only the first argument is processed.
; =============================================================================
org 0x100
section .data
    msg_parsed   db 'Parsed: ', 0
    msg_invalid  db 'No argument provided', 0x0D, 0x0A, 0
    newline      db 0x0D, 0x0A, 0

section .bss
    parsed_str   resb 256

section .text
global _start
_start:
    ; --- Step 1: Access command-line arguments via PSP ---
    mov cl, [0x80]
    xor ch, ch
    jcxz .invalid

    mov si, 0x81

    ; --- Step 2: Skip leading spaces ---
.skip_spaces:
    cmp cx, 0
    je .invalid
    cmp byte [si], ' '
    jne .start_extraction
    inc si
    dec cx
    jmp .skip_spaces

    ; --- Step 3: Extract string ---
.start_extraction:
    mov di, parsed_str
.extract_loop:
    cmp cx, 0
    je .done_parsing
    mov al, [si]
    cmp al, ' '
    je .done_parsing
    mov [di], al
    inc di
    inc si
    dec cx
    jmp .extract_loop

.done_parsing:
    ; Check if any characters were extracted
    cmp di, parsed_str
    je .invalid

    mov byte [di], 0 ; Null-terminate the string

    ; --- Step 4: Print result ---
    mov si, msg_parsed
    call PrintString

    mov si, parsed_str
    call PrintString

    mov si, newline
    call PrintString

    jmp .exit

.invalid:
    mov si, msg_invalid
    call PrintString

.exit:
    mov ah, 0x4C
    int 0x21

; =============================================================================
;   PROCEDURES
; =============================================================================

PrintString:
    push ax
.print_loop:
    mov al, [si]
    cmp al, 0
    je .done
    mov ah, 0x0E
    int 0x10
    inc si
    jmp .print_loop
.done:
    pop ax
    ret