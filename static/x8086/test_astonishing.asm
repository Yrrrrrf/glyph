; =============================================================
; GLYPH ASTONISHING TEST SUITE
; =============================================================
; Objetivo: Verificar diagnósticos forenses y reglas del Equipo 2
; Instrucciones Permitidas: 
; CMC, CMPSB, NOP, POPA, AAD, AAM, MUL, INC, IDIV, INT
; AND, LEA, OR, XOR, JNAE, JNE, JNLE, LOOPE, JA, JC
; =============================================================

; -------------------------------------------------------------
; CASO 1: Errores de Segmento y Pila
; -------------------------------------------------------------
.stacks segment      ; [ESPERADO: PARSER - Declaración de segmento inválida]
.stack segment       ; [ESPERADO: Correcta]
    dw 100 dup(0)    ; [ESPERADO: Correcta]
    dw 100 dup       ; [ESPERADO: PARSER - Formato DUP inválido]
    dw 100 dupy(0)   ; [ESPERADO: PARSER - Sintaxis inválida]
    db 10            ; [ESPERADO: REGLA - 'DB' no permitido en segmento de pila]
ends                 ; [ESPERADO: Correcta]

; -------------------------------------------------------------
; CASO 2: Errores de Datos y Léxicos (Forenses)
; -------------------------------------------------------------
datasegment          ; [ESPERADO: PARSER - Declaración de segmento inválida]
.data segment        ; [ESPERADO: Correcta]
    var1 db 10       ; [ESPERADO: Correcta]
    var2 db 0FFh     ; [ESPERADO: Correcta]
    
    ; -- Diagnóstico Forense: Hexadecimal --
    var3 db FFh      ; [ESPERADO: PARSER - Constante Hex inválida (falta 0 inicial)]
    var4 db 1Ah      ; [ESPERADO: PARSER - Constante Hex inválida (falta 0 inicial)]
    
    ; -- Diagnóstico Forense: Comillas --
    str1 db "Bien"   ; [ESPERADO: Correcta]
    str2 db "Error   ; [ESPERADO: PARSER - Faltan comillas de cierre]
    str3 db 'Mal     ; [ESPERADO: PARSER - Faltan comillas de cierre]
    
    ; -- Errores de Contexto --
    mov ax, 0        ; [ESPERADO: REGLA - Instrucción 'MOV' no permitida en datos]
ends

; -------------------------------------------------------------
; CASO 3: Errores de Código y Equipo 2
; -------------------------------------------------------------
.code segment
inicio:
    ; -- Instrucciones Válidas (Equipo 2) --
    nop              ; [ESPERADO: Correcta]
    xor ax, ax       ; [ESPERADO: Correcta]
    inc ax           ; [ESPERADO: Correcta]
    int 21h          ; [ESPERADO: Correcta]
    
    ; -- Instrucciones Inválidas (Restricción Equipo 2) --
    mov ax, bx       ; [ESPERADO: REGLA - 'MOV' no es una instrucción válida]
    add ax, 1        ; [ESPERADO: REGLA - 'ADD' no es una instrucción válida]
    sub ax, bx       ; [ESPERADO: REGLA - 'SUB' no es una instrucción válida]
    jmp inicio       ; [ESPERADO: REGLA - 'JMP' no es una instrucción válida]
    
    ; -- Errores Forenses dentro de instrucciones --
    and ax, FFh      ; [ESPERADO: PARSER - Constante Hex inválida (falta 0 inicial)]
    or ax, "bad      ; [ESPERADO: PARSER - Faltan comillas de cierre]
    
    ; -- Lógica de Etiquetas --
    ja inicio        ; [ESPERADO: Correcta]
    ja no_existe     ; [ESPERADO: REGLA - Etiqueta 'no_existe' no definida previamente]
    
    ; -- Error de Contexto en Código --
    malo db 10       ; [ESPERADO: REGLA - Declaración de datos no permitida en código]
    
ends
end inicio
