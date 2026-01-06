use glyph::analyze_full_program_struct;

fn main() {
    let test_cases = vec![
        // Valid Cases (Should NOT fail)
        (".CODE SEGMENT\nMOV AX, 0FFh\nENDS", false, ""),
        (".CODE SEGMENT\nMOV AX, 21h\nENDS", false, ""),
        (".DATA SEGMENT\nvar1 DW 100 DUP(0)\nENDS", false, ""),
        (".CODE SEGMENT\nstart:\nEND start", false, ""),
        // Invalid Cases (Should Fail)
        (".stacks segment", true, "Declaración de segmento inválida"),
        (
            ".CODE SEGMENT\nMOV AX, 0FF\nENDS",
            true,
            "Invalid syntax", // Expect parser error for 0FF
        ),
        ("msg DB \"hello", true, "Faltan comillas de cierre"),
        ("MOV AX, [BX", true, "Corchetes desbalanceados"),
        // Validator Errors
        (".DATA SEGMENT\nvar2 DB FFh\nENDS", true, "falta 0 inicial"),
        (
            ".DATA SEGMENT\nMOV AX, 0\nENDS",
            true,
            "no permitida en segmento de datos",
        ),
    ];

    let mut failed = false;

    for (code, should_fail, expected) in test_cases {
        let result = analyze_full_program_struct(code);
        let errors = result.errors;
        let has_error = !errors.is_empty();

        if should_fail {
            if has_error {
                let found = errors
                    .iter()
                    .any(|e| e.to_lowercase().contains(&expected.to_lowercase()));
                if found {
                    println!(
                        "PASS: '{}' -> Found '{}'",
                        code.lines().next().unwrap(),
                        expected
                    );
                } else {
                    println!(
                        "FAIL: '{}' -> Expected '{}', Got: {:?}",
                        code, expected, errors
                    );
                    failed = true;
                }
            } else {
                println!(
                    "FAIL: '{}' -> Expected Error '{}', but passed.",
                    code, expected
                );
                failed = true;
            }
        } else {
            if has_error {
                println!("FAIL: '{}' -> Expected Success, Got: {:?}", code, errors);
                failed = true;
            } else {
                println!("PASS: '{}' -> Success", code.lines().next().unwrap());
            }
        }
    }

    if failed {
        std::process::exit(1);
    }
}
