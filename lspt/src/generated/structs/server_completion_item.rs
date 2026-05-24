// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ServerCompletionItemOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server has support for completion item label
    /// details (see also `CompletionItemLabelDetails`) when
    /// receiving a completion item in a resolve call.
    ///
    /// @since 3.17.0
    pub label_details_support: Option<bool>,
}

pub type Options = ServerCompletionItemOptions;