use std::{
    fmt::{self, Display},
    io::Error as IoError,
};

use crate::redirection::InvalidStrError;

#[derive(Debug)]
pub enum Error {
    IO(IoError),
    InvalidRequest,
    Parse(InvalidStrError),
    ArgumentCount,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match &self {
            Self::IO(err) => err.to_string(),
            Self::InvalidRequest => "Request was invalid".to_string(),
            Self::ArgumentCount => "Invalid number of arguments given".to_string(),
            Self::Parse(_) => "Invalid string error".to_string(),
        };

        write!(f, "{}", message)
    }
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::IO(e)
    }
}
