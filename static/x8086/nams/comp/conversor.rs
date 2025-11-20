use std::io;
use std::f64::consts::PI;

/// Esta es la función principal, equivalente al punto de entrada `START` en el código de ensamblador.
fn main() {
    // --- Equivalente al DATA SEGMENT ---
    // Mensajes para mostrar al usuario.
    let msg_menu = "Seleccione la unidad de entrada:\n1. Grados\n2. Radianes\n3. Centesimales";
    let msg_entrada = "\nIngrese el valor del angulo (ej. 45.5):";
    let msg_grados = "\nGrados:";
    let msg_rad = "\nRadianes:";
    let msg_cent = "\nCentesimales:";

    // --- Inicio de la lógica del CODE SEGMENT ---

    // Muestra el menú de opciones (equivalente a `LEA DX, msgMenu` y `INT 21h`).
    println!("{}", msg_menu);

    // Lee la opción del usuario (equivalente a `MOV AH, 1` y `INT 21h`).
    let mut choice_input = String::new();
    io::stdin()
        .read_line(&mut choice_input)
        .expect("Error al leer la línea.");
    
    // Convierte la entrada de texto a un número.
    let choice: u32 = match choice_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Entrada no válida. Por favor ingrese 1, 2, o 3.");
            return;
        }
    };

    // Muestra el mensaje para ingresar el valor (equivalente a `LEA DX, msgEntrada` y `INT 21h`).
    println!("{}", msg_entrada);

    // Lee el valor del ángulo como texto (equivalente a leer en `buffer`).
    let mut value_input = String::new();
    io::stdin()
        .read_line(&mut value_input)
        .expect("Error al leer la línea.");

    // Convierte la cadena de texto a un número de punto flotante (`f64`).
    // Esto reemplaza la necesidad del procedimiento `Cadena_A_Num`.
    let valor: f64 = match value_input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Valor de ángulo no válido.");
            return;
        }
    };
    
    // El código en ensamblador convierte primero todas las entradas a grados
    // y luego calcula los otros dos valores. Seguiremos la misma lógica.
    let grados_base: f64;

    // Bloque de selección, equivalente a la cadena de `CMP` y `JE`.
    match choice {
        // INPUT_GRADOS: El valor ya está en grados.
        1 => {
            grados_base = valor;
        }
        // INPUT_RAD: Conversión de radianes a grados.
        2 => {
            // La fórmula es: grados = radianes * 180 / PI
            grados_base = valor * 180.0 / PI;
        }
        // INPUT_CENT: Conversión de centesimales (gradianes) a grados.
        3 => {
            // La fórmula es: grados = centesimales * 9 / 10
            grados_base = valor * 9.0 / 10.0;
        }
        // Si la opción no es válida, termina el programa.
        _ => {
            println!("Opción no válida.");
            return;
        }
    }

    // --- Sección CALCULAR ---
    // Una vez que tenemos el valor base en grados, calculamos las otras unidades.
    
    let grados_final = grados_base;

    // Calcular radianes a partir de grados.
    // La fórmula es: radianes = grados * PI / 180
    let radianes_final = grados_base * PI / 180.0;
    
    // Calcular centesimales a partir de grados.
    // La fórmula es: centesimales = grados * 10 / 9
    let centesimales_final = grados_base * 10.0 / 9.0;


    // --- Impresión de Resultados ---
    // Esta sección reemplaza las llamadas a `Imprimir_Numero`.
    // Usamos `print!` y `println!` para un formato similar al original.

    print!("{}", msg_grados);
    println!(" {:.4}", grados_final);

    print!("{}", msg_rad);
    println!(" {:.4}", radianes_final);

    print!("{}", msg_cent);
    println!(" {:.4}", centesimales_final);
    
    // El programa termina aquí, equivalente a `MOV AH, 4Ch` y `INT 21h`.
}
