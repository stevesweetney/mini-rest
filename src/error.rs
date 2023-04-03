use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct AudioError;
pub type Result<T> = std::result::Result<T, AudioError>;

const ERROR_DESCRIPTION: &str = "Audio related error";

impl Display for AudioError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", ERROR_DESCRIPTION)
    }
}

impl Error for AudioError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        ERROR_DESCRIPTION
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}
