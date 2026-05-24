// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A text document identifier to optionally denote a specific version of a text document.
pub struct OptionalVersionedTextDocumentIdentifier {
    /// The text document's uri.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The version number of this document. If a versioned text document identifier
    /// is sent from the server to the client and the file is not open in the editor
    /// (the server has not received an open notification before) the server can send
    /// `null` to indicate that the version is unknown and the content on disk is the
    /// truth (as specified with document content ownership).
    pub version: Option<i32>,
}
