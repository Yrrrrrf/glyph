; =============================================================================
;   01-data/01-addr-value.asm
;   Demonstrates the crucial difference between a variable's VALUE
;   and its memory ADDRESS.
; =============================================================================
;
;   CONCEPT: Every variable in memory has two key properties:
;   1. Its VALUE: The data stored in that memory location (e.g., the number 5).
;   2. Its ADDRESS: The numerical location in memory where the value is stored
;      (e.g., memory cell #278).
;
;   - `mov al, [my_var]`   : The square brackets `[]` are the key. They mean
;                            "dereference". It tells the CPU, "Go to the address
;                            of my_var and get the VALUE stored there."
;
;   - `lea ax, [my_var]`   : `LEA` stands for "Load Effective Address". It tells
;                            the CPU, "Don't get the value. I want the ADDRESS
;                            of my_var itself." The address is then stored in AX.
;
;   - `mov ax, my_var`     : In NASM, this is a convenient shorthand that does the
;                            exact same thing as `lea ax, [my_var]`.

org 0x100

section .data
    my_var   db 'A'             ; A variable holding the character 'A'.
    msg_val  db 'Value: ', 0      ; A label for printing the value.
    msg_addr db 'Address: ', 0   ; A label for printing the address.
    newline  db 0x0D, 0x0A, 0

section .text
global _start

_start:
    ; --- Part 1: Get the VALUE ---
    ; First, print the descriptive string "Value: ".
    mov si, msg_val
    call PrintString

    ; Now, get the content of my_var.
    mov al, [my_var]    ; The brackets [] mean "get the contents at this address".
    mov ah, 0x0E        ; Use BIOS teletype print for a single character.
    int 0x10            ; Prints the character in AL, which is 'A'.

    ; Print a newline to separate the two parts.
    mov si, newline
    call PrintString

    ; --- Part 2: Get the ADDRESS ---
    ; Print the descriptive string "Address: ".
    mov si, msg_addr
    call PrintString

    ; Now, get the memory location of my_var.
    lea ax, [my_var]    ; LEA puts the memory location of my_var into the AX register.
    ; You could also use: mov ax, my_var ; NASM is smart, this does the same thing.

    ; The address is just a number, so we can print it with our decimal print function.
    call PrintAX_Decimal ; Print the numeric address (e.g., 278).

    ; --- Exit ---
    mov ah, 0x4C
    int 0x21

; =============================================================================
;   PROCEDURES (Helper functions)
; =============================================================================

PrintString:
    push ax
.loop:
    mov al, [si]
    cmp al, 0
    je .done
    mov ah, 0x0E
    int 0x10
    inc si
    jmp .loop
.done:
    pop ax
    ret

PrintAX_Decimal:
    push bx
    push cx
    push dx
    xor cx, cx
    mov bx, 10
.divide_loop:
    xor dx, dx
    div bx
    push dx
    inc cx
    cmp ax, 0
    jne .divide_loop
.print_loop:
    pop dx
    add dl, '0'
    mov ah, 0x0E
    mov al, dl
    int 0x10
    loop .print_loop
    pop dx
    pop cx
    pop bx
    ret
