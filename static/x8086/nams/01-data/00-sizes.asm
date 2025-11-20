; =============================================================================
;   01-data/00-sizes.asm
;   Demonstrates defining variables of different sizes (byte and word)
;   and loading them into correctly-sized registers.
; =============================================================================
;
;   CONCEPT: Data in memory has a specific size. The most common sizes in
;   16-bit assembly are BYTE (8 bits) and WORD (16 bits). When you move
;   data from memory to a register, the register must be the same size.
;
;   - `db` (Define Byte): Reserves 8 bits of memory.
;   - `dw` (Define Word): Reserves 16 bits of memory.
;
;   - `al`, `bl`, `cl`, `dl`: 8-bit general-purpose registers.
;   - `ax`, `bx`, `cx`, `dx`: 16-bit general-purpose registers.
;
;   You cannot `mov ax, [my_byte]` because you can't fit 8 bits of data
;   into a 16-bit register without ambiguity (the other 8 bits are unknown).
;   You must match the sizes: `mov al, [my_byte]`.

org 0x100

section .data
    ; Here we define our variables in the data section.
    my_byte db 7          ; db = Define Byte (8 bits). Value is 7.
    my_word dw 300        ; dw = Define Word (16 bits). Value is 300.
    newline db 0x0D, 0x0A, 0 ; A standard carriage return and line feed, null-terminated.

section .text
global _start

_start:
    ; --- Part 1: Handle the 8-bit BYTE ---
    ; To print a number, we must first convert it to its ASCII character equivalent.
    ; The number 7 is not the same as the character '7'.
    ; ASCII '0' is value 48, '1' is 49, etc. So, we add 48 ('0') to our number.
    mov al, [my_byte]   ; Move the 8-bit value from my_byte into the 8-bit AL register.
    add al, '0'         ; Convert the number (7) to a printable character ('7').
    mov ah, 0x0E        ; Select BIOS teletype function (print one character).
    int 0x10            ; Call the BIOS interrupt to print the character in AL.

    ; --- Print a newline to separate our outputs ---
    mov si, newline     ; Point SI register to the start of our newline string.
    call PrintString    ; Call the helper procedure to print it.

    ; --- Part 2: Handle the 16-bit WORD ---
    ; Printing a multi-digit number is more complex. We can't just add '0'.
    ; We need to mathematically separate the digits (3, 0, 0) and print them one by one.
    ; This logic is handled by the `PrintAX_Decimal` procedure.
    mov ax, [my_word]   ; Move the 16-bit value from my_word into the 16-bit AX register.
    call PrintAX_Decimal ; Call a procedure to print the full number (300).

    ; --- Part 3: Exit ---
    ; This is the standard DOS exit call.
    mov ah, 0x4C
    int 0x21

; =============================================================================
;   PROCEDURES (Helper functions)
; =============================================================================

; --- PrintString ---
; Prints a null-terminated string.
; Input: SI must point to the beginning of the string.
PrintString:
    push ax             ; Save AX because we use it inside this function.
.loop:
    mov al, [si]        ; Get the character that SI is pointing to.
    cmp al, 0           ; Is it the null terminator (0)?
    je .done            ; If yes, we're done.
    mov ah, 0x0E        ; If no, prepare to print the character.
    int 0x10
    inc si              ; Move pointer to the next character.
    jmp .loop           ; Repeat.
.done:
    pop ax              ; Restore the original value of AX.
    ret                 ; Return from the procedure.

; --- PrintAX_Decimal ---
; Prints the 16-bit number in AX as a decimal string.
; It works by repeatedly dividing the number by 10 and pushing the
; remainder onto the stack. Then it pops the remainders and prints them
; as characters.
PrintAX_Decimal:
    ; Save registers we will modify
    push bx
    push cx
    push dx
    xor cx, cx          ; CX will count how many digits we have.
    mov bx, 10          ; We'll be dividing by 10.
.divide_loop:
    xor dx, dx          ; Clear DX for the division (DX:AX / BX).
    div bx              ; Divide DX:AX by BX. AX gets quotient, DX gets remainder.
    push dx             ; Push the remainder (a digit) onto the stack.
    inc cx              ; Increment our digit counter.
    cmp ax, 0           ; Is the quotient zero?
    jne .divide_loop    ; If not, repeat the division.
.print_loop:
    pop dx              ; Pop a digit off the stack.
    add dl, '0'         ; Convert it to a printable character.
    mov ah, 0x0E        ; Prepare to print.
    mov al, dl
    int 0x10
    loop .print_loop    ; `loop` instruction decrements CX and jumps if not zero.
    ; Restore registers
    pop dx
    pop cx
    pop bx
    ret
