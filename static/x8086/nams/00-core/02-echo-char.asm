; =============================================================================
;   0x-01-echo-char.asm
;   An atomic script that performs a single task with clean output.
;
;   - Waits for the user to press any single key.
;   - Immediately prints (echoes) that same character to the screen.
;   - Prints a newline for clean formatting before exiting.
; =============================================================================

org 0x100           ; Origin for a .com program

section .text
global _start

_start:
    ; --- Step 1: Wait for and read a single key press ---
    ; Set AH to 0x00 for the BIOS "Read Key Stroke" service.
    mov ah, 0x00
    ; Call the BIOS keyboard interrupt. The program will pause here until a key is pressed.
    ; The ASCII code of the key is returned in the AL register.
    int 0x16

    ; --- Step 2: Print (echo) the character back to the user ---
    ; The DOS "Display Character" service requires the character to be in DL.
    ; Move the character we just read from AL into DL.
    mov dl, al
    ; Set AH to 0x02 for the DOS "Display Character" service.
    mov ah, 0x02
    ; Call the DOS interrupt to print the character now stored in DL.
    int 0x21

    ; --- Step 3: Print a newline for clean formatting ---
    ; This consists of two characters: Carriage Return (CR) and Line Feed (LF).

    ; Print Carriage Return (moves cursor to the start of the line).
    mov dl, 0x0D
    mov ah, 0x02
    int 0x21

    ; Print Line Feed (moves cursor down one line).
    mov dl, 0x0A
    mov ah, 0x02
    int 0x21

    ; --- Step 4: Exit the program gracefully ---
    ; Set AH to 0x4C for the DOS "Terminate with Return Code" service.
    mov ah, 0x4C
    ; Call the DOS interrupt to end the program.
    int 0x21
