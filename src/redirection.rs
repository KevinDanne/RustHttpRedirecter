use std::{fs, str::FromStr};

use crate::error::Error;

#[derive(Debug)]
pub struct InvalidStrError;

impl From<InvalidStrError> for Error {
    fn from(e: InvalidStrError) -> Self {
        Self::Parse(e)
    }
}

pub struct Redirection {
    pub from: String,
    pub to: String,
}

impl FromStr for Redirection {
    type Err = InvalidStrError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once("=>").ok_or(InvalidStrError)?;

        Ok(Self {
            from: from.trim().to_string(),
            to: to.trim().to_string(),
        })
    }
}

pub fn get_redirections(file_name: &str) -> Result<Vec<Redirection>, Error> {
    let contents = fs::read_to_string(format!("./routes/{}", file_name))?;

    let mut redirections: Vec<Redirection> = vec![];

    for line in contents.lines() {
        let redirection = Redirection::from_str(line)?;
        redirections.push(redirection);
    }

    Ok(redirections)
}
