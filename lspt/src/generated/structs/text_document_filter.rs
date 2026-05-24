// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentFilterClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports Relative Patterns.
    ///
    /// @since 3.18.0
    pub relative_pattern_support: Option<bool>,
}

pub type ClientCapabilities = TextDocumentFilterClientCapabilities;