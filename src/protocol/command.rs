
use serde::{ Serialize, Deserialize };
use crate::error::ProtocolError;

/// Command package for animus communication.
/// Use this struct and methods within your application 
/// to generate accurate command messages.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
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
    pub fn decode(input: &[u8]) -> Result<Self, ProtocolError> {

        bincode::deserialize::<Self>(&input).map_err(|_| ProtocolError::Encode)
    }

    /// Serialize a Command to bytes.
    pub fn encode(&self) -> Result<Vec<u8>, ProtocolError> {

        bincode::serialize(&self).map_err(|_| ProtocolError::Decode)
    }

}

/// Commands for controlling service behavior.
/// Parallel to `AnimusCommand` from Brainstorm.
#[derive(Debug, Clone, Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
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

    /// Returns a list each `Structure` in the network.
    ListStructures,

    /// Returns a list of the tract names of each Output.
    ListOutputs,

    /// Returns a list of the tract names of each Input.
    ListInputs,

    /// Returns a list of `cajal_cx::ReceiverInfo` of each Input.
    ReportInputs,

    /// Returns the ReceiverInfo of a tract by name, if it exists.
    InputInfo(String),

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

    /// Connect the appropriate Output to the Input in the report.
    /// Passes the `ReceiverReport` as bytes.
    LinkOutput(cajal_cx::tract::receiver::ReceiverInfo),

    /// Attempt to link an Output to an arbitrary SocketAddr.
    /// This linkage is not checked for ReceiverInfo (unsafe), 
    /// so only use it when you can guarantee the SocketAddr is correct.
    UncheckedLink { tract: String, port: std::net::SocketAddr }
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
            Self::ListOutputs => "ListOutputs",
            Self::ListInputs => "ListInputs",
            Self::ReportInputs => "ReportReceivers",
            Self::Wake => "Wake",
            Self::Save => "Save",
            Self::Sleep => "Sleep",
            Self::Terminate => "Terminate",
            Self::Ignore => "Ignore",
            Self::InputInfo(..) => "InputInfo",
            Self::LinkOutput(..) => "LinkOutput",
            Self::UncheckedLink {..} => "UncheckedLink",
        };
        write!(f, "{}", as_str)
    }
}
