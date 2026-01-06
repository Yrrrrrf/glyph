; =============================================================================
;   4x-fn-proc.inc
;   A library file containing a procedure to be imported.
;   NOTE: This is NOT a standalone program. It has no 'org' or '_start'.
; =============================================================================

; --- Data Section for this Library ---
; By convention, place the data needed by the procedures right here.
section .data
    imported_message db "I'm from file 4x-fn-proc.inc", 0x0D, 0x0A, 0

; --- Text/Code Section for this Library ---
section .text

; -----------------------------------------------------------------------------
; PrintExternalMessage: Prints a hardcoded message defined in this file.
; -----------------------------------------------------------------------------
PrintExternalMessage:
    ; Point SI to the message defined within this included file.
    mov si, imported_message
    ; Call the helper procedure (also in this file) to print it.
    call PrintString_Helper
    ret

; -----------------------------------------------------------------------------
; PrintString_Helper: A basic, null-terminated string printing procedure.
; We include it here so our library is self-contained.
; Input: SI = pointer to string
; -----------------------------------------------------------------------------
PrintString_Helper:
    push ax
.loop:
    mov al, [si]
    cmp al, 0
    je .done
    mov ah, 0x0E        ; BIOS teletype output
    int 0x10
    inc si
    jmp .loop
.done:
    pop ax
    ret
