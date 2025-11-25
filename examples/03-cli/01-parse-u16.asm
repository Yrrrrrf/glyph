; =============================================================================
;   2x-01-parse-u16.asm
;   Parses the first command-line argument as an unsigned decimal integer.
;
;   - Skips leading spaces.
;   - Converts digits to a 16-bit unsigned integer in AX.
;   - Prints the parsed number back in decimal for confirmation.
;
;   Usage: run-src src/03-cli/01-parse-u16.asm 65535
;   Output: Parsed: 65535
;
;   Note: Only the first argument is processed. Non-numeric input prints an error.
; =============================================================================
org 0x100
section .data
    msg_parsed   db 'Parsed: ', 0
    msg_invalid  db 'Invalid integer', 0x0D, 0x0A, 0
    newline      db 0x0D, 0x0A, 0

section .text
global _start
_start:
    ; --- Step 1: Access command-line arguments via PSP ---
    ; The length of the command line is at 0x80 (1 byte)
    mov cl, [0x80]
    xor ch, ch          ; Clear CH to make CX a full 16-bit counter
    jcxz .invalid       ; If no args, fail

    ; SI points to the start of the command-line buffer (PSP + 0x81)
    mov si, 0x81

    ; --- Step 2: Skip leading spaces ---
.skip_spaces:
    cmp byte [si], ' '
    jne .start_parse
    inc si
    loop .skip_spaces
    jmp .invalid        ; Only spaces? Invalid.

    ; --- Step 3: Parse digits into AX ---
.start_parse:
    xor ax, ax          ; Clear accumulator (AX = 0)

.parse_loop:
    ; Stop if no more characters
    cmp cx, 0
    je .done_parsing

    ; Check if current char is a digit ('0' to '9')
    mov dl, [si]
    cmp dl, '0'
    jb .invalid
    cmp dl, '9'
    ja .invalid

    ; --- Multiply AX by 10 (to shift digits left) ---
    ; Use 16-bit multiplication to avoid overflow issues
    push bx             ; Save BX (used for the multiplier)
    mov bx, 10          ; Multiplier
    mul bx              ; DX:AX = AX * 10
    ; For inputs <= 65535, AX*10 can overflow. Check DX.
    cmp dx, 0
    jne .invalid        ; Overflow
    pop bx              ; Restore BX

    ; --- Add current digit ---
    mov dl, [si]        ; Load the character again
    sub dl, '0'         ; Convert ASCII digit to numeric value (0-9)
    xor dh, dh          ; Clear DH to make DX a 16-bit value (0-9)
    add ax, dx          ; Add the digit to the accumulated value
    jc .invalid         ; Overflow

    ; Advance
    inc si
    dec cx
    jmp .parse_loop

.done_parsing:
.print_result:
    ; Print "Parsed: "
    mov si, msg_parsed
    call PrintString

    ; Print the number in AX as decimal
    call PrintAX_Decimal

    ; Newline
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
;   PROCEDURES (copied/adapted from 0x-change_base.asm)
; =============================================================================

; -----------------------------------------------------------------------------
; PrintString: Prints a null-terminated string.
; Input: SI = pointer to string
; -----------------------------------------------------------------------------
PrintString:
    push ax
.print_loop:
    mov al, [si]
    cmp al, 0
    je .done
    mov ah, 0x0E        ; BIOS teletype output
    int 0x10
    inc si
    jmp .print_loop
.done:
    pop ax
    ret

; -----------------------------------------------------------------------------
; PrintAX_Decimal: Prints the unsigned 16-bit integer in AX as decimal.
; -----------------------------------------------------------------------------
PrintAX_Decimal:
    push ax
    push bx
    push cx
    push dx

.positive:
    mov cx, 10
    xor bx, bx          ; Digit counter

    ; Extract digits (least significant first) and push to stack
.extract_loop:
    cmp ax, 0
    je .print_digits
    xor dx, dx
    div cx              ; AX = AX / 10, DX = remainder
    add dl, '0'
    push dx
    inc bx
    jmp .extract_loop

.print_digits:
    cmp bx, 0
    jne .pop_loop
    ; Number was zero
    mov al, '0'
    mov ah, 0x0E
    int 0x10
    jmp .done_print

.pop_loop:
    pop ax
    mov ah, 0x0E
    int 0x10
    dec bx
    jnz .pop_loop

.done_print:
    pop dx
    pop cx
    pop bx
    pop ax
    ret