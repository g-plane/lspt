// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in an open text document notification
pub struct DidOpenTextDocumentParams {
    /// The document that was opened.
    pub text_document: TextDocumentItem,
}
