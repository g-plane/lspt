// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A versioned notebook document identifier.
///
/// @since 3.17.0
pub struct VersionedNotebookDocumentIdentifier {
    /// The version number of this notebook document.
    pub version: i32,

    /// The notebook document's uri.
    pub uri: Uri,
}

pub type Identifier = VersionedNotebookDocumentIdentifier;