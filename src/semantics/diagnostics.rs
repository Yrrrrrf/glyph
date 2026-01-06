pub fn diagnose_syntax_error(line: &str) -> String {
    let lower = line.to_lowercase();
    let trimmed = line.trim();

    // If parser failed and it looks like a segment decl, it's likely malformed.
    if lower.contains("segment") {
        return "Declaración de segmento inválida".to_string();
    }

    // Check for bad hex (starts with letter or non-zero digit, ends with h)
    // Simple check: word ending in 'h' where first char is not '0'
    for word in trimmed.split_whitespace() {
        // Strip common delimiters if any attached (like comma)
        let clean_word = word.trim_matches(|c| c == ',' || c == '[' || c == ']');
        if clean_word.to_lowercase().ends_with('h') {
            if clean_word.len() > 1 {
                let val = &clean_word[..clean_word.len() - 1];
                // Check if it's hex digits
                if val.chars().all(|c| c.is_ascii_hexdigit()) {
                    // STRICT CHECK: Must start with 0
                    if !val.starts_with('0') {
                        return format!(
                            "Constante Hex inválida '{}' (falta 0 inicial)",
                            clean_word
                        );
                    }
                }
            }
        }
    }

    if lower.contains("dup") && (!line.contains('(') || !line.contains(')')) {
        return "Formato DUP inválido. Use: count DUP(val)".to_string();
    }

    if (line.matches('"').count() % 2 != 0) || (line.matches('‘').count() % 2 != 0) {
        return "Faltan comillas de cierre".to_string();
    }

    if line.contains('[') && !line.contains(']') {
        return "Corchetes desbalanceados".to_string();
    }

    "Sintaxis inválida o token faltante".to_string()
}
