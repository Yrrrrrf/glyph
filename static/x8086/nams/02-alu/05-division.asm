; =============================================================================
;   1x-05-division.asm
;   An atomic script that performs division on two hardcoded single-digit
;   numbers and prints the result in the format "a / b = c".
; =============================================================================

org 0x100           ; Origin for a .com program

section .data
    ; --- Hardcoded numbers ---
    num1 db 7
    num2 db 3

    ; --- Message template ---
    ; '  /   =  ' with space for the numbers and result.
    ; The string is terminated with CR, LF, and '$' for DOS printing.
    msg db '  /   =  ', 0x0D, 0x0A, '$'

section .text
    global _start

_start:
    ; --- Step 1: Place the first number (num1) into the message ---
    mov al, [num1]
    add al, '0'
    mov [msg], al

    ; --- Step 2: Place the second number (num2) into the message ---
    mov al, [num2]
    add al, '0'
    mov [msg + 4], al

    ; --- Step 3: Calculate the quotient ---
    mov al, [num1]
    mov ah, 0           ; clear ah for 16-bit division
    mov bl, [num2]
    div bl              ; ax / bl, quotient in al, remainder in ah

    ; --- Step 4: Convert the quotient to ASCII ---
    add al, '0'         ; convert quotient to ascii

    ; --- Step 5: Place the quotient into the message ---
    mov [msg + 8], al   ; quotient

    ; --- Step 6: Print the complete message to the screen ---
    mov ah, 0x09
    mov dx, msg
    int 0x21

    ; --- Step 7: Exit the program gracefully ---
    mov ah, 0x4C
    int 0x21
