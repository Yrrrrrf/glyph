//! User-defined symbols (labels, variables)
use super::Token;
use serde::Serialize;
use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Symbol(pub String);

impl Token for Symbol {
    fn from_str(s: &str) -> Option<Self> {
        // Must start with letter/underscore, then alphanumeric/underscore
        // and NOT end with h/H or b/B (which are constants)
        let is_valid = s.chars().next()?.is_alphabetic() || s.starts_with('_');
        let is_not_const =
            !(s.ends_with('h') || s.ends_with('H') || s.ends_with('b') || s.ends_with('B'));

        if is_valid && is_not_const && s.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Some(Symbol(s.to_string()))
        } else {
            None
        }
    }

    fn to_string(&self) -> String {
        self.0.clone()
    }
}
