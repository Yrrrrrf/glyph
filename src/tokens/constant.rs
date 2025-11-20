// src/tokens/constant.rs
use super::*;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Constant {
    Decimal(u64),
    Hexadecimal(u64),
    Binary(u8),
    String(String),
}

impl Token for Constant {
    fn from_str(s: &str) -> Option<Self> {
        // 1. Strings
        if (s.starts_with('"') && s.ends_with('"') && s.len() > 1)
            || (s.starts_with('\'') && s.ends_with('\'') && s.len() > 1)
        {
            return Some(Constant::String(s[1..s.len() - 1].to_string()));
        }

        // 2. Hexadecimal with Prefix (0x100)
        if let Some(hex_part) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
            if !hex_part.is_empty() {
                return u64::from_str_radix(hex_part, 16)
                    .ok()
                    .map(Constant::Hexadecimal);
            }
        }

        // 3. Hexadecimal with Suffix (0FFh)
        // STRICT CHECK: Must start with a digit to be a hex constant in this parser
        // This prevents 'Ah' (Register) from being read as Hex(10)
        if let Some(hex_part) = s.strip_suffix('h').or_else(|| s.strip_suffix('H')) {
            if !hex_part.is_empty() && s.chars().next()?.is_numeric() {
                return u64::from_str_radix(hex_part, 16)
                    .ok()
                    .map(Constant::Hexadecimal);
            }
        }

        // 4. Binary (1010b)
        if let Some(bin_part) = s.strip_suffix('b').or_else(|| s.strip_suffix('B')) {
            if !bin_part.is_empty()
                && s.chars().next()?.is_numeric()
                && bin_part.chars().all(|c| c == '0' || c == '1')
            {
                return u8::from_str_radix(bin_part, 2).ok().map(Constant::Binary);
            }
        }

        // 5. Decimal (123)
        s.parse().ok().map(Constant::Decimal)
    }

    fn to_string(&self) -> String {
        match self {
            Constant::Decimal(n) => n.to_string(),
            Constant::Hexadecimal(n) => format!("0x{:X}", n), // Standardize on 0x for output
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
