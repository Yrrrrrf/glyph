; =============================================================================
;   Universal Base Converter (Versión Final con Impresión Amigable)
;
;   Este programa convierte un número entero de una base a otra y muestra
;   el resultado de una manera clara y legible.
;
;   CÓMO USARLO:
;   1. Ve a la sección .data al final de este archivo.
;   2. Modifica los valores de 'input_num', 'input_base' y 'output_base'.
;   3. Compila y ejecuta.
; =============================================================================

org 0x100           ; Origen para un programa .com

section .text
global _start


_start:
    ; --- 1. Imprimir la información del número original ---
    mov si, msg_orig
    call PrintString

    mov si, input_num       ; Usar el valor fijo de .data
    call PrintString

    mov si, msg_base_open
    call PrintString

    mov ax, [input_base]    ; Usar el valor fijo de .data
    call PrintAX_Decimal

    mov si, msg_base_close
    call PrintString

    mov si, msg_arrow
    call PrintString

    ; --- 2. Convertir la cadena de entrada a un número binario (en AX) ---
    mov si, input_num
    mov bx, [input_base]
    call StringToDecimal    ; El resultado en decimal quedará en AX

    ; --- 3. Convertir el número de AX a la base de salida ---
    mov bx, [output_base]   ; Usar el valor fijo de .data
    mov di, result_buf
    call DecimalToString    ; Convierte AX a una cadena en la nueva base

    ; --- 4. Imprimir el resultado de la conversión ---
    mov si, msg_conv
    call PrintString

    mov si, result_buf
    call PrintString

    mov si, msg_base_open
    call PrintString

    mov ax, [output_base]
    call PrintAX_Decimal

    mov si, msg_base_close
    call PrintString

    mov si, msg_newline
    call PrintString

    hlt                     ; Detener el procesador. Fin del programa.


; =============================================================================
;   PROCEDIMIENTOS
; =============================================================================

; -----------------------------------------------------------------------------
; PrintString: Imprime una cadena de texto terminada en 0.
; Entrada: SI = Puntero a la cadena.
; -----------------------------------------------------------------------------
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
; StringToDecimal: Convierte una cadena en cualquier base (2-16) a un número binario.
; -----------------------------------------------------------------------------
StringToDecimal:
    push bx
    push cx
    push si
    xor ax, ax          ; ax = 0 (acumulador del resultado)
.loop:
    mov cl, [si]
    cmp cl, 0
    je .done

    ; Convertir caracter ('0'-'9', 'a'-'f', 'A'-'F') a valor numérico
    cmp cl, '9'
    jle .is_digit
    cmp cl, 'F'
    jle .is_upper_hex
    sub cl, 'a' - 10 ; para 'a'-'f'
    jmp .accumulate
.is_upper_hex:
    sub cl, 'A' - 10 ; para 'A'-'F'
    jmp .accumulate
.is_digit:
    sub cl, '0'

.accumulate:
    mul bx              ; ax = ax * base
    xor ch, ch          ; Asegurar que la parte alta de CX es 0
    add ax, cx          ; ax = ax + nuevo_digito
    inc si
    jmp .loop
.done:
    pop si
    pop cx
    pop bx
    ret

; -----------------------------------------------------------------------------
; DecimalToString: Convierte un número en AX a una cadena en cualquier base (2-16).
; -----------------------------------------------------------------------------
DecimalToString:
    push ax
    push bx
    push cx
    push dx
    mov cx, bx          ; Usar CX para la base en la división
    xor bx, bx          ; Usar BX como contador de dígitos
.loop:
    cmp ax, 0
    je .print
    xor dx, dx          ; Limpiar DX antes de dividir
    div cx              ; ax = ax / cx, dx = residuo
    
    ; Convertir residuo (0-15) a caracter
    cmp dx, 9
    jle .is_digit
    add dl, 'A' - 10
    jmp .push_digit
.is_digit:
    add dl, '0'

.push_digit:
    push dx             ; Guardar caracter en la pila
    inc bx              ; Contar un dígito más
    jmp .loop

.print:
    cmp bx, 0           ; Si el contador es 0, el número era 0
    je .no_digits
.pop_loop:
    pop ax              ; Sacar caracter de la pila
    mov [di], al        ; Guardarlo en el buffer de resultado
    inc di
    dec bx
    jnz .pop_loop       ; Repetir hasta que no queden dígitos
    jmp .done

.no_digits:
    mov byte [di], '0'  ; Si el número era 0, poner '0'
    inc di
.done:
    mov byte [di], 0    ; Poner fin de cadena en el buffer
    pop dx
    pop cx
    pop bx
    pop ax
    ret

; -----------------------------------------------------------------------------
; PrintAX_Decimal: Imprime el valor de AX como un número decimal.
; -----------------------------------------------------------------------------
PrintAX_Decimal:
    push ax
    push cx
    push dx
    mov cx, 10
    xor bx, bx
.loop1:
    cmp ax, 0
    je .print1
    xor dx, dx
    div cx
    add dl, '0'
    push dx
    inc bx
    jmp .loop1
.print1:
    cmp bx, 0
    je .no_digits1
.pop_loop1:
    pop ax
    mov ah, 0x0E
    int 0x10
    dec bx
    jnz .pop_loop1
    jmp .done1
.no_digits1:
    mov al, '0'
    mov ah, 0x0E
    int 0x10
.done1:
    pop dx
    pop cx
    pop ax
    ret


; =============================================================================
;   SECCIÓN DE DATOS - ¡MODIFICA LOS VALORES AQUÍ!
; =============================================================================
section .data

    ; --- PARÁMETROS DE CONVERSIÓN ---
    input_num    db 'A3', 0        ; Número de entrada (cadena terminada en 0)
    input_base   dw 16             ; Base de entrada (2-16)
    output_base  dw 10             ; Base de salida (2-16)
    ; ---------------------------------
    ; --- OTROS EJEMPLOS PARA PROBAR:
    ; input_num db '255', 0 / input_base dw 10 / output_base dw 16  -> FF
    ; input_num db '1101', 0 / input_base dw 2 / output_base dw 10   -> 13
    ; input_num db '77', 0 / input_base dw 8 / output_base dw 2      -> 111111
    ; input_num db '1010', 0 / input_base dw 2 / output_base dw 10    -> 10

    ; --- Mensajes para la impresión ---
    msg_orig        db 'Original: ', 0
    msg_conv        db 'Convertido: ', 0
    msg_base_open   db ' (Base ', 0
    msg_base_close  db ')', 0
    msg_arrow       db ' -> ', 0
    msg_newline     db 0x0D, 0x0A, 0

    ; --- Buffer para almacenar la cadena resultado ---
    result_buf      resb 17           ; 16 bits -> máx. 16 dígitos binarios + 1 nulo.
