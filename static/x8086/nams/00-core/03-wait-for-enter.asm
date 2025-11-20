; =============================================================================
;   02-wait-for-enter.asm (Revised for immediate prompt display)
;   An atomic script that acts as a gatekeeper:
;   1. Prints a message to the screen asking the user to press Enter.
;      (Uses BIOS INT 10h for immediate display)
;   2. Enters a loop, waiting for keyboard input.
;   3. Ignores any key that is NOT the Enter key.
;   4. Exits the program only when the Enter key is pressed.
; =============================================================================

org 0x100           ; Origin for a .com program

section .data
    ; The message to prompt the user.
    ; NOTE: For BIOS INT 10h character-by-character output,
    ; the string MUST end with a null byte (0).
    prompt_msg db 'Press Enter to continue...', 0

section .text
global _start

_start:
    ; --- Step 1: Print the prompt message using BIOS INT 10h (character by character) ---
    ; This ensures the message appears immediately before any input is requested.
    mov si, prompt_msg ; SI points to the start of our message.

print_prompt_loop:
    mov al, [si]       ; Load the current character into AL.
    cmp al, 0          ; Compare character with null (end of string).
    je end_print_prompt ; If it's null, jump out of the print loop.

    ; If not null, print it using BIOS Teletype Output.
    mov ah, 0x0E       ; Function 0Eh of INT 10h: "Teletype output".
    mov bh, 0x00       ; Video page number (usually 0).
    int 0x10           ; Call BIOS video interrupt.

    inc si             ; Move pointer to the next character.
    jmp print_prompt_loop ; Repeat the print loop.

end_print_prompt:
    ; Optionally, print a newline after the prompt for better formatting
    ; mov al, 0x0D ; Carriage Return
    ; mov ah, 0x0E
    ; int 0x10
    ; mov al, 0x0A ; Line Feed
    ; mov ah, 0x0E
    ; int 0x10

; --- Step 2: Begin a loop to wait for the correct key ---
wait_loop:
    ; Wait for any key press.
    mov ah, 0x00
    int 0x16           ; BIOS waits for a key, returns it in AL.

    ; Check if the key pressed was 'Enter'.
    ; The ASCII code for the Enter key (Carriage Return) is 0x0D.
    cmp al, 0x0D       ; Compare the character in AL with 0x0D.

    ; Make a decision based on the comparison.
    je exit_program    ; 'je' means "Jump if Equal". If the key was Enter,
                       ; jump to the exit_program label.

    ; If the key was NOT Enter, the 'je' instruction does nothing.
    ; The program continues to the next line, which loops back.
    jmp wait_loop      ; 'jmp' means "Jump". Go back to the wait_loop label
                       ; to wait for the next key press.

; --- Step 3: Exit the program gracefully ---
; This label is the target for our 'je' jump. We only get here
; when the user has pressed the Enter key.
exit_program:
    mov ah, 0x4C
    int 0x21           ; Call DOS service to terminate the program.
