// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describe options to be used when registered for text document change events.
pub struct TextDocumentChangeRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    /// How documents are synced to the server.
    pub sync_kind: TextDocumentSyncKind,
}
