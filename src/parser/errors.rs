// use thiserror::Error;
use crate::token::token::Token;

#[derive(Debug, Copy, Clone)]
pub enum ParserError {
    // #[error("Unexpected token")]
    UnexpectedToken,
}