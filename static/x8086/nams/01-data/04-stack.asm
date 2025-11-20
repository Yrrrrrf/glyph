; =============================================================================
;   01-data/04-stack.asm
;   Demonstrates the Last-In, First-Out (LIFO) nature of the stack.
; =============================================================================
;
;   CONCEPT: The stack is a special area of memory for temporary storage.
;   It works like a stack of plates: you can only add a new plate to the top,
;   and you can only remove the top plate.
;
;   This is called LIFO: Last-In, First-Out.
;
;   - `push <value>`: Takes a 16-bit value and places it on the top of the stack.
;                     The stack pointer (SP) register is automatically updated.
;
;   - `pop <register>`: Takes the 16-bit value from the top of the stack and
;                       puts it into the specified register. The stack pointer
;                       is again automatically updated.
;
;   The stack is crucial for calling procedures (functions) and for temporarily
;   saving the state of registers.

org 0x100

section .text
global _start

_start:
    ; --- Step 1: Load two registers with different, recognizable values ---
    mov ax, 1111
    mov bx, 9999

    ; --- Step 2: PUSH them onto the stack in a specific order ---
    ; Think of putting plates on a physical stack.
    push ax ; Plate 1111 goes on the bottom.
    push bx ; Plate 9999 goes on top of it.

    ; At this point, the stack in memory looks like this (conceptually):
    ; TOP -> [ 9999 ]  (Last one in)
    ;        [ 1111 ]  (First one in)

    ; --- Step 3: Clear the registers ---
    ; We set AX and BX to zero to prove that the `pop` operation is truly
    ; retrieving the values from the stack and not just using what was already there.
    mov ax, 0
    mov bx, 0

    ; --- Step 4: POP them back off the stack ---
    ; You must take the top plate first. The order is reversed.
    pop ax ; AX gets the top value from the stack, which is 9999.
    pop bx ; BX gets the next available value, which is 1111.

    ; --- Step 5: Print the results to see that they have been swapped ---
    ; We print AX first, then BX, to show the LIFO principle in action.
    call PrintAX_Decimal ; Prints the content of AX, which is now 9999.
    mov si, newline
    call PrintString
    mov ax, bx           ; We have to move BX into AX because our print function only works on AX.
    call PrintAX_Decimal ; Prints the content of the original BX, which is 1111.

    ; --- Exit ---
    mov ah, 0x4C
    int 0x21

; =============================================================================
;   PROCEDURES (Helper functions)
; =============================================================================

; We need a data section just for the newline variable for the helpers.
section .data
    newline db 0x0D, 0x0A, 0

; The procedures need to be in a code section.
section .text
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