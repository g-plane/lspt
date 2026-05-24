// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientSemanticTokensRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client will send the `textDocument/semanticTokens/range` request if
    /// the server provides a corresponding handler.
    pub range: Option<ClientSemanticTokensRequestRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client will send the `textDocument/semanticTokens/full` request if
    /// the server provides a corresponding handler.
    pub full: Option<ClientSemanticTokensRequestFull>,
}

pub type Options = ClientSemanticTokensRequestOptions;