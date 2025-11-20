; hello.asm
; "Hola, mundo!" escrito para la BIOS, compatible con 8086tiny.
; Imprime el mensaje caracter por caracter usando la interrupción de video.

; Oh no!


org 0x100          ; Origen para un programa .com

section .text
start:
    mov si, msg     ; SI apunta al inicio de nuestro mensaje.

print_loop:
    mov al, [si]    ; Carga el caracter actual en el registro AL.
    cmp al, 0       ; Compara el caracter con 0 (el fin de la cadena).
    je end          ; Si es cero, salta al final.

    ; Si no es cero, lo imprimimos usando la BIOS.
    mov ah, 0x0E    ; Función 0Eh de INT 10h: "Teletype output".
    mov bh, 0x00    ; Número de página de video.
    int 0x10        ; Llama a la interrupción de video de la BIOS.

    inc si          ; Mueve el puntero al siguiente caracter.
    jmp print_loop  ; Repite el bucle.

end:
    hlt             ; HALT. Detiene el procesador. No hay OS al cual "salir".

section .data
; La cadena de texto.
; 0x0D = Retorno de Carro, 0x0A = Nueva Línea.
; Importante: La cadena DEBE terminar con un byte nulo (0) para que nuestro bucle sepa cuándo parar.
msg db 'Hola, BIOS!', 0x0D, 0x0A, 0
