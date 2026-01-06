use glyph::analyze_full_program_struct;
use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("nahu-asm/ejemplo_errores.asm");
    if !path.exists() {
        println!("Test file not found at {:?}, skipping test.", path);
        return;
    }

    let source = fs::read_to_string(path).expect("Something went wrong reading the file");

    let result = analyze_full_program_struct(&source);

    let mut found_sem_tag = false;

    println!("Checking for [SEM] tags in error messages...");
    for line in result.line_analysis {
        if let Some(err) = line.error_message {
            println!("Line {}: {}", line.line_number, err);
            if err.contains("[SEM]") {
                found_sem_tag = true;
            }
        }
    }

    if found_sem_tag {
        println!("\nSUCCESS: Found [SEM] tags in error messages.");
    } else {
        println!("\nFAILURE: No [SEM] tags found in error messages.");
        std::process::exit(1);
    }
}
