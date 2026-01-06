; =============================================================================
;   01-data/02-pointers-indirect.asm
;   Demonstrates using a register (BX) as a pointer to access data.
; =============================================================================
;
;   CONCEPT: Instead of accessing data directly by its name (e.g., `[my_var]`),
;   we can first store its memory address in a register. This register is now
;   a "pointer". We can then use this pointer to get the data.
;   This is called "indirect addressing".
;
;   Why is this useful? It allows for dynamic memory access. The program can
;   decide *at runtime* which address to put in the pointer register, allowing
;   it to access different data using the same code.
;
;   - `mov bx, secret_letter` : This gets the ADDRESS of `secret_letter` and
;                               stores it in the `bx` register.
;
;   - `mov al, [bx]`          : This uses the address *in* `bx` to fetch the
;                               VALUE from memory. The CPU first looks at `bx`,
;                               sees the address stored there, then goes to that
;                               address in memory and grabs the data.

org 0x100

section .data
    secret_letter db 'P' ; The data we want to find.

section .text
global _start

_start:
    ; --- Step 1: Create the pointer ---
    ; Get the address of our data and store it in the BX register.
    ; BX will now "point to" the memory location of secret_letter.
    ; BX is a common choice for a pointer (B for Base address).
    mov bx, secret_letter

    ; --- Step 2: Dereference the pointer to get the data ---
    ; The brackets around BX tell the CPU: "Don't give me the value OF bx,
    ; give me the value at the memory address STORED INSIDE bx".
    ; This is the core of indirect addressing.
    mov al, [bx]

    ; --- Step 3: Print the character we found ---
    ; We use the standard BIOS teletype output to prove we got the right character.
    mov ah, 0x0E
    int 0x10            ; Should print 'P'.

    ; --- Exit ---
    mov ah, 0x4C
    int 0x21
