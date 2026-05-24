// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameter of a `textDocument/prepareCallHierarchy` request.
///
/// @since 3.16.0
pub struct CallHierarchyPrepareParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}
