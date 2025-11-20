// src/lex.rs
//! Assembly lexer using logic_tracer's algorithm
use crate::tokens::{AssemblyToken, Token};

pub struct AssemblyLexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
}

impl AssemblyLexer {
    pub fn new(source: &str) -> Self {
        // Remove comments (;) but keep lines intact
        let clean: String = source
            .lines()
            .map(|line| {
                if let Some(pos) = line.find(';') {
                    &line[..pos]
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        Self {
            chars: clean.chars().collect(),
            pos: 0,
            line: 1,
        }
    }

    /// Helper to peek at the next non-whitespace character
    fn peek_char(&self, offset: usize) -> Option<char> {
        if self.pos + offset < self.chars.len() {
            Some(self.chars[self.pos + offset])
        } else {
            None
        }
    }
}

impl Iterator for AssemblyLexer {
    type Item = (AssemblyToken, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // 1. Skip whitespace and track lines
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            if c == '\n' {
                self.line += 1;
                self.pos += 1;
            } else if c.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }

        if self.pos >= self.chars.len() {
            return None;
        }

        let start_line = self.line;
        let ch = self.chars[self.pos];

        // 2. Handle Strings ("..." or '...') - COMPOUND ELEMENT
        if ch == '"' || ch == '\'' {
            return self.parse_quoted_string(start_line);
        }

        // 3. Handle Brackets ([...]) - COMPOUND ELEMENT
        // Requirement: "[xxx]" must not be separated
        if ch == '[' {
            let mut accum = String::new();
            accum.push(ch);
            self.pos += 1;
            while self.pos < self.chars.len() {
                let c = self.chars[self.pos];
                accum.push(c);
                self.pos += 1;
                if c == ']' {
                    break;
                }
            }
            // Return the whole [BX+SI] as a Symbol (or Invalid if empty)
            // We treat memory references as Symbols in the Lexer phase for simplicity
            return Some((
                AssemblyToken::Symbol(crate::tokens::symbol::Symbol(accum)),
                start_line,
            ));
        }

        // 4. Handle Punctuation
        // Check if it is punctuation ONLY if it's not part of a word/number logic
        let single_char_str = ch.to_string();
        if crate::tokens::punctuation::Punctuation::from_str(&single_char_str).is_some() {
            self.pos += 1;
            // Re-check logic: Punctuation from_str might match '.', but '.' can start a label or directive
            // So we only return punctuation if it doesn't look like the start of a word
            // Exception: Comma and Colon are always punctuation
            if ch == ',' || ch == ':' {
                return Some((
                    AssemblyToken::Punctuation(
                        crate::tokens::punctuation::Punctuation::from_str(&single_char_str)
                            .unwrap(),
                    ),
                    start_line,
                ));
            }
            // For '.' or others, we fall through to word accumulation to see if it's .DATA
        }

        // 5. Accumulate a Word
        let mut accum = String::new();
        let start_pos = self.pos;

        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            // Break on whitespace, newline, or hard punctuation
            if c.is_whitespace() || c == '\n' || c == ',' || c == ':' || c == '[' || c == ']' {
                break;
            }
            accum.push(c);
            self.pos += 1;
        }

        // 6. Check for "Compound Words" (Lookahead)
        // Requirement: "byte ptr", ".data segment", "dup(xxx)"

        // A. Check DUP(xxx)
        if accum.to_lowercase() == "dup" {
            // Check if next char is '(' ignoring whitespace
            let mut temp_pos = self.pos;
            while temp_pos < self.chars.len()
                && self.chars[temp_pos].is_whitespace()
                && self.chars[temp_pos] != '\n'
            {
                temp_pos += 1;
            }

            if temp_pos < self.chars.len() && self.chars[temp_pos] == '(' {
                // It is a DUP! Consume the whitespace and the parens
                let mut full_dup = accum.clone();

                // Add the gap spaces if any (optional, usually we normalize, but let's just grab the parens)
                // Simply fast forward self.pos to temp_pos and consume until ')'
                self.pos = temp_pos;

                while self.pos < self.chars.len() {
                    let c = self.chars[self.pos];
                    full_dup.push(c);
                    self.pos += 1;
                    if c == ')' {
                        break;
                    }
                }
                // Return as Symbol (since DUP(3) isn't exactly a keyword constant, it's a construct)
                // or define a specific token for it. For this assignment, Symbol is safest.
                return Some((
                    AssemblyToken::Symbol(crate::tokens::symbol::Symbol(full_dup)),
                    start_line,
                ));
            }
        }

        // B. Check "WORD PTR", "BYTE PTR", ".DATA SEGMENT", etc.
        // We look ahead for specific keywords
        let lower_accum = accum.to_lowercase();
        if lower_accum == "byte"
            || lower_accum == "word"
            || lower_accum == "dword"
            || lower_accum == ".data"
            || lower_accum == ".code"
            || lower_accum == ".stack"
        {
            // Peek next word
            let mut temp_pos = self.pos;
            let mut gap_whitespace = String::new();

            // Skip white
            while temp_pos < self.chars.len()
                && self.chars[temp_pos].is_whitespace()
                && self.chars[temp_pos] != '\n'
            {
                gap_whitespace.push(self.chars[temp_pos]);
                temp_pos += 1;
            }

            // Read next word
            let mut next_word = String::new();
            while temp_pos < self.chars.len() {
                let c = self.chars[temp_pos];
                if c.is_whitespace() || c == '\n' || c == ',' || c == ':' {
                    break;
                }
                next_word.push(c);
                temp_pos += 1;
            }

            let lower_next = next_word.to_lowercase();
            let merged = if (lower_accum.ends_with("ptr") == false && lower_next == "ptr")
                || (lower_next == "segment")
            {
                // FOUND COMPOUND: "byte" + " " + "ptr"
                Some((gap_whitespace, next_word))
            } else {
                None
            };

            if let Some((gap, second_part)) = merged {
                accum.push_str(&gap);
                accum.push_str(&second_part);
                self.pos = temp_pos; // Advance real position
            }
        }

        // 7. Tokenize the Result
        if !accum.is_empty() {
            if let Some(token) = Self::try_tokenize(&accum) {
                return Some((token, start_line));
            } else {
                // If it contains spaces (like "byte ptr") but wasn't in our enum, try_tokenize fails.
                // But since we updated pseudoinstruction.rs, "BYTE PTR" should match!

                // Fallback for things like "dup(5)" or "[bx]" which won't match basic tokens
                // We return them as Symbols.
                return Some((
                    AssemblyToken::Symbol(crate::tokens::symbol::Symbol(accum)),
                    start_line,
                ));
            }
        }

        // Fallback for lone punctuation that wasn't caught earlier (like '.')
        if let Some(p) = crate::tokens::punctuation::Punctuation::from_str(&ch.to_string()) {
            self.pos += 1;
            return Some((AssemblyToken::Punctuation(p), start_line));
        }

        self.pos += 1;
        Some((AssemblyToken::Invalid(ch.to_string()), start_line))
    }
}

impl AssemblyLexer {
    // In src/lex.rs

    fn parse_quoted_string(&mut self, current_line: usize) -> Option<(AssemblyToken, usize)> {
        let quote_char = self.chars[self.pos];
        let mut accum = String::new();
        accum.push(quote_char);
        self.pos += 1;

        while self.pos < self.chars.len() {
            let ch = self.chars[self.pos];

            // CRITICAL FIX: Stop if we hit a newline!
            if ch == '\n' {
                // Return what we have as Invalid, so the rest of the file can be processed
                return Some((AssemblyToken::Invalid(accum), current_line));
            }

            accum.push(ch);
            self.pos += 1;

            if ch == quote_char {
                // Valid closed string
                if let Some(token) = Self::try_tokenize(&accum) {
                    return Some((token, current_line));
                }
                break;
            }
        }

        // End of file reached without closing quote
        Some((AssemblyToken::Invalid(accum), current_line))
    }

    fn try_tokenize(s: &str) -> Option<AssemblyToken> {
        // 1. Keywords & Registers (Specifics first)
        if let Some(r) = crate::tokens::register::Register::from_str(s) {
            return Some(AssemblyToken::Register(r));
        }
        if let Some(i) = crate::tokens::instruction::Instruction::from_str(s) {
            return Some(AssemblyToken::Instruction(i));
        }
        if let Some(p) = crate::tokens::pseudoinstruction::Pseudoinstruction::from_str(s) {
            return Some(AssemblyToken::Pseudoinstruction(p));
        }

        // 2. Constants
        if let Some(c) = crate::tokens::constant::Constant::from_str(s) {
            return Some(AssemblyToken::Constant(c));
        }

        // 3. Punctuation
        if let Some(p) = crate::tokens::punctuation::Punctuation::from_str(s) {
            return Some(AssemblyToken::Punctuation(p));
        }

        // 4. Symbols (Catch-all)
        // Modified logic in tokens/symbol.rs handles basic identifiers.
        // For "Complex" symbols like [BX] or DUP(3), symbol::from_str might fail
        // if it enforces alphanumeric only.
        // We might need to relax symbol.rs OR handle it here.
        // Let's check symbol.rs...

        if let Some(sym) = crate::tokens::symbol::Symbol::from_str(s) {
            return Some(AssemblyToken::Symbol(sym));
        }

        // If we are here, it might be a "Compound Symbol" that Symbol::from_str rejected
        // (e.g. contains brackets or parens). We should accept it if it came from our greedy logic.
        if s.contains('[') || s.contains('(') {
            return Some(AssemblyToken::Symbol(crate::tokens::symbol::Symbol(
                s.to_string(),
            )));
        }

        None
    }
}

pub fn lexer(source: &str) -> AssemblyLexer {
    AssemblyLexer::new(source)
}
