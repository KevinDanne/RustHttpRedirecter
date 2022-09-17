use std::{error::Error, fmt, fs, io, str::FromStr};

#[derive(Debug)]
pub struct InvalidStrError;

impl Error for InvalidStrError {}

impl fmt::Display for InvalidStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "String {} is invalid", self)
    }
}

pub struct Redirection {
    pub from: String,
    pub to: String,
}

impl FromStr for Redirection {
    type Err = InvalidStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = if let Some((from, to)) = s.split_once("=>") {
            (from, to)
        } else {
            return Err(InvalidStrError);
        };

        Ok(Self {
            from: from.trim().to_string(),
            to: to.trim().to_string(),
        })
    }
}

#[derive(Debug)]
pub enum ReadRedirectionError {
    IOError(io::Error),
    ParseError(InvalidStrError),
}

impl Error for ReadRedirectionError {}

impl fmt::Display for ReadRedirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IOError(err) => err.fmt(f),
            Self::ParseError(err) => err.fmt(f),
        }
    }
}

pub fn get_redirections(file_name: &str) -> Result<Vec<Redirection>, ReadRedirectionError> {
    let contents = match fs::read_to_string("./routes/".to_string() + file_name) {
        Ok(val) => val,
        Err(err) => return Err(ReadRedirectionError::IOError(err)),
    };

    let mut redirections: Vec<Redirection> = vec![];

    for line in contents.lines() {
        let redirection = match Redirection::from_str(line) {
            Ok(val) => val,
            Err(err) => return Err(ReadRedirectionError::ParseError(err)),
        };
        redirections.push(redirection);
    }

    Ok(redirections)
}
