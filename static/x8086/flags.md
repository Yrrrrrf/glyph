---
subject: Ensambladores
---

[[asm/asm-16-bits|asm-16-bits]]

# Tabla Completa del Registro de Banderas (FLAGS) de 16 bits

| Bit | Abreviatura | Nombre                                        | Tipo    | Introducido en | Descripción                                                                                                  |
| :-: | :---------: | :-------------------------------------------- | :------ | :------------: | :----------------------------------------------------------------------------------------------------------- |
|  0  |   **CF**    | Carry Flag (Acarreo)                          | Estado  |      8086      | Se activa (1) si una operación sin signo produce un acarreo o préstamo del bit más significativo.            |
|  1  |      -      | _Reservado_                                   | -       |      8086      | No utilizado en el 8086, siempre con valor 1 en procesadores posteriores.                                    |
|  2  |   **PF**    | Parity Flag (Paridad)                         | Estado  |      8086      | Se activa (1) si el byte menos significativo del resultado contiene un número par de bits '1'.               |
|  3  |      -      | _Reservado_                                   | -       |      8086      | No utilizado.                                                                                                |
|  4  |   **AF**    | Auxiliary Carry Flag (Acarreo Auxiliar)       | Estado  |      8086      | Se activa (1) si hay un acarreo o préstamo entre el bit 3 y el bit 4. Usado para aritmética BCD.             |
|  5  |      -      | _Reservado_                                   | -       |      8086      | No utilizado.                                                                                                |
|  6  |   **ZF**    | Zero Flag (Cero)                              | Estado  |      8086      | Se activa (1) si el resultado de una operación es cero.                                                      |
|  7  |   **SF**    | Sign Flag (Signo)                             | Estado  |      8086      | Se iguala al bit más significativo del resultado (1 para negativo, 0 para positivo).                         |
|  8  |   **TF**    | Trap Flag (Trampa)                            | Control |      8086      | Si se activa (1), el procesador entra en modo de ejecución paso a paso (depuración).                         |
|  9  |   **IF**    | Interrupt Flag (Interrupción)                 | Control |      8086      | Si se activa (1), permite que la CPU responda a interrupciones externas enmascarables.                       |
| 10  |   **DF**    | Direction Flag (Dirección)                    | Control |      8086      | Controla la dirección de las operaciones con cadenas (0=incremento, 1=decremento).                           |
| 11  |   **OF**    | Overflow Flag (Desbordamiento)                | Estado  |      8086      | Se activa (1) si una operación con signo produce un resultado demasiado grande para el destino.              |
| 12  |  **IOPL**   | I/O Privilege Level (Nivel de Privilegio E/S) | Sistema |   **80286**    | Campo de 2 bits (junto con el bit 13) que indica el nivel de privilegio requerido para instrucciones de E/S. |
| 13  |             |                                               |         |   **80286**    | Parte del campo IOPL.                                                                                        |
| 14  |   **NT**    | Nested Task Flag (Tarea Anidada)              | Sistema |   **80286**    | Se activa (1) para indicar que una tarea está anidada dentro de otra en modo protegido.                      |
| 15  |      -      | _Reservado_                                   | -       |      8086      | No utilizado en procesadores Intel.                                                                          |

**Resumen de la Evolución:**

- **Procesador 8086:** Solo utilizaba 9 de los 16 bits (CF, PF, AF, ZF, SF, TF,
  IF, DF, OF). Los otros 7 bits estaban reservados o no tenían una función
  definida.
- **Procesador 80286:** Introdujo el "modo protegido" y dio uso a tres bits
  previamente reservados para crear las banderas **IOPL** (bits 12 y 13) y
  **NT** (bit 14), esenciales para la multitarea y la protección de memoria.
- **Procesador 80386 y Posteriores:** Expandieron este registro a 32 bits
  (EFLAGS) y luego a 64 bits (RFLAGS), añadiendo aún más banderas como VM
  (Virtual 8086 Mode) y RF (Resume Flag) para funcionalidades más avanzadas.
