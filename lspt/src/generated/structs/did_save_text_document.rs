// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in a save text document notification
pub struct DidSaveTextDocumentParams {
    /// The document that was saved.
    pub text_document: TextDocumentIdentifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional the content when saved. Depends on the includeText value
    /// when the save notification was requested.
    pub text: Option<String>,
}

pub type Params = DidSaveTextDocumentParams;