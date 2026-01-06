; =============================================================================
;   2x-03-parse-i16.asm
;   Parses a signed 16-bit integer from the first command-line argument.
;
;   - Handles an optional leading '-' for negative numbers.
;   - Converts the ASCII string to a 16-bit integer.
;   - Prints the decimal representation of the number.
;
;   Usage: nasm -f bin src/03-cli/03-parse-i16.asm -o 03-parse-i16.com
;          ./2x-03-parse-i16.com -123
;   Output: Parsed: -123
; =============================================================================
org 0x100

section .data
    msg_parsed   db 'Parsed: ', 0
    msg_invalid  db 'Invalid or no argument', 0x0D, 0x0A, 0
    newline      db 0x0D, 0x0A, 0
    is_negative  db 0

section .text
global _start

_start:
    ; --- Get command-line argument ---
    mov cl, [0x80]
    xor ch, ch
    jcxz .invalid_input

    mov si, 0x81

    ; --- Skip leading spaces ---
.skip_spaces:
    cmp cx, 0
    je .invalid_input
    cmp byte [si], ' '
    jne .check_sign
    inc si
    dec cx
    jmp .skip_spaces

.check_sign:
    ; --- Check for negative sign ---
    cmp byte [si], '-'
    jne .parse_loop
    mov byte [is_negative], 1
    inc si
    dec cx
    jz .invalid_input ; No numbers after sign

    ; --- Parse the integer string ---
.parse_loop:
    xor ax, ax             ; Parsed number
    xor bx, bx             ; Current digit
    mov di, 10             ; For multiplication

.convert_char:
    cmp cx, 0
    je .done_parsing
    mov bl, [si]
    cmp bl, ' '
    je .done_parsing

    ; --- Convert character to digit ---
    sub bl, '0'
    jc .invalid_input      ; Not a digit
    cmp bl, 9
    ja .invalid_input      ; Not a digit

    ; --- Add to total ---
    mul di                 ; ax = ax * 10
    add ax, bx             ; ax = ax + digit

    inc si
    dec cx
    jmp .convert_char

.done_parsing:
    ; --- Apply sign ---
    cmp byte [is_negative], 1
    jne .print_result
    neg ax                 ; Two's complement for negative

.print_result:
    mov si, msg_parsed
    call PrintString

    call PrintAX_Decimal

    mov si, newline
    call PrintString
    jmp .exit

.invalid_input:
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