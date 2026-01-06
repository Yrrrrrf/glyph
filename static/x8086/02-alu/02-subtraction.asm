; =============================================================================
;   1x-02-substraction.asm
;   An atomic script that subtracts two hardcoded single-digit numbers and
;   prints the result in the format "a - b = c".
; =============================================================================

org 0x100           ; Origin for a .com program

section .data
    ; --- Hardcoded numbers ---
    num1 db 8
    num2 db 3

    ; --- Message template ---
    ; '  -   =  ' with space for the numbers and result.
    ; The string is terminated with CR, LF, and '$' for DOS printing.
    msg db '  -   =  ', 0x0D, 0x0A, '$'

section .text
    global _start

_start:
    ; --- Step 1: Place the first number (num1) into the message ---
    mov al, [num1]      ; Load the value of num1 into AL.
    add al, '0'         ; Convert the number to its ASCII character representation.
    mov [msg], al       ; Place the ASCII character at the beginning of the message.

    ; --- Step 2: Place the second number (num2) into the message ---
    mov al, [num2]      ; Load the value of num2 into AL.
    add al, '0'         ; Convert to ASCII.
    mov [msg + 4], al   ; Place the character at the 5th position of the message.

    ; --- Step 3: Calculate the difference and place it into the message ---
    mov al, [num1]      ; Load num1 into AL again.
    sub al, [num2]      ; Subtract num2 from it. The difference is now in AL.
    add al, '0'         ; Convert the difference to ASCII.
    mov [msg + 8], al   ; Place the difference character at the 9th position.

    ; --- Step 4: Print the complete message to the screen ---
    ; Use the DOS "Print String" service.
    mov ah, 0x09
    ; DX must point to the start of the $-terminated string.
    mov dx, msg
    int 0x21

    ; --- Step 5: Exit the program gracefully ---
    ; Use the DOS "Terminate with Return Code" service.
    mov ah, 0x4C
    int 0x21
