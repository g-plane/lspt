// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The params sent in an open notebook document notification.
///
/// @since 3.17.0
pub struct DidOpenNotebookDocumentParams {
    /// The notebook document that got opened.
    pub notebook_document: NotebookDocument,

    /// The text documents that represent the content
    /// of a notebook cell.
    pub cell_text_documents: Vec<TextDocumentItem>,
}
