// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A notebook document.
///
/// @since 3.17.0
pub struct NotebookDocument {
    /// The notebook document's uri.
    pub uri: Uri,

    /// The type of the notebook.
    pub notebook_type: String,

    /// The version number of this document (it will increase after each
    /// change, including undo/redo).
    pub version: i32,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional metadata stored with the notebook
    /// document.
    ///
    /// Note: should always be an object literal (e.g. LSPObject)
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    /// The cells of a notebook.
    pub cells: Vec<NotebookCell>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A change event for a notebook document.
///
/// @since 3.17.0
pub struct NotebookDocumentChangeEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The changed meta data if any.
    ///
    /// Note: should always be an object literal (e.g. LSPObject)
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Changes to cells
    pub cells: Option<NotebookDocumentCellChanges>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A literal to identify a notebook document in the client.
///
/// @since 3.17.0
pub struct NotebookDocumentIdentifier {
    /// The notebook document's uri.
    pub uri: Uri,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct NotebookDocumentFilterWithNotebook {
    /// The notebook to be synced If a string
    /// value is provided it matches against the
    /// notebook type. '*' matches every notebook.
    pub notebook: NotebookDocumentFilterNotebook,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The cells of the matching notebook to be synced.
    pub cells: Option<Vec<NotebookCellLanguage>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct NotebookDocumentFilterWithCells {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The notebook to be synced If a string
    /// value is provided it matches against the
    /// notebook type. '*' matches every notebook.
    pub notebook: Option<NotebookDocumentFilterNotebook>,

    /// The cells of the matching notebook to be synced.
    pub cells: Vec<NotebookCellLanguage>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Cell changes to a notebook document.
///
/// @since 3.18.0
pub struct NotebookDocumentCellChanges {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Changes to the cell structure to add or
    /// remove cells.
    pub structure: Option<NotebookDocumentCellChangeStructure>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Changes to notebook cells properties like its
    /// kind, execution summary or metadata.
    pub data: Option<Vec<NotebookCell>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Changes to the text content of notebook cells.
    pub text_content: Option<Vec<NotebookDocumentCellContentChanges>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Structural changes to cells in a notebook document.
///
/// @since 3.18.0
pub struct NotebookDocumentCellChangeStructure {
    /// The change to the cell array.
    pub array: NotebookCellArrayChange,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional opened cell text documents.
    pub did_open: Option<Vec<TextDocumentItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional closed cell text documents.
    pub did_close: Option<Vec<TextDocumentIdentifier>>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Content changes to a cell in a notebook document.
///
/// @since 3.18.0
pub struct NotebookDocumentCellContentChanges {
    pub document: VersionedTextDocumentIdentifier,

    pub changes: Vec<TextDocumentContentChangeEvent>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Capabilities specific to the notebook document support.
///
/// @since 3.17.0
pub struct NotebookDocumentClientCapabilities {
    /// Capabilities specific to notebook document synchronization
    ///
    /// @since 3.17.0
    pub synchronization: NotebookDocumentSyncClientCapabilities,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A notebook document filter where `notebookType` is required field.
///
/// @since 3.18.0
pub struct NotebookDocumentFilterNotebookType {
    /// The type of the enclosing notebook.
    pub notebook_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
    pub scheme: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A glob pattern.
    pub pattern: Option<GlobPattern>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A notebook document filter where `scheme` is required field.
///
/// @since 3.18.0
pub struct NotebookDocumentFilterScheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The type of the enclosing notebook.
    pub notebook_type: Option<String>,

    /// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
    pub scheme: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A glob pattern.
    pub pattern: Option<GlobPattern>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A notebook document filter where `pattern` is required field.
///
/// @since 3.18.0
pub struct NotebookDocumentFilterPattern {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The type of the enclosing notebook.
    pub notebook_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
    pub scheme: Option<String>,

    /// A glob pattern.
    pub pattern: GlobPattern,
}
