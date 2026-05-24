// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link ExecuteCommandRequest}.
pub struct ExecuteCommandParams {
    /// The identifier of the actual command handler.
    pub command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Arguments that the command should be invoked with.
    pub arguments: Option<Vec<serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link ExecuteCommandRequest}.
pub struct ExecuteCommandRegistrationOptions {
    /// The commands to be executed on the server
    pub commands: Vec<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The server capabilities of a {@link ExecuteCommandRequest}.
pub struct ExecuteCommandOptions {
    /// The commands to be executed on the server
    pub commands: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The client capabilities of a {@link ExecuteCommandRequest}.
pub struct ExecuteCommandClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Execute command supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

pub type Params = ExecuteCommandParams;

pub type RegistrationOptions = ExecuteCommandRegistrationOptions;

pub type Options = ExecuteCommandOptions;

pub type ClientCapabilities = ExecuteCommandClientCapabilities;