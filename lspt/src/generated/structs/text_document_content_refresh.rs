// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for the `workspace/textDocumentContent/refresh` request.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentRefreshParams {
    /// The uri of the text document to refresh.
    pub uri: Uri,
}

#[cfg(feature = "proposed")]
pub type Params = TextDocumentContentRefreshParams;