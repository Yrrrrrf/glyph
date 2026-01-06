; =============================================================================
;   Conversor de Ángulos Definitivo (Versión Corregida) - NASM
;
;   Este programa convierte un valor fijo en grados a sus equivalentes en
;   Radianes y Gradianes.
; =============================================================================

org 0x100

section .data
    ; --- Parámetros de Entrada ---
    grados_input    dw 45           ; El ángulo en grados que queremos convertir.

    ; --- Constantes para Punto Fijo ---
    FACTOR          dw 10000        ; Factor de escala para 4 decimales de precisión.
    PI_ESCALADO     dw 31416        ; π * FACTOR (3.1416 * 10000)

    ; --- Mensajes para la UI ---
    msg_grados      db 'Grados:    ', 0
    msg_radianes    db 'Radianes:  ', 0
    msg_gradianes   db 'Gradianes: ', 0
    msg_punto       db '.', 0
    newline         db 0x0D, 0x0A, 0

section .text
global _start

_start:
    ; --- 1. Imprimir el valor de entrada ---
    mov si, msg_grados
    call PrintString
    mov ax, [grados_input]
    call PrintAX_Decimal
    mov si, newline
    call PrintString

    ; ===========================================================
    ; --- 2. CONVERSIÓN A RADIANES ---
    ; ===========================================================
    mov ax, [grados_input]
    mov bx, [PI_ESCALADO]
    mul bx
    mov bx, 180
    div bx
    mov si, msg_radianes
    call PrintString
    call PrintFixedPoint

    mov si, newline
    call PrintString

    ; ===========================================================
    ; --- 3. CONVERSIÓN A GRADIANES (Lógica a prueba de overflow) ---
    ; ===========================================================
    ; Calculamos la parte entera y la fraccionaria por separado.
    ; Parte entera = (grados * 10) / 9
    mov ax, [grados_input]
    mov bx, 10
    mul bx
    mov dx, 0 ; Limpiamos DX porque el resultado de mul cabe en AX
    mov bx, 9
    div bx                      ; AX = parte entera, DX = residuo para la parte fraccionaria

    ; Guardamos la parte entera para más tarde
    push ax
    ; Guardamos el residuo para calcular la fracción
    push dx

    ; Imprimir el texto "Gradianes: "
    mov si, msg_gradianes
    call PrintString

    ; Recuperamos e imprimimos la parte entera
    pop dx  ; Residuo (aún no lo usamos)
    pop ax  ; Parte entera
    call PrintAX_Decimal

    ; Imprimir el punto decimal
    mov si, msg_punto
    call PrintString

    ; Calcular e imprimir la parte fraccionaria
    ; Fracción = (residuo * FACTOR) / 9
    mov ax, dx ; Movemos el residuo a AX
    mov bx, [FACTOR]
    mul bx
    mov bx, 9
    div bx
    call PrintAX_Decimal

    ; --- 4. Salir del programa ---
    mov si, newline
    call PrintString
    mov ah, 0x4C
    int 0x21

; =============================================================================
;   PROCEDIMIENTOS
; =============================================================================

PrintFixedPoint:
    push ax
    push bx
    push dx
    mov bx, [FACTOR]
    div bx
    call PrintAX_Decimal
    mov si, msg_punto
    call PrintString
    mov ax, dx
    call PrintAX_Decimal
    pop dx
    pop bx
    pop ax
    ret

PrintString:
    push ax
.loop:
    mov al, [si]
    cmp al, 0
    je .done
    mov ah, 0x0E
    int 0x10
    inc si
    jmp .loop
.done:
    pop ax
    ret

; -----------------------------------------------------------------------------
; PrintAX_Decimal (CORREGIDO): Imprime el valor de AX como un número decimal.
; -----------------------------------------------------------------------------
PrintAX_Decimal:
    push bx
    push cx
    push dx

    ; Si el número es 0, imprimir '0' y salir para evitar un bucle problemático.
    cmp ax, 0
    jne .start_division
    mov al, '0'
    mov ah, 0x0E
    int 0x10
    jmp .done_printing

.start_division:
    mov cx, 0
    mov bx, 10
.divide_loop:
    xor dx, dx
    div bx
    push dx
    inc cx
    cmp ax, 0
    jne .divide_loop

.print_loop:
    pop dx
    add dl, '0'
    mov ah, 0x0E
    mov al, dl
    int 0x10
    loop .print_loop

.done_printing:
    pop dx
    pop cx
    pop bx
    ret
