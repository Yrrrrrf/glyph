# Assembly Lexer Grammar Guide

## Overview
This document describes the token categories and grammar rules for an x86 Assembly lexer. The lexer recognizes 6 main token categories, with some having subcategories.

---

## Token Categories

### 1. **Instruction** (Instrucción)
CPU-executable operations. These are the actual machine instructions that the processor can execute.

**Subcategories:**

- **Data Transfer**: Move data between registers, memory, and stack
  - Examples: `MOV`, `PUSH`, `POP`, `XCHG`, `LEA`
  
- **Arithmetic**: Mathematical operations
  - Examples: `ADD`, `SUB`, `MUL`, `DIV`, `INC`, `DEC`, `NEG`
  
- **Logic**: Bitwise operations
  - Examples: `AND`, `OR`, `XOR`, `NOT`, `SHL`, `SHR`
  
- **Control Flags**: Manipulate CPU flags
  - Examples: `CLC`, `STC`, `CMC`, `CLD`, `STD`, `CLI`, `STI`
  
- **Control Transfer**: Unconditional jumps and calls
  - Examples: `JMP`, `CALL`, `RET`, `LOOP`
  
- **Conditional Jump**: Conditional branching
  - Examples: `JE`, `JNE`, `JZ`, `JNZ`, `JG`, `JL`, `JGE`, `JLE`, `JA`, `JB`
  
- **Interrupt**: Software interrupts
  - Examples: `INT`, `IRET`

**Grammar Rule:**
```
Instruction := [A-Z]+ (case-insensitive)
```

---

### 2. **Pseudoinstruction** (Pseudoinstrucción / Directiva)
Assembler directives that don't generate machine code but control assembly process.

**Examples:**
- **Segment directives**: `SEGMENT`, `ENDS`, `.DATA`, `.CODE`, `.STACK`
- **Data definition**: `DB`, `DW`, `DD`, `DQ`, `DT` (byte, word, dword, qword, tbyte)
- **Duplication**: `DUP`
- **Constants**: `EQU`
- **Type operators**: `PTR`, `BYTE`, `WORD`, `DWORD`, `QWORD`
- **Program structure**: `END`, `PROC`, `ENDP`, `MACRO`, `ENDM`

**Grammar Rule:**
```
Pseudoinstruction := [A-Z]+ | '.'[A-Z]+ (case-insensitive)
```

---

### 3. **Register** (Registro)
CPU registers for storing data temporarily.

**Categories:**
- **General Purpose (16-bit)**: `AX`, `BX`, `CX`, `DX`
- **General Purpose (8-bit)**: `AL`, `AH`, `BL`, `BH`, `CL`, `CH`, `DL`, `DH`
- **Index/Pointer**: `SI`, `DI`, `BP`, `SP`
- **Segment**: `CS`, `DS`, `ES`, `SS`, `FS`, `GS`
- **Extended (32-bit)**: `EAX`, `EBX`, `ECX`, `EDX`, `ESI`, `EDI`, `EBP`, `ESP`
- **Extended (64-bit)**: `RAX`, `RBX`, `RCX`, `RDX`, `RSI`, `RDI`, `RBP`, `RSP`

**Grammar Rule:**
```
Register := [A-Z]{2,3} (case-insensitive, predefined list)
```

---

### 4. **Symbol** (Símbolo)
User-defined identifiers for labels, variables, and constants.

**Types:**

- **Label Declaration**: Marks a position in code
  ```
  start:
  loop_begin:
  my_function:
  ```
  
- **Label Reference**: Uses a declared label
  ```
  JMP start
  CALL my_function
  ```
  
- **Variable Name**: Named memory location
  ```
  count DB 0
  message DB "Hello"
  ```

**Grammar Rules:**
```
LabelDeclaration := [a-zA-Z_][a-zA-Z0-9_]* ':'
LabelReference   := [a-zA-Z_][a-zA-Z0-9_]*
VariableName     := [a-zA-Z_][a-zA-Z0-9_]*
SectionName      := '.'[a-zA-Z_][a-zA-Z0-9_]*  (e.g., .data, .text)
```

**Rules:**
- Must start with letter or underscore
- Can contain letters, digits, underscores
- Case-sensitive (unlike instructions)
- Cannot be reserved keywords

---

### 5. **Constant** (Constante)
Literal values in different formats.

**Subcategories:**

- **Decimal**: Base-10 numbers
  ```
  123
  456
  0
  ```
  **Regex**: `[0-9]+`

- **Hexadecimal**: Base-16 numbers (suffix: `h` or `H`)
  ```
  0Fh
  1Ah
  FFh
  0A5h
  ```
  **Regex**: `[0-9A-Fa-f]+[Hh]`
  **Note**: Must start with digit (use `0` prefix if needed: `0Ah` not `Ah`)

- **Binary**: Base-2 numbers (suffix: `b` or `B`)
  ```
  1010b
  11110000b
  1b
  ```
  **Regex**: `[01]+[Bb]`

- **String**: Text enclosed in double quotes
  ```
  "Hello, World!"
  "Press any key..."
  ""  (empty string)
  ```
  **Regex**: `"[^"]*"`
  **Note**: Each character stored as ASCII byte value

**Grammar Rules:**
```
ConstantDec := [0-9]+
ConstantHex := [0-9][0-9A-Fa-f]*[Hh]
ConstantBin := [01]+[Bb]
ConstantStr := '"' [^"]* '"'
```

---

### 6. **Invalid**
Any token that doesn't match the above categories.

**Examples:**
- `@invalid`
- `123abc` (without proper suffix)
- `#symbol`
- Malformed strings: `"unclosed`

---

## Special Syntax Elements

### Punctuation/Operators
- **Comma** `,` - Separates operands
- **Left Bracket** `[` - Memory addressing start
- **Right Bracket** `]` - Memory addressing end
- **Plus** `+` - Address arithmetic
- **Minus** `-` - Address arithmetic

### Comments
- **Line Comment**: `;` to end of line
  ```assembly
  MOV AX, 5  ; This is a comment
  ; This entire line is a comment
  ```

### Whitespace
- Spaces, tabs, newlines, form feeds
- Ignored by lexer (not part of tokens)

---

## Example Program Tokenization

```assembly
.data
    message DB "Hello", 0
    count   DW 10

.code
start:
    MOV AX, count
    ADD AX, 5
    INT 21h
```

**Tokens:**
1. `Pseudoinstruction(.data)`
2. `Symbol(message)`
3. `Pseudoinstruction(DB)`
4. `Constant("Hello")` - String
5. `Comma`
6. `Constant(0)` - Decimal
7. `Symbol(count)`
8. `Pseudoinstruction(DW)`
9. `Constant(10)` - Decimal
10. `Pseudoinstruction(.code)`
11. `Symbol(start)` - Label declaration
12. `Instruction(MOV)` - Data Transfer
13. `Register(AX)`
14. `Comma`
15. `Symbol(count)` - Label reference
16. `Instruction(ADD)` - Arithmetic
17. `Register(AX)`
18. `Comma`
19. `Constant(5)` - Decimal
20. `Instruction(INT)` - Interrupt
21. `Constant(21h)` - Hexadecimal

---

## Priority Rules

When multiple patterns could match, the lexer follows these priorities:

1. **Longest match first** (greedy matching)
2. **Keywords before identifiers**
   - `MOV` is `Instruction`, not `Symbol`
   - `DB` is `Pseudoinstruction`, not `Symbol`
3. **Specific patterns before general**
   - `0Fh` is `ConstantHex` before checking `ConstantDec`
   - `start:` is label declaration before checking identifier
4. **Case-insensitive for keywords**
   - `MOV` = `mov` = `MoV` (all are `Instruction`)
   - But symbols are case-sensitive: `Start` ≠ `start`

---

## Common Patterns

### Memory Addressing
```assembly
MOV AX, [BX]           ; Register indirect
MOV AX, [BX + SI]      ; Base + Index
MOV AX, [BX + 10]      ; Base + Displacement
MOV BYTE PTR [BX], 5   ; Type override
```

**Tokens**: `Instruction`, `Register`, `Comma`, `LeftBracket`, `Register`, `Plus`, `Constant`, `RightBracket`

### Data Definition
```assembly
array DB 10 DUP(0)     ; 10 bytes of 0
value DW 1234h         ; Word with hex value
text  DB "Hello", 0    ; String with null terminator
```

---

## Error Detection

The lexer marks as `Invalid` tokens that:
- Start with invalid characters (`@`, `#`, `$` unless in specific contexts)
- Have malformed number formats (`0xAB` instead of `0ABh`)
- Unclosed strings
- Invalid character sequences

**Next Step**: The parser validates token sequences according to assembly syntax rules.