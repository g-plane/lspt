// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a `workspace/didChangeWorkspaceFolders` notification.
pub struct DidChangeWorkspaceFoldersParams {
    /// The actual workspace folder change event.
    pub event: WorkspaceFoldersChangeEvent,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The workspace folder change event.
pub struct WorkspaceFoldersChangeEvent {
    /// The array of added workspace folders
    pub added: Vec<WorkspaceFolder>,

    /// The array of the removed workspace folders
    pub removed: Vec<WorkspaceFolder>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFoldersServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server has support for workspace folders
    pub supported: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the server wants to receive workspace folder
    /// change notifications.
    ///
    /// If a string is provided the string is treated as an ID
    /// under which the notification is registered on the client
    /// side. The ID can be used to unregister for these events
    /// using the `client/unregisterCapability` request.
    pub change_notifications: Option<WorkspaceFoldersChangeNotifications>,
}

pub type DidChangeParams = DidChangeWorkspaceFoldersParams;

pub type ChangeEvent = WorkspaceFoldersChangeEvent;

pub type ServerCapabilities = WorkspaceFoldersServerCapabilities;