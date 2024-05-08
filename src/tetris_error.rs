use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum TetrisError {
    SimepleErr(&'static str),
}

impl Display for TetrisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TetrisError")
    }
}

impl Error for TetrisError {}
