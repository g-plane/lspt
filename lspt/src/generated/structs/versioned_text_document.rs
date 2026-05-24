// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A text document identifier to denote a specific version of a text document.
pub struct VersionedTextDocumentIdentifier {
    /// The text document's uri.
    pub uri: Uri,

    /// The version number of this document.
    pub version: i32,
}

pub type Identifier = VersionedTextDocumentIdentifier;