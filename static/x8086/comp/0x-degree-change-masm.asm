;----------------------------------------------------------
; Conversor de Ángulos en EMU8086    
; Convierte entre Grados ? Radianes ? Centesimales
; Usa punto fijo (3 decimales) multiplicando todo ×1000
; p ˜ 3.1416 ? constante 31416 (escalada ×10000) 
;
; Integrantes de Equipo: 
;
; John Alexander Martinez Garcia
; Fernando Bryan Reza Campos      
; Jesus Naresh Suarez Gonzalez
;----------------------------------------------------------

DATA SEGMENT
    msgMenu     DB 'Seleccione la unidad de entrada:',0Dh,0Ah
                DB '1. Grados',0Dh,0Ah
                DB '2. Radianes',0Dh,0Ah
                DB '3. Centesimales',0Dh,0Ah,'$'

    msgEntrada  DB 0Dh,0Ah,'Ingrese el valor del angulo (ej. 45.5): $'
    msgGrados   DB 0Dh,0Ah,'Grados: $'
    msgRad      DB 0Dh,0Ah,'Radianes: $'
    msgCent     DB 0Dh,0Ah,'Centesimales: $'

    buffer      DB 20,?,20 DUP(0)   ; buffer para lectura con INT 21h/0Ah
    valor       DW ?                ; valor en punto fijo ×1000

    grados      DW ?
    radianes    DW ?
    centes      DW ?

    pi          DW 31416            ; p ×10000
DATA ENDS

CODE SEGMENT
ASSUME CS:CODE, DS:DATA

START:
    ; Inicializar DS
    MOV AX, DATA
    MOV DS, AX

    ; Mostrar menú
    LEA DX, msgMenu
    MOV AH, 9
    INT 21h

    ; Leer opción (1-3)
    MOV AH, 1
    INT 21h
    SUB AL, '0'      ; de ASCII a número
    MOV BL, AL       ; guardar opción

    ; Pedir valor
    LEA DX, msgEntrada
    MOV AH, 9
    INT 21h

    ; Leer número como cadena
    LEA DX, buffer
    MOV AH, 0Ah
    INT 21h

    ; Convertir cadena a entero ×1000
    CALL Cadena_A_Num

    ; Guardar en AX el valor convertido
    MOV valor, AX

    ;-----------------------------------------------
    ; Normalizar a grados según la opción elegida
    ;-----------------------------------------------
    CMP BL, 1
    JE INPUT_GRADOS
    CMP BL, 2
    JE INPUT_RAD
    CMP BL, 3
    JE INPUT_CENT

INPUT_GRADOS:
    MOV AX, valor
    MOV grados, AX
    JMP CALCULAR

INPUT_RAD:
    ; grados = rad * 180000 / p
    MOV AX, valor
    MOV BX, 180      ; Usar 180 y luego multiplicar por 1000
    MUL BX          ; DX:AX = rad * 180
    MOV BX, 1000    ; Multiplicar por 1000 para el escalado
    MUL BX          ; DX:AX = rad * 180000
    DIV pi          ; AX = resultado
    MOV grados, AX
    JMP CALCULAR

INPUT_CENT:
    ; grados = cent * 9 / 10
    MOV AX, valor
    MOV BX, 9
    MUL BX
    MOV BX, 10
    DIV BX
    MOV grados, AX
    JMP CALCULAR

;-----------------------------------------------
; Cálculos de radianes y centesimales
;-----------------------------------------------
CALCULAR:
    ; rad = grados * p / 180000
    MOV AX, grados
    MOV BX, pi
    MUL BX
    MOV BX, 180     ; Dividir en dos pasos
    DIV BX
    MOV BX, 1000    ; Segundo paso de división
    DIV BX
    MOV radianes, AX

    ; cent = grados * 10 / 9
    MOV AX, grados
    MOV BX, 10
    MUL BX
    MOV BX, 9
    DIV BX
    MOV centes, AX

;-----------------------------------------------
; Mostrar resultados
;-----------------------------------------------
    ; Grados
    LEA DX, msgGrados
    MOV AH, 9
    INT 21h
    MOV AX, grados
    CALL Imprimir_Numero

    ; Radianes
    LEA DX, msgRad
    MOV AH, 9
    INT 21h
    MOV AX, radianes
    CALL Imprimir_Numero

    ; Centesimales
    LEA DX, msgCent
    MOV AH, 9
    INT 21h
    MOV AX, centes
    CALL Imprimir_Numero

    ; Salir
    MOV AH, 4Ch
    INT 21h

;-----------------------------------------------
; SUBRUTINAS
;-----------------------------------------------

; Convierte cadena (en buffer) a número ×1000
; Entrada: buffer con "45.5"
; Salida: AX = 45500
Cadena_A_Num PROC
    XOR AX, AX
    XOR CX, CX
    LEA SI, buffer+2  ; salto a datos reales
NEXT_CHAR:
    MOV AL, [SI]
    INC SI
    CMP AL, 0Dh
    JE FIN_CONV
    CMP AL, '.'
    JE DECIMALS
    SUB AL, '0'
    MOV BL, AL
    MOV AX, CX
    MOV DX, 10
    MUL DX
    ADD AX, BX
    MOV CX, AX
    JMP NEXT_CHAR
DECIMALS:
    MOV AL, [SI]
    INC SI
    SUB AL, '0'
    MOV BX, AX
    MOV AX, CX
    MOV DX, 100
    MUL DX
    MOV CX, 100
    MOV AX, BX
    MUL CX
    ADD AX, BX
FIN_CONV:
    MOV AX, CX
    MOV BX, 1000
    MUL BX
    RET
Cadena_A_Num ENDP

; Imprimir número en formato entero.decimal (3 dígitos)
; Entrada: AX = valor ×1000
Imprimir_Numero PROC
    ; Separar parte entera y decimal
    MOV BX, 1000
    XOR DX, DX
    DIV BX        ; AX = entero, DX = decimales
    PUSH DX       ; guardar decimales
    ; imprimir parte entera
    CALL Imprimir_Entero
    ; imprimir punto
    MOV DL, '.'
    MOV AH, 2
    INT 21h
    ; imprimir decimales (3 cifras, con ceros)
    POP AX
    MOV CX, 3
DEC_LOOP:
    MOV BX, 10
    XOR DX, DX
    DIV BX
    PUSH DX
    LOOP DEC_LOOP
    MOV CX, 3
PRINT_DEC:
    POP DX
    ADD DL, '0'
    MOV AH, 2
    INT 21h
    LOOP PRINT_DEC
    RET
Imprimir_Numero ENDP

; Imprimir entero positivo (AX)
Imprimir_Entero PROC
    XOR CX, CX
    MOV BX, 10
ENTR1:
    XOR DX, DX
    DIV BX
    PUSH DX
    INC CX
    OR AX, AX
    JNZ ENTR1
ENTR2:
    POP DX
    ADD DL, '0'
    MOV AH, 2
    INT 21h
    LOOP ENTR2
    RET
Imprimir_Entero ENDP

CODE ENDS
END START