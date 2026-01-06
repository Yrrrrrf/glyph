use glyph::analyze_full_program_struct;

fn main() {
    let test_cases = vec![
        (".stacks segment", "Declaración de segmento inválida"),
        ("MOV AX, 21h", "Constante Hex inválida"),
        ("msg DB \"hello", "Faltan comillas de cierre"),
        ("MOV AX, [BX", "Corchetes desbalanceados"),
    ];

    let mut failed = false;

    for (code, expected) in test_cases {
        let result = analyze_full_program_struct(code);
        let errors = result.errors;

        let found = errors.iter().any(|e| e.contains(expected));

        if found {
            println!("PASS: '{}' -> Found '{}'", code, expected);
        } else {
            println!(
                "FAIL: '{}' -> Expected '{}', Got: {:?}",
                code, expected, errors
            );
            failed = true;
        }
    }

    if failed {
        std::process::exit(1);
    }
}
