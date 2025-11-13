//! Literal constants (decimal, hex, binary, string)
use super::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Constant {
    Decimal(u64),
    Hexadecimal(u64),
    Binary(u8), // Store as u8 for simplicity
    String(String),
}

impl Token for Constant {
    fn from_str(s: &str) -> Option<Self> {
        // String: "hello"
        if s.starts_with('"') && s.ends_with('"') && s.len() > 1 {
            return Some(Constant::String(s[1..s.len() - 1].to_string()));
        }

        // Hex: FFh or 0FFh
        if let Some(hex_part) = s.strip_suffix('h').or_else(|| s.strip_suffix('H')) {
            if !hex_part.is_empty() {
                return u64::from_str_radix(hex_part, 16)
                    .ok()
                    .map(Constant::Hexadecimal);
            }
        }

        // Binary: 1010b
        if let Some(bin_part) = s.strip_suffix('b').or_else(|| s.strip_suffix('B')) {
            if bin_part.chars().all(|c| c == '0' || c == '1') {
                return u8::from_str_radix(bin_part, 2).ok().map(Constant::Binary);
            }
        }

        // Decimal: 123
        s.parse().ok().map(Constant::Decimal)
    }

    fn to_string(&self) -> String {
        match self {
            Constant::Decimal(n) => n.to_string(),
            Constant::Hexadecimal(n) => format!("{:X}h", n),
            Constant::Binary(n) => format!("{:b}b", n),
            Constant::String(s) => format!("\"{}\"", s),
        }
    }
}

/// For analysis panel
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum ConstantVariant {
    Decimal,
    Hexadecimal,
    Binary,
    String,
}

impl Constant {
    pub fn variant(&self) -> ConstantVariant {
        match self {
            Self::Decimal(_) => ConstantVariant::Decimal,
            Self::Hexadecimal(_) => ConstantVariant::Hexadecimal,
            Self::Binary(_) => ConstantVariant::Binary,
            Self::String(_) => ConstantVariant::String,
        }
    }
}
