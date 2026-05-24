// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The params sent in a save notebook document notification.
///
/// @since 3.17.0
pub struct DidSaveNotebookDocumentParams {
    /// The notebook document that got saved.
    pub notebook_document: NotebookDocumentIdentifier,
}
