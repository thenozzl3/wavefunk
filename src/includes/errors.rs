use core::fmt;
use std::io;

#[derive(Debug)]
pub enum MatrixError {
    Io(io::Error),
    ParseError,
    ArgsError,
}

impl fmt::Display for MatrixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            MatrixError::Io(ref err) => err.fmt(f),
            MatrixError::ParseError => write!(f, "example matrix error"),
            MatrixError::ArgsError => write!(f, "args error"),
        }
    }
}
