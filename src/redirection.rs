use std::{fs, io, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("string is invalid")]
pub struct InvalidStrError;

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

#[derive(Debug, Error)]
pub enum ReadRedirectionError {
    #[error("IO operation failed")]
    IOError(#[from] io::Error),
    #[error(transparent)]
    ParseError(#[from] InvalidStrError),
}

pub fn get_redirections(file_name: &str) -> Result<Vec<Redirection>, ReadRedirectionError> {
    let contents = fs::read_to_string("./routes/".to_string() + file_name)?;

    let mut redirections: Vec<Redirection> = vec![];

    for line in contents.lines() {
        let redirection = Redirection::from_str(line)?;
        redirections.push(redirection);
    }

    Ok(redirections)
}
