; =============================================================================
;   2x-04-parse-hex.asm
;   Parses a hexadecimal number from the first command-line argument.
;
;   - Handles an optional leading "0x".
;   - Converts the ASCII string to a 16-bit integer.
;   - Prints the decimal representation of the number.
;
;   Usage: nasm -f bin src/03-cli/04-parse-hex.asm -o 04-parse-hex.com
;          ./2x-03-parse-hex.com 0xFF
;   Output: Parsed: 255
; =============================================================================
org 0x100

section .data
    msg_parsed   db 'Parsed: ', 0
    msg_invalid  db 'Invalid or no argument', 0x0D, 0x0A, 0
    newline      db 0x0D, 0x0A, 0

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
    jne .check_prefix
    inc si
    dec cx
    jmp .skip_spaces

.check_prefix:
    ; --- Check for "0x" prefix ---
    cmp byte [si], '0'
    jne .parse_loop
    cmp byte [si+1], 'x'
    jne .parse_loop
    add si, 2
    sub cx, 2
    jcxz .invalid_input ; No numbers after prefix

    ; --- Parse the integer string ---
.parse_loop:
    xor ax, ax             ; Parsed number
    xor bx, bx             ; Current digit

.convert_char:
    cmp cx, 0
    je .done_parsing
    mov bl, [si]
    cmp bl, ' '
    je .done_parsing

    ; --- Convert character to digit ---
    cmp bl, '0'
    jb .invalid_input
    cmp bl, '9'
    jbe .is_digit

    cmp bl, 'A'
    jb .invalid_input
    cmp bl, 'F'
    jbe .is_uppercase_hex

    cmp bl, 'a'
    jb .invalid_input
    cmp bl, 'f'
    jbe .is_lowercase_hex

    jmp .invalid_input

.is_digit:
    sub bl, '0'
    jmp .add_to_total

.is_uppercase_hex:
    sub bl, 'A'
    add bl, 10
    jmp .add_to_total

.is_lowercase_hex:
    sub bl, 'a'
    add bl, 10

.add_to_total:
    shl ax, 4              ; ax = ax * 16
    add al, bl             ; ax = ax + digit

    inc si
    dec cx
    jmp .convert_char

.done_parsing:
.print_result:
    mov si, msg_parsed
    call PrintString

    call PrintAX_Decimal

    mov si, newline
    call PrintString
    jmp .exit

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

; -----------------------------------------------------------------------------
; PrintAX_Decimal: Prints the signed 16-bit value in AX in decimal.
; -----------------------------------------------------------------------------
PrintAX_Decimal:
    cmp ax, 0
    jge .positive

    ; --- Handle negative ---
    push ax
    mov al, '-'
    mov ah, 0x0E
    int 0x10
    pop ax
    neg ax

.positive:
    xor cx, cx             ; Digit counter
    mov bx, 10             ; Divisor

.divide_loop:
    xor dx, dx             ; Clear upper part of dividend
    div bx                 ; ax = ax / 10, dx = remainder
    push dx                ; Push remainder on stack
    inc cx                 ; Increment digit count
    cmp ax, 0
    jne .divide_loop

.print_digits:
    cmp cx, 0
    je .done_print
    pop dx                 ; Pop digit
    add dl, '0'            ; Convert to ASCII
    mov al, dl
    mov ah, 0x0E
    int 0x10
    dec cx
    jmp .print_digits

.done_print:
    ret
