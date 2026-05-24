// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a change configuration notification.
pub struct DidChangeConfigurationParams {
    /// The actual changed settings
    pub settings: serde_json::Value,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeConfigurationRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<DidChangeConfigurationSection>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeConfigurationClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Did change configuration notification supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

pub type Params = DidChangeConfigurationParams;

pub type RegistrationOptions = DidChangeConfigurationRegistrationOptions;

pub type ClientCapabilities = DidChangeConfigurationClientCapabilities;