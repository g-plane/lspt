// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientShowMessageActionItemOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports additional attributes which
    /// are preserved and send back to the server in the
    /// request's response.
    pub additional_properties_support: Option<bool>,
}

pub type Options = ClientShowMessageActionItemOptions;