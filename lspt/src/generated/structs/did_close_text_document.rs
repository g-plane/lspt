// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in a close text document notification
pub struct DidCloseTextDocumentParams {
    /// The document that was closed.
    pub text_document: TextDocumentIdentifier,
}

pub type Params = DidCloseTextDocumentParams;