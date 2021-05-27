use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("{pos:?}: unexpected token: expected {expected} found {found}")]
    UnexpectedToken {
        pos: (usize, usize, usize),
        expected: String,
        found: String,
    },
}
