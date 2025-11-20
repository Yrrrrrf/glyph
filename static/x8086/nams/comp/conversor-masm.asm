DATA SEGMENT
    msgMenu     DB 'Seleccione la unidad de entrada:',0Dh,0Ah
                DB '1. Grados',0Dh,0Ah
                DB '2. Radianes',0Dh,0Ah
                DB '3. Centesimales',0Dh,0Ah,'$'

    msgEntrada  DB 0Dh,0Ah,'Ingrese el valor del angulo (ej. 45.5): $'
    msgGrados   DB 0Dh,0Ah,'Grados: $'
    msgRad      DB 0Dh,0Ah,'Radianes: $'
    msgCent     DB 0Dh,0Ah,'Centesimales: $'

    buffer      DB 20,?,20 DUP(0)   
    valor       DW ?                

    grados      DW ?
    radianes    DW ?
    centes      DW ?

    pi          DW 31416            
DATA ENDS

CODE SEGMENT
ASSUME CS:CODE, DS:DATA

START:
    
    MOV AX, DATA
    MOV DS, AX

    
    LEA DX, msgMenu
    MOV AH, 9
    INT 21h

    
    MOV AH, 1
    INT 21h
    SUB AL, '0'      
    MOV BL, AL       

    
    LEA DX, msgEntrada
    MOV AH, 9
    INT 21h

    
    LEA DX, buffer
    MOV AH, 0Ah
    INT 21h

    
    CALL Cadena_A_Num

    
    MOV valor, AX

    
    
    
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
    
    MOV AX, valor
    MOV BX, 180      
    MUL BX          
    MOV BX, 1000    
    MUL BX          
    DIV pi          
    MOV grados, AX
    JMP CALCULAR

INPUT_CENT:
    
    MOV AX, valor
    MOV BX, 9
    MUL BX
    MOV BX, 10
    DIV BX
    MOV grados, AX
    JMP CALCULAR
CALCULAR:
    
    MOV AX, grados
    MOV BX, pi
    MUL BX
    MOV BX, 180     
    DIV BX
    MOV BX, 1000    
    DIV BX
    MOV radianes, AX

    
    MOV AX, grados
    MOV BX, 10
    MUL BX
    MOV BX, 9
    DIV BX
    MOV centes, AX
    
    LEA DX, msgGrados
    MOV AH, 9
    INT 21h
    MOV AX, grados
    CALL Imprimir_Numero

    
    LEA DX, msgRad
    MOV AH, 9
    INT 21h
    MOV AX, radianes
    CALL Imprimir_Numero

    
    LEA DX, msgCent
    MOV AH, 9
    INT 21h
    MOV AX, centes
    CALL Imprimir_Numero

    
    MOV AH, 4Ch
    INT 21h
Cadena_A_Num PROC
    XOR AX, AX
    XOR CX, CX
    LEA SI, buffer+2  
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
Cadena_A_Num ENDPImprimir_Numero PROC
    
    MOV BX, 1000
    XOR DX, DX
    DIV BX        
    PUSH DX       
    
    CALL Imprimir_Entero
    
    MOV DL, '.'
    MOV AH, 2
    INT 21h
    
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