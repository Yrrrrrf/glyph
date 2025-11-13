//! Punctuation tokens
use super::*;
use serde::Serialize;

define_tokens!(Punctuation {
    Comma => [","],
    Period => ["."],
    LeftBracket => ["["],
    RightBracket => ["]"],
    Plus => ["+"],
    Minus => ["-"],
    Colon => [":"],
});