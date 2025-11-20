// src/tokens/symbol.rs
use super::Token;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Symbol(pub String);

impl Token for Symbol {
    fn from_str(s: &str) -> Option<Self> {
        if s.is_empty() {
            return None;
        }

        let first_char = s.chars().next()?;

        // Valid Symbol: Starts with Letter or Underscore
        // It CANNOT start with a number (that would be a constant)
        let is_valid_start = first_char.is_alphabetic() || first_char == '_';

        if is_valid_start && s.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Some(Symbol(s.to_string()))
        } else {
            None
        }
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }
}
