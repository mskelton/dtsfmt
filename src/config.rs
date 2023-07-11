use std::{fmt, path::PathBuf};

/// Defines the name of an input - either a file or stdin.
#[derive(Clone, Debug)]
pub enum Filename {
    Real(PathBuf),
    Stdin,
}

impl fmt::Display for Filename {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Filename::Real(p) => write!(f, "{}", p.to_str().unwrap()),
            Filename::Stdin => write!(f, "<stdin>"),
        }
    }
}
