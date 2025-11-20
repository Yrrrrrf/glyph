; =============================================================================
;   0x-03-echo-str.asm
;   An atomic script that prompts for, reads, and echoes a full string.
;
;   - Prints a prompt message immediately.
;   - Provides a live echo, printing each character as it is typed.
;   - Stops reading when the user presses the Enter key.
;   - Prints a newline, then prints the complete captured string.
;   - Prints a final newline and exits gracefully.
; =============================================================================

org 0x100           ; Origin for a .com program

section .data
    ; A prompt message, terminated with a null byte (0) for BIOS printing.
    prompt_msg db 'Input: ', 0
    ; A reusable newline string, terminated with a '$' for DOS printing.
    newline db 0x0D, 0x0A, '$'

section .bss
    ; Reserve 64 bytes of memory to store the user's input.
    input_buffer resb 64

section .text
global _start

_start:
    ; --- Step 1: Print the prompt message using BIOS ---
    ; This ensures the message appears immediately.
    mov si, prompt_msg ; SI points to the start of our message.
print_prompt_loop:
    mov al, [si]       ; Load the current character into AL.
    cmp al, 0          ; Check if it's the end of the string (null).
    je end_print_prompt ; If yes, exit the print loop.

    mov ah, 0x0E       ; Use BIOS "Teletype Output" function.
    int 0x10           ; Call BIOS video interrupt to print the character.

    inc si             ; Move pointer to the next character.
    jmp print_prompt_loop ; Repeat the loop.

end_print_prompt:
    ; --- Step 2: Prepare to read the string ---
    ; Load the starting address of our buffer into the DI register.
    ; DI will be used as a pointer to the current position in the buffer.
    mov di, input_buffer

; --- Step 3: Start a loop to read characters one by one ---
read_loop:
    ; Wait for a single key press from the user.
    mov ah, 0x00
    int 0x16           ; BIOS waits for a key, returns its ASCII code in AL.

    ; Check if the key pressed was 'Enter' (ASCII 0x0D).
    cmp al, 0x0D
    je end_of_input    ; If it was Enter, jump to the finalization section.

    ; --- Step 4: Provide a live echo of the character ---
    mov dl, al
    mov ah, 0x02
    int 0x21           ; Call DOS to print the character so the user sees it.

    ; --- Step 5: Store the character and advance the pointer ---
    mov [di], al       ; Store the character from AL into the buffer.
    inc di             ; Move the pointer to the next empty byte.

    ; --- Step 6: Repeat the process ---
    jmp read_loop      ; Jump back to the start of the loop for the next character.

; --- Step 7: Finalize and print the captured string ---
end_of_input:
    ; Add a '$' to terminate the string for the DOS print function.
    mov byte [di], '$'

    ; Print a newline to separate the input from the final output.
    mov ah, 0x09
    mov dx, newline
    int 0x21

    ; Print the user's complete, stored string.
    mov ah, 0x09
    mov dx, input_buffer ; DX must point to the start of the $-terminated string.
    int 0x21

    ; Print another newline for clean formatting before the program exits.
    mov ah, 0x09
    mov dx, newline
    int 0x21

    ; --- Step 8: Exit the program ---
    mov ah, 0x4C
    int 0x21           ; Call DOS service to terminate the program.
