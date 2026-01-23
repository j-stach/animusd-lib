
use serde::{ Serialize, Deserialize };
use crate::error::ProtocolError;

use super::Action;


/// Response package for animus communication.
/// Use this struct and methods within your application 
/// to accurately parse response messages.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Report {

    /// The name of the animus sending the response.
    pub name: String,

    /// The command action that triggered the response.
    pub action: Action,

    /// The result of the action, reporting success/fail.
    pub outcome: Outcome, 
} 

impl Report {

    /// Serialize a Response to bytes.
    pub fn encode(&self) -> Result<Vec<u8>, ProtocolError> {

        bincode::serialize(&self).map_err(|_| ProtocolError::Decode)
    }

    /// Deserialize a Response message from bytes.
    pub fn decode(input: &[u8]) -> Result<Self, ProtocolError> {

        bincode::deserialize::<Self>(&input).map_err(|_| ProtocolError::Encode)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
pub enum Outcome {

    /// The action was succesfully executed.
    Success,

    /// The command was received but the action failed.
    /// See `/docs/troubleshooting.md`.
    Fail,

    /// Some commands request serialized information from the Animus.
    Return(Vec<u8>)
}

use std::fmt;

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let as_str = match self {
            Self::Success => "Success",
            Self::Fail => "Fail",
            Self::Return(..) => "Return",
        };
        write!(f, "{}", as_str)
    }
}

