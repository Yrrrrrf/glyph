; =============================================================================
;   01-data/03-iter.asm
;   A practical example of pointers: iterating through a string.
; =============================================================================
;
;   CONCEPT: Pointers are essential for working with sequential data like
;   arrays or strings. A string is just a series of characters laid out one
;   after the other in memory.
;
;   The process is simple:
;   1. Point a register to the first character.
;   2. Process the character (e.g., print it).
;   3. Increment the pointer so it points to the next character.
;   4. Repeat until you reach a designated end-marker (in this case, a null
;      byte, which is the number 0).
;
;   This is the fundamental mechanism behind almost all loops that process
;   data structures in any programming language.

org 0x100

section .data
    ; A C-style string: a sequence of characters ending with a null byte (0).
    my_string db 'Hello!', 0

section .text
global _start

_start:
    ; --- Step 1: Point SI to the start of our string ---
    ; We use SI (Source Index) by convention for the source of a data operation.
    mov si, my_string

print_loop: ; This is the label that marks the top of our loop.

    ; --- Step 2: Get the character that SI is currently pointing at ---
    ; Dereference the SI pointer to get the byte from memory.
    mov al, [si]

    ; --- Step 3: Check if we've reached the end of the string ---
    ; We compare the character we just fetched with 0 (the null terminator).
    cmp al, 0
    je exit_program ; `je` means "Jump if Equal". If the character is 0, we're done.

    ; --- Step 4: If not the end, print the character ---
    ; This is the standard BIOS print character routine.
    mov ah, 0x0E
    int 0x10

    ; --- Step 5: Move our pointer to the next character in memory ---
    ; `inc` adds 1 to the value in SI. Since a character is one byte, this
    ; moves the pointer to the very next byte in memory, which is the next character.
    inc si

    ; --- Step 6: Go back to the top of the loop ---
    ; `jmp` is an unconditional jump. It forces execution back to the `print_loop` label.
    jmp print_loop

exit_program: ; This is the label we jump to when the loop is finished.
    ; --- Standard DOS Exit ---
    mov ah, 0x4C
    int 0x21
