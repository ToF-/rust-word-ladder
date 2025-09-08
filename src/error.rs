use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct WordNotFoundError {
    pub word: String
}

impl fmt::Display for WordNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the word {} does not exist", self.word)
    }
}

impl Error for WordNotFoundError {}
