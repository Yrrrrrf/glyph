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
        // Remove whitespace/comments but preserve for error reporting
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
}

impl Iterator for AssemblyLexer {
    type Item = (AssemblyToken, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // 1. Skip whitespace
        while self.pos < self.chars.len() {
            if self.chars[self.pos] == '\n' {
                self.line += 1;
                self.pos += 1;
            } else if self.chars[self.pos].is_whitespace() {
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

        // 2. Handle Strings ("..." or '...')
        if ch == '"' || ch == '\'' {
            return self.parse_quoted_string(start_line);
        }

        // 3. Handle Punctuation (Single characters usually)
        // We try this first. If the char is punctuation (like ',' or ':'), return immediately.
        let single_char_str = ch.to_string();
        if let Some(p) = crate::tokens::punctuation::Punctuation::from_str(&single_char_str) {
            self.pos += 1;
            return Some((AssemblyToken::Punctuation(p), start_line));
        }

        // 4. Handle Words (Instructions, Registers, Constants, Symbols)
        // Accumulate everything that looks like a word (alphanumeric, _, ., $)
        // We do NOT stop on numbers or 'x' inside a word.
        let mut accum = String::new();
        while self.pos < self.chars.len() {
            let c = self.chars[self.pos];
            
            // Stop if we hit whitespace, newline, or punctuation
            if c.is_whitespace() || c == '\n' {
                break;
            }
            // If we hit a punctuation character (like ',' or ':'), stop!
            // But only if it IS actually punctuation.
            if crate::tokens::punctuation::Punctuation::from_str(&c.to_string()).is_some() {
                break;
            }
            
            accum.push(c);
            self.pos += 1;
        }

        // 5. Tokenize the accumulated word
        if !accum.is_empty() {
            if let Some(token) = Self::try_tokenize(&accum) {
                return Some((token, start_line));
            } else {
                // If we consumed a chunk but it's valid (e.g. "0x" without digits), it's invalid
                return Some((AssemblyToken::Invalid(accum), start_line));
            }
        }

        // Fallback (should rarely reach here unless strange characters)
        self.pos += 1;
        Some((AssemblyToken::Invalid(ch.to_string()), start_line))
    }
}

impl AssemblyLexer {
    /// Parse a quoted string token - FIXED to return (token, line)
    fn parse_quoted_string(&mut self, current_line: usize) -> Option<(AssemblyToken, usize)> {
        // Removed self.pos check - already validated by caller
        let quote_char = self.chars[self.pos];
        let mut accum = String::new();
        accum.push(quote_char);
        self.pos += 1;

        // Accumulate until closing quote or end of input
        while self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            accum.push(ch);
            self.pos += 1;

            if ch == quote_char {
                // Found closing quote - try to tokenize
                if let Some(token) = Self::try_tokenize(&accum) {
                    return Some((token, current_line));
                }
            }
        }

        // Unclosed string - return as invalid
        Some((AssemblyToken::Invalid(accum), current_line))
    }

    /// Try to tokenize a string - order matters for disambiguation!
fn try_tokenize(s: &str) -> Option<AssemblyToken> {
        // 1. Keywords & Registers (Specifics first)
        // This prevents 'AH' (Register) from being read as Hex 'A'
        if let Some(r) = crate::tokens::register::Register::from_str(s) {
            return Some(AssemblyToken::Register(r));
        }
        if let Some(i) = crate::tokens::instruction::Instruction::from_str(s) {
            return Some(AssemblyToken::Instruction(i));
        }
        if let Some(p) = crate::tokens::pseudoinstruction::Pseudoinstruction::from_str(s) {
            return Some(AssemblyToken::Pseudoinstruction(p));
        }

        // 2. Constants (0x..., 123, "str")
        if let Some(c) = crate::tokens::constant::Constant::from_str(s) {
            return Some(AssemblyToken::Constant(c));
        }

        // 3. Punctuation
        if let Some(p) = crate::tokens::punctuation::Punctuation::from_str(s) {
            return Some(AssemblyToken::Punctuation(p));
        }

        // 4. Symbols (Catch-all for identifiers)
        // Note: Constant logic now requires leading digit for Hex/Bin, 
        // so 'my_byte' won't be confused for a constant.
        if let Some(sym) = crate::tokens::symbol::Symbol::from_str(s) {
            return Some(AssemblyToken::Symbol(sym));
        }

        None
    }

}

/// Public API - matches original logos interface
pub fn lexer(source: &str) -> AssemblyLexer {
    AssemblyLexer::new(source)
}
