use thiserror::Error;
use crate::token::token::Token;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{pos:?}: unexpected token: expected {expected} found {found}")]
    UnexpectedToken{pos: (usize, usize), expected: String, found: String},
}
