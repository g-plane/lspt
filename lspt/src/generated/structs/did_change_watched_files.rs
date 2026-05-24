// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The watched files change notification's parameters.
pub struct DidChangeWatchedFilesParams {
    /// The actual file events.
    pub changes: Vec<FileEvent>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describe options to be used when registered for text document change events.
pub struct DidChangeWatchedFilesRegistrationOptions {
    /// The watchers to register.
    pub watchers: Vec<FileSystemWatcher>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeWatchedFilesClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Did change watched files notification supports dynamic registration. Please note
    /// that the current protocol doesn't support static configuration for file changes
    /// from the server side.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client has support for {@link  RelativePattern relative pattern}
    /// or not.
    ///
    /// @since 3.17.0
    pub relative_pattern_support: Option<bool>,
}

pub type Params = DidChangeWatchedFilesParams;

pub type RegistrationOptions = DidChangeWatchedFilesRegistrationOptions;

pub type ClientCapabilities = DidChangeWatchedFilesClientCapabilities;