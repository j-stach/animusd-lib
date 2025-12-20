
use crate::error::{ EncodeError, DecodeError };

/// Command package for animus communication.
/// Use this struct and methods within your application 
/// to generate accurate command messages.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(bincode::Encode, bincode::Decode)]
pub struct Command {

    /// The name of the animus this command targets.
    pub name: String,

    /// The action to be executed by the animus.
    pub action: Action,
} 

impl Command {

    /// Provide the name of the animus and the action to perform.
    pub fn new(name: &str, action: Action) -> Self {
        Command { 
            name: name.to_owned(), 
            action 
        }
    }

    /// The animus will do nothing.
    /// This is used to ignore non-command UDP messages,
    /// but may also be useful in testing.
    pub fn ignore() -> Self {
        Command {
            name: String::new(),
            action: Action::Ignore,
        }
    }

    /// Deserialize a command message from bytes.
    pub fn decode(input: &[u8]) -> Result<Self, DecodeError> {

        let config = bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();

        let (cmd, _): (Self, usize) = 
            bincode::decode_from_slice(input, config)
                .unwrap();

        Ok(cmd)
    }

    /// Serialize a Command to bytes.
    pub fn encode(&self) -> Result<Vec<u8>, EncodeError> {

        let config = bincode::config::standard()
            .with_big_endian()
            .with_fixed_int_encoding();

        let bytes = bincode::encode_to_vec(self, config).unwrap();

        Ok(bytes)
    }

}

/// Commands for controlling service behavior.
/// Parallel to `AnimusCommand` from Brainstorm.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[derive(bincode::Encode, bincode::Decode)]
pub enum Action {

    /// Gets the name of the animus.
    /// Will execute even when not targeted: used for discovery.
    Query,

    /// Gets the name of the `Complex` that is managed by this animus.
    Name,

    /// Gets the version of `animusd` that is running.
    Version,

    /// Returns "Success" if animus is awake, and "Fail" if asleep.
    Status,

    /// List each `Structure` in the network.
    ListStructures,

    /// Begin processing signals.
    Wake,

    /// Serialize the new network state back to the save file.
    Save,

    /// Stop processing new stimuli (spin down neurotransmission).
    Sleep,

    /// Shut down the Animus service.
    Terminate,

    /// The animus will do nothing.
    /// This is used to ignore non-command UDP messages.
    Ignore,
}

use std::fmt;

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> { 
        let as_str = match self {
            Self::Query => "Query",
            Self::Name => "Name",
            Self::Version => "Version",
            Self::Status => "Status",
            Self::ListStructures => "ListStructures",
            Self::Wake => "Wake",
            Self::Save => "Save",
            Self::Sleep => "Sleep",
            Self::Terminate => "Terminate",
            Self::Ignore => "Ignore",
        };
        write!(f, "{}", as_str)
    }
}
