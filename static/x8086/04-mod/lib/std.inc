; =============================================================================
;   lib.inc
;   A library of common procedures.
; =============================================================================

; --- Text/Code Section for this Library ---
section .text

; -----------------------------------------------------------------------------
; PrintString: A basic, null-terminated string printing procedure.
; Input: SI = pointer to string
; -----------------------------------------------------------------------------
PrintString:
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
