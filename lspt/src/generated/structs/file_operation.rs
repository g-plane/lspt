// DO NOT EDIT THIS GENERATED FILE.

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

pub type RegistrationOptions = FileOperationRegistrationOptions;

pub type Options = FileOperationOptions;

pub type ClientCapabilities = FileOperationClientCapabilities;