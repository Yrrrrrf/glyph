; =============================================================================
;   2x-05-parse-bool.asm
;   Parses a boolean flag from the first command-line argument.
;
;   - Handles `true`/`false`, `on`/`off`, `1`/`0`.
;   - Case-insensitive comparison.
;   - Prints the parsed boolean value.
;
;   Usage: nasm -f bin src/03-cli/05-parse-bool.asm -o 05-parse-bool.com
;          ./2x-05-parse-bool.com true
;   Output: Parsed: true
; =============================================================================
org 0x100

section .data
    msg_parsed   db 'Parsed: ', 0
    msg_true     db 'true', 0
    msg_false    db 'false', 0
    msg_invalid  db 'Invalid boolean value', 0x0D, 0x0A, 0
    newline      db 0x0D, 0x0A, 0

section .bss
    input_str    resb 256

section .text
global _start

_start:
    ; --- Get command-line argument ---
    mov cl, [0x80]
    xor ch, ch
    jcxz .invalid_input

    mov si, 0x81
    jmp .main_logic

.invalid_input:
    mov si, msg_invalid
    call PrintString
    jmp .exit

.main_logic:

    ; --- Skip leading spaces ---
.skip_spaces:
    cmp cx, 0
    je .invalid_input
    cmp byte [si], ' '
    jne .extract_str
    inc si
    dec cx
    jmp .skip_spaces

.extract_str:
    ; --- Extract and convert to lowercase ---
    mov di, input_str
.extract_loop:
    cmp cx, 0
    je .done_extracting
    mov al, [si]
    cmp al, ' '
    je .done_extracting

    ; Convert to lowercase
    cmp al, 'A'
    jb .store_char
    cmp al, 'Z'
    ja .store_char
    add al, 32 ; 'a' - 'A'

.store_char:
    mov [di], al
    inc di
    inc si
    dec cx
    jmp .extract_loop

.done_extracting:
    mov byte [di], 0 ; Null-terminate

    ; --- Compare with boolean strings ---
    mov si, input_str

    ; Check for "true"
    mov si, input_str
    mov di, str_true
    call CompareString
    je .is_true

    ; Check for "on"
    mov si, input_str
    mov di, str_on
    call CompareString
    je .is_true

    ; Check for "1"
    mov si, input_str
    mov di, str_1
    call CompareString
    je .is_true

    ; Check for "false"
    mov si, input_str
    mov di, str_false
    call CompareString
    je .is_false

    ; Check for "off"
    mov si, input_str
    mov di, str_off
    call CompareString
    je .is_false

    ; Check for "0"
    mov si, input_str
    mov di, str_0
    call CompareString
    je .is_false

    jmp .invalid_input

.is_true:
    mov si, msg_parsed
    call PrintString
    mov si, msg_true
    call PrintString
    jmp .done

.is_false:
    mov si, msg_parsed
    call PrintString
    mov si, msg_false
    call PrintString
    jmp .done

.done:
    mov si, newline
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
    je .done_print
    mov ah, 0x0E
    int 0x10
    inc si
    jmp .print_loop
.done_print:
    pop ax
    ret

CompareString: ; (si, di) -> zf
    push ax
    push cx
.compare_loop:
    mov al, [si]
    mov cl, [di]
    cmp al, cl
    jne .not_equal
    cmp al, 0
    je .equal ; Both are null
    inc si
    inc di
    jmp .compare_loop
.not_equal:
    pop cx
    pop ax
    ret
.equal:
    pop cx
    pop ax
    cmp al, al ; Set ZF
    ret

; --- String constants for comparison ---
section .data
str_true:  db 'true', 0
str_on:    db 'on', 0
str_1:     db '1', 0
str_false: db 'false', 0
str_off:   db 'off', 0
str_0:     db '0', 0
