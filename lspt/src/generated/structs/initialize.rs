// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The process Id of the parent process that started
    /// the server.
    ///
    /// Is `null` if the process has not been started by another process.
    /// If the parent process is not alive then the server should exit.
    pub process_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Information about the client
    ///
    /// @since 3.15.0
    pub client_info: Option<ClientInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The locale the client is currently showing the user interface
    /// in. This must not necessarily be the locale of the operating
    /// system.
    ///
    /// Uses IETF language tags as the value's syntax
    /// (See https://en.wikipedia.org/wiki/IETF_language_tag)
    ///
    /// @since 3.16.0
    pub locale: Option<String>,

    /// The capabilities provided by the client (editor or tool)
    pub capabilities: ClientCapabilities,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// User provided initialization options.
    pub initialization_options: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The initial trace setting. If omitted trace is disabled ('off').
    pub trace: Option<TraceValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The workspace folders configured in the client when the server starts.
    ///
    /// This property is only available if the client supports workspace folders.
    /// It can be `null` if the client supports workspace folders but none are
    /// configured.
    ///
    /// @since 3.6.0
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result returned from an initialize request.
pub struct InitializeResult {
    /// The capabilities the language server provides.
    pub capabilities: ServerCapabilities,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Information about the server.
    ///
    /// @since 3.15.0
    pub server_info: Option<ServerInfo>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The data type of the ResponseError if the
/// initialize request fails.
pub struct InitializeError {
    /// Indicates whether the client execute the following retry logic:
    /// (1) show the message provided by the ResponseError to the user
    /// (2) user selects retry or cancel
    /// (3) if user selected retry the initialize method is sent again.
    pub retry: bool,
}

pub type Params = InitializeParams;

pub type Result = InitializeResult;