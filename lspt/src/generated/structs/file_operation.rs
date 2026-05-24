// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in notifications/requests for user-initiated creation of
/// files.
///
/// @since 3.16.0
pub struct CreateFilesParams {
    /// An array of all files/folders created in this operation.
    pub files: Vec<FileCreate>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in notifications/requests for user-initiated renames of
/// files.
///
/// @since 3.16.0
pub struct RenameFilesParams {
    /// An array of all files/folders renamed in this operation. When a folder is renamed, only
    /// the folder will be included, and not its children.
    pub files: Vec<FileRename>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in notifications/requests for user-initiated deletes of
/// files.
///
/// @since 3.16.0
pub struct DeleteFilesParams {
    /// An array of all files/folders deleted in this operation.
    pub files: Vec<FileDelete>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents information on a file/folder create.
///
/// @since 3.16.0
pub struct FileCreate {
    /// A file:// URI for the location of the file/folder being created.
    pub uri: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents information on a file/folder rename.
///
/// @since 3.16.0
pub struct FileRename {
    /// A file:// URI for the original location of the file/folder being renamed.
    pub old_uri: String,

    /// A file:// URI for the new location of the file/folder being renamed.
    pub new_uri: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents information on a file/folder delete.
///
/// @since 3.16.0
pub struct FileDelete {
    /// A file:// URI for the location of the file/folder being deleted.
    pub uri: String,
}


mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The options to register for file operations.
    ///
    /// @since 3.16.0
    pub struct FileOperationRegistrationOptions {
        /// The actual filters.
        pub filters: Vec<FileOperationFilter>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// A filter to describe in which file operation requests or notifications
    /// the server is interested in receiving.
    ///
    /// @since 3.16.0
    pub struct FileOperationFilter {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// A Uri scheme like `file` or `untitled`.
        pub scheme: Option<String>,

        /// The actual file operation pattern.
        pub pattern: FileOperationPattern,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// A pattern to describe in which file operation requests or notifications
    /// the server is interested in receiving.
    ///
    /// @since 3.16.0
    pub struct FileOperationPattern {
        /// The glob pattern to match. Glob patterns can have the following syntax:
        /// - `*` to match one or more characters in a path segment
        /// - `?` to match on one character in a path segment
        /// - `**` to match any number of path segments, including none
        /// - `{}` to group sub patterns into an OR expression. (e.g. `**​/*.{ts,js}` matches all TypeScript and JavaScript files)
        /// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
        /// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
        pub glob: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether to match files or folders with this pattern.
        ///
        /// Matches both if undefined.
        pub matches: Option<FileOperationPatternKind>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Additional options used during matching.
        pub options: Option<FileOperationPatternOptions>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Matching options for the file operation pattern.
    ///
    /// @since 3.16.0
    pub struct FileOperationPatternOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// The pattern should be matched ignoring casing.
        pub ignore_case: Option<bool>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Options for notifications/requests for user operations on files.
    ///
    /// @since 3.16.0
    pub struct FileOperationOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// The server is interested in receiving didCreateFiles notifications.
        pub did_create: Option<FileOperationRegistrationOptions>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The server is interested in receiving willCreateFiles requests.
        pub will_create: Option<FileOperationRegistrationOptions>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The server is interested in receiving didRenameFiles notifications.
        pub did_rename: Option<FileOperationRegistrationOptions>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The server is interested in receiving willRenameFiles requests.
        pub will_rename: Option<FileOperationRegistrationOptions>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The server is interested in receiving didDeleteFiles file notifications.
        pub did_delete: Option<FileOperationRegistrationOptions>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The server is interested in receiving willDeleteFiles file requests.
        pub will_delete: Option<FileOperationRegistrationOptions>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Capabilities relating to events from file operations by the user in the client.
    ///
    /// These events do not come from the file system, they come from user operations
    /// like renaming a file in the UI.
    ///
    /// @since 3.16.0
    pub struct FileOperationClientCapabilities {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether the client supports dynamic registration for file requests/notifications.
        pub dynamic_registration: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client has support for sending didCreateFiles notifications.
        pub did_create: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client has support for sending willCreateFiles requests.
        pub will_create: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client has support for sending didRenameFiles notifications.
        pub did_rename: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client has support for sending willRenameFiles requests.
        pub will_rename: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client has support for sending didDeleteFiles notifications.
        pub did_delete: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client has support for sending willDeleteFiles requests.
        pub will_delete: Option<bool>,
    }
}

pub type RegistrationOptions = raw::FileOperationRegistrationOptions;

pub type Filter = raw::FileOperationFilter;

pub type Pattern = raw::FileOperationPattern;

pub type PatternOptions = raw::FileOperationPatternOptions;

pub type Options = raw::FileOperationOptions;

pub type ClientCapabilities = raw::FileOperationClientCapabilities;