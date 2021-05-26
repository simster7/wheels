use thiserror::Error;
use crate::token::token::Token;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("unexpected token: expected {expected:?} found {found:?}")]
    UnexpectedToken{expected: String, found: String},
}
