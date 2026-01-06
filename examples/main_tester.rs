use glyph::analyze_full_program_struct;
use std::fs;

fn main() {
    let source = fs::read_to_string("static/x8086/test_astonishing.asm")
        // let source = fs::read_to_string("static/x8086/plantilla.asm")
        .expect("Something went wrong reading the file");

    let result = analyze_full_program_struct(&source);

    // Print headers similar to Phase 2 Table
    println!(
        "{:<4} | {:<20} | {:<50} | {:<10} | {:<20}",
        "Line", "Status", "Instruction", "Address", "Machine Code"
    );
    println!("{}", "-".repeat(110));

    for line in result.line_analysis {
        let status = if line.is_correct {
            "Correcta"
        } else {
            "Incorrecta"
        };
        let instr = line.instruction.trim();
        let addr = line.address.unwrap_or_default();
        let code = line.machine_code.unwrap_or_default();
        let err = line.error_message.unwrap_or_default();

        let display_instr = if !err.is_empty() {
            format!("{} [ERR: {}]", instr, err)
        } else {
            instr.to_string()
        };

        println!(
            "{:<4} | {:<20} | {:<50} | {:<10} | {:<20}",
            line.line_number,
            status,
            display_instr.chars().take(50).collect::<String>(),
            addr,
            code
        );
    }
}
