use crate::error::ParseError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CommandType {
    Get,
    Set,
}

impl FromStr for CommandType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(Self::Get),
            "set" => Ok(Self::Set),
            other => Err(ParseError::InvalidCommandType(other.to_owned())),
        }
    }
}