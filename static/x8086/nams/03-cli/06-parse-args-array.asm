; =============================================================================
;   2x-06-parse-args.asm
;   Tokenizes the full command line into an argc/argv-style array in memory.
;
;   - Parses the command-line string from the PSP.
;   - Replaces spaces with null terminators.
;   - Stores argument pointers in an `argv` array.
;   - Prints the argument count and each argument on a new line.
;
;   Usage: nasm -f bin src/03-cli/06-parse-args-array.asm -o 06-parse-args-array.com
;          ./2x-06-parse-args.com arg1 "arg 2" arg3
;   Output: argc: 3
;           argv[0]: arg1
;           argv[1]: "arg 2"
;           argv[2]: arg3
; =============================================================================
org 0x100

section .data
    msg_argc     db 'argc: ', 0
    msg_argv     db 'argv[', 0
    msg_bracket  db ']: ', 0
    newline      db 0x0D, 0x0A, 0

section .bss
    argc         resw 1
    argv         resw 32 ; Max 32 arguments

section .text
global _start

_start:
    ; --- Get command-line argument ---
    mov cl, [0x80]
    xor ch, ch
    jcxz .no_args

    mov si, 0x81
    mov di, argv
    xor bx, bx ; argc counter

.parse_loop:
    ; Skip leading spaces
.skip_spaces:
    cmp cx, 0
    je .done_parsing
    cmp byte [si], ' '
    jne .start_arg
    inc si
    dec cx
    jmp .skip_spaces

.start_arg:
    ; Store pointer to the argument
    mov [di], si
    add di, 2
    inc bx

    ; Find end of argument
.find_end:
    cmp cx, 0
    je .done_parsing
    cmp byte [si], ' '
    je .end_arg
    inc si
    dec cx
    jmp .find_end

.end_arg:
    ; Null-terminate the argument
    mov byte [si], 0
    inc si
    dec cx
    jmp .parse_loop

.no_args:
.done_parsing:
    mov [argc], bx

    ; --- Print argc ---
    ;mov si, msg_argc
    ;call PrintString
    ;mov ax, [argc]
    ;call PrintAX_Decimal
    ;mov si, newline
    ;call PrintString

    ; --- Print argv ---
    mov cx, [argc]
    jcxz .exit
    mov bp, argv
    xor bx, bx ; loop counter

.print_argv_loop:
    mov si, msg_argv
    call PrintString

    mov ax, bx
    call PrintAX_Decimal

    mov si, msg_bracket
    call PrintString

    mov si, [bp]
    call PrintString

    mov si, newline
    call PrintString

    add bp, 2
    inc bx
    loop .print_argv_loop

.exit:
    mov ah, 0x4C
    int 0x21

; =============================================================================
;   PROCEDURES
; =============================================================================

PrintString:
    push ax
.print_loop:
    mov al, [si]
    cmp al, 0
    je .done_print
    mov ah, 0x0E
    int 0x10
    inc si
    jmp .print_loop
.done_print:
    pop ax
    ret

PrintAX_Decimal:
    push bx
    push cx
    push dx
    cmp ax, 0
    jge .positive

    push ax
    mov al, '-'
    mov ah, 0x0E
    int 0x10
    pop ax
    neg ax

.positive:
    xor cx, cx
    mov bx, 10

.divide_loop:
    xor dx, dx
    div bx
    push dx
    inc cx
    cmp ax, 0
    jne .divide_loop

.print_digits:
    cmp cx, 0
    je .done_print_ax
    pop dx
    add dl, '0'
    mov al, dl
    mov ah, 0x0E
    int 0x10
    dec cx
    jmp .print_digits

.done_print_ax:
    pop dx
    pop cx
    pop bx
    ret
