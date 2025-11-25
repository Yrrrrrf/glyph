; =============================================================================
;   1x-03-multiplication.asm
;   An atomic script that multiplies two hardcoded single-digit numbers and
;   prints the result in the format "a * b = cd".
; =============================================================================

org 0x100           ; Origin for a .com program

section .data
    ; --- Hardcoded numbers ---
    num1 db 4
    num2 db 4

    ; --- Message template ---
    ; '  *   =   ' with space for the numbers and result.
    ; The string is terminated with CR, LF, and '$' for DOS printing.
    msg db '  *   =   ', 0x0D, 0x0A, '$'
    ten db 10

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

    ; --- Step 3: Calculate the product ---
    mov al, [num1]
    mov bl, [num2]
    mul bl              ; ax = al * bl

    ; --- Step 4: Convert the product to two ASCII digits ---
    div byte [ten]      ; al = ax / 10, ah = ax % 10

    ; al has the first digit, ah has the second
    add al, '0'         ; convert first digit to ascii
    add ah, '0'         ; convert second digit to ascii

    ; --- Step 5: Place the digits into the message ---
    mov [msg + 8], al   ; first digit
    mov [msg + 9], ah   ; second digit

    ; --- Step 6: Print the complete message to the screen ---
    mov ah, 0x09
    mov dx, msg
    int 0x21

    ; --- Step 7: Exit the program gracefully ---
    mov ah, 0x4C
    int 0x21
