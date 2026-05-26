// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An event describing a file change.
pub struct FileEvent {
    /// The file's uri.
    pub uri: Uri,

    #[serde(rename = "type")]
    /// The change type.
    pub ty: FileChangeType,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSystemWatcher {
    /// The glob pattern to watch. See {@link GlobPattern glob pattern} for more detail.
    ///
    /// @since 3.17.0 support for relative patterns.
    pub glob_pattern: GlobPattern,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The kind of events of interest. If omitted it defaults
    /// to WatchKind.Create | WatchKind.Change | WatchKind.Delete
    /// which is 7.
    pub kind: Option<WatchKind>,
}


mod raw {
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
}

pub type Params = raw::DidChangeWatchedFilesParams;

pub type RegistrationOptions = raw::DidChangeWatchedFilesRegistrationOptions;

pub type ClientCapabilities = raw::DidChangeWatchedFilesClientCapabilities;