
use super::Action;
use crate::error::{ EncodeError, DecodeError };


/// Response package for TCP communication.
/// Use this struct and methods within your application 
/// to accurately parse response messages.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(bincode::Encode, bincode::Decode)]
pub struct Report {

    /// The name of the animus sending the response.
    pub name: String,

    /// The command action that triggered the response.
    pub action: Action,

    /// The result of the action, reporting success/fail.
    pub outcome: Outcome, 
} 

impl Report {

    // Serialize a Response to bytes.
    #[allow(dead_code)]
    pub(crate) fn encode(&self) -> Result<Vec<u8>, EncodeError> {

        let config = bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();

        bincode::encode_to_vec(self, config)
    }

    /// Deserialize a Response message from bytes.
    pub fn decode(input: &[u8]) -> Result<Self, DecodeError> {

        let config = bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();

        let (rsp, _): (Self, usize) = 
            bincode::decode_from_slice(input, config)?;

        Ok(rsp)
    }
}


#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(bincode::Encode, bincode::Decode)]
pub enum Outcome {

    /// The action was succesfully executed.
    Success,

    /// The command was received but the action failed.
    /// See `/docs/troubleshooting.md`.
    Fail,

    /// Some commands request information from the Animus.
    Return(String)
}

use std::fmt;

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let as_str = match self {
            Self::Success => "Success",
            Self::Fail => "Fail",
            Self::Return(msg) => msg,
        };
        write!(f, "{}", as_str)
    }
}

