// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientFoldingRangeKindOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The folding range kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    pub value_set: Option<Vec<FoldingRangeKind>>,
}

pub type Options = ClientFoldingRangeKindOptions;