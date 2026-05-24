// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities specific to regular expressions.
///
/// @since 3.16.0
pub struct RegularExpressionsClientCapabilities {
    /// The engine's name.
    pub engine: RegularExpressionEngineKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The engine's version.
    pub version: Option<String>,
}

pub type ClientCapabilities = RegularExpressionsClientCapabilities;