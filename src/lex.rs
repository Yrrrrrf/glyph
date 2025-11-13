//! Assembly lexer using logic_tracer's algorithm
use crate::tokens::{AssemblyToken, Token};

pub struct AssemblyLexer {
    chars: Vec<char>,
    pos: usize,
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
        }
    }
}

impl Iterator for AssemblyLexer {
    type Item = AssemblyToken;

    fn next(&mut self) -> Option<Self::Item> {
        // Skip whitespace
        while self.pos < self.chars.len() {
            if self.chars[self.pos].is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }

        if self.pos >= self.chars.len() {
            return None;
        }

        // Special handling for quoted strings
        if self.chars[self.pos] == '"' {
            return self.parse_quoted_string();
        }

        // Find the longest valid token starting from the current position
        let mut longest_token = None;
        let mut longest_len = 0;
        let mut current_pos = self.pos;
        let mut accum = String::new();

        // Try to build the longest possible token
        while current_pos < self.chars.len() {
            accum.push(self.chars[current_pos]);
            
            if let Some(token) = Self::try_tokenize(&accum) {
                // This is a valid token, remember it
                longest_token = Some(token);
                longest_len = accum.len();
            } else {
                // Current accumulation is not valid, stop here
                break;
            }
            
            current_pos += 1;
        }

        // If we found a valid token, advance position and return it
        if let Some(token) = longest_token {
            self.pos += longest_len;
            return Some(token);
        } else {
            // If no valid token found at all, try single character tokens
            let single_char = self.chars[self.pos].to_string();
            if let Some(token) = Self::try_tokenize(&single_char) {
                self.pos += 1;
                return Some(token);
            } else {
                // Skip this character if it's not a valid token
                self.pos += 1;
                return None;
            }
        }
    }
}

impl AssemblyLexer {
    /// Parse a quoted string token
    fn parse_quoted_string(&mut self) -> Option<AssemblyToken> {
        if self.pos >= self.chars.len() || self.chars[self.pos] != '"' {
            return None;
        }

        let mut accum = String::new();
        
        // Add the opening quote
        accum.push(self.chars[self.pos]);
        self.pos += 1;

        // Continue until we find the closing quote or reach end of input
        while self.pos < self.chars.len() {
            let ch = self.chars[self.pos];
            accum.push(ch);
            self.pos += 1;

            // If this is a closing quote, we're done
            if ch == '"' {
                // If we have a valid string token, return it
                if let Some(token) = Self::try_tokenize(&accum) {
                    return Some(token);
                }
            }
        }

        // If we reached end of input without finding closing quote,
        // we'll return what we have but this should not happen in a valid program
        // For now, let's just return None to skip the incomplete string
        None
    }
}

impl AssemblyLexer {
    /// Try to tokenize a string - order matters for disambiguation!
    fn try_tokenize(s: &str) -> Option<AssemblyToken> {
        // 1. Constants first (FFh should be constant, not symbol)
        if let Some(c) = crate::tokens::constant::Constant::from_str(s) {
            return Some(AssemblyToken::Constant(c));
        }

        // 2. Keywords (case-insensitive)
        if let Some(i) = crate::tokens::instruction::Instruction::from_str(s) {
            return Some(AssemblyToken::Instruction(i));
        }
        if let Some(p) = crate::tokens::pseudoinstruction::Pseudoinstruction::from_str(s) {
            return Some(AssemblyToken::Pseudoinstruction(p));
        }
        if let Some(r) = crate::tokens::register::Register::from_str(s) {
            return Some(AssemblyToken::Register(r));
        }

        // 3. Punctuation
        if let Some(p) = crate::tokens::punctuation::Punctuation::from_str(s) {
            return Some(AssemblyToken::Punctuation(p));
        }

        // 4. Symbols (catch-all, but not constants)
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