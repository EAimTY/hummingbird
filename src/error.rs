use std::{convert::Infallible, error::Error, fmt};

#[derive(Debug)]
pub enum ErrorHandler {
    Infallible(Infallible),
}

impl From<Infallible> for ErrorHandler {
    fn from(err: Infallible) -> Self {
        ErrorHandler::Infallible(err)
    }
}

impl fmt::Display for ErrorHandler {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ErrorHandler::*;
        match self {
            Infallible(err) => write!(out, "infallible error: {}", err),
        }
    }
}

impl Error for ErrorHandler {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::ErrorHandler::*;
        Some(match self {
            Infallible(err) => err,
        })
    }
}
