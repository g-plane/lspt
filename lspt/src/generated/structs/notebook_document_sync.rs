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


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A notebook cell.
///
/// A cell's document URI must be unique across ALL notebook
/// cells and can therefore be used to uniquely identify a
/// notebook cell or the cell's text document.
///
/// @since 3.17.0
pub struct NotebookCell {
    /// The cell's kind
    pub kind: NotebookCellKind,

    /// The URI of the cell's text document
    /// content.
    pub document: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional metadata stored with the cell.
    ///
    /// Note: should always be an object literal (e.g. LSPObject)
    pub metadata: Option<HashMap<String, serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional execution summary information
    /// if supported by the client.
    pub execution_summary: Option<ExecutionSummary>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionSummary {
    /// A strict monotonically increasing value
    /// indicating the execution order of a cell
    /// inside a notebook.
    pub execution_order: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the execution was successful or
    /// not if known by the client.
    pub success: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct NotebookCellLanguage {
    pub language: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A change describing how to move a `NotebookCell`
/// array from state S to S'.
///
/// @since 3.17.0
pub struct NotebookCellArrayChange {
    /// The start oftest of the cell that changed.
    pub start: u32,

    /// The deleted cells
    pub delete_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The new cells, if any
    pub cells: Option<Vec<NotebookCell>>,
}


mod raw {
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


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Registration options specific to a notebook.
    ///
    /// @since 3.17.0
    pub struct NotebookDocumentSyncRegistrationOptions {
        /// The notebooks to be synced
        pub notebook_selector: Vec<NotebookSelectorItem>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether save notification should be forwarded to
        /// the server. Will only be honored if mode === `notebook`.
        pub save: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The id used to register the request. The id can be used to deregister
        /// the request again. See also Registration#id.
        pub id: Option<String>,
    }


    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The params sent in a change notebook document notification.
    ///
    /// @since 3.17.0
    pub struct DidChangeNotebookDocumentParams {
        /// The notebook document that did change. The version number points
        /// to the version after all provided changes have been applied. If
        /// only the text document content of a cell changes the notebook version
        /// doesn't necessarily have to change.
        pub notebook_document: VersionedNotebookDocumentIdentifier,

        /// The actual changes to the notebook document.
        ///
        /// The changes describe single state changes to the notebook document.
        /// So if there are two changes c1 (at array index 0) and c2 (at array
        /// index 1) for a notebook in state S then c1 moves the notebook from
        /// S to S' and c2 from S' to S''. So c1 is computed on the state S and
        /// c2 is computed on the state S'.
        ///
        /// To mirror the content of a notebook using change events use the following approach:
        /// - start with the same initial content
        /// - apply the 'notebookDocument/didChange' notifications in the order you receive them.
        /// - apply the `NotebookChangeEvent`s in a single notification in the order
        ///   you receive them.
        pub change: NotebookDocumentChangeEvent,
    }


    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The params sent in a save notebook document notification.
    ///
    /// @since 3.17.0
    pub struct DidSaveNotebookDocumentParams {
        /// The notebook document that got saved.
        pub notebook_document: NotebookDocumentIdentifier,
    }


    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The params sent in a close notebook document notification.
    ///
    /// @since 3.17.0
    pub struct DidCloseNotebookDocumentParams {
        /// The notebook document that got closed.
        pub notebook_document: NotebookDocumentIdentifier,

        /// The text documents that represent the content
        /// of a notebook cell that got closed.
        pub cell_text_documents: Vec<TextDocumentIdentifier>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Options specific to a notebook plus its cells
    /// to be synced to the server.
    ///
    /// If a selector provides a notebook document
    /// filter but no cell selector all cells of a
    /// matching notebook document will be synced.
    ///
    /// If a selector provides no notebook document
    /// filter but only a cell selector all notebook
    /// document that contain at least one matching
    /// cell will be synced.
    ///
    /// @since 3.17.0
    pub struct NotebookDocumentSyncOptions {
        /// The notebooks to be synced
        pub notebook_selector: Vec<NotebookSelectorItem>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether save notification should be forwarded to
        /// the server. Will only be honored if mode === `notebook`.
        pub save: Option<bool>,
    }


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
    /// Notebook specific client capabilities.
    ///
    /// @since 3.17.0
    pub struct NotebookDocumentSyncClientCapabilities {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether implementation supports dynamic registration. If this is
        /// set to `true` the client supports the new
        /// `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
        /// return value for the corresponding server capability as well.
        pub dynamic_registration: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client supports sending execution summary data per cell.
        pub execution_summary_support: Option<bool>,
    }
}

pub type RegistrationOptions = raw::NotebookDocumentSyncRegistrationOptions;

pub type Options = raw::NotebookDocumentSyncOptions;

pub type VersionedIdentifier = raw::VersionedNotebookDocumentIdentifier;

pub type ChangeEvent = raw::NotebookDocumentChangeEvent;

pub type Identifier = raw::NotebookDocumentIdentifier;

pub type ClientCapabilities = raw::NotebookDocumentSyncClientCapabilities;

pub mod did {
    pub type OpenParams = super::raw::DidOpenNotebookDocumentParams;

    pub type ChangeParams = super::raw::DidChangeNotebookDocumentParams;

    pub type SaveParams = super::raw::DidSaveNotebookDocumentParams;

    pub type CloseParams = super::raw::DidCloseNotebookDocumentParams;
}

pub mod filter {
    pub mod with {
        pub type Notebook = super::super::raw::NotebookDocumentFilterWithNotebook;

        pub type Cells = super::super::raw::NotebookDocumentFilterWithCells;
    }
}

pub mod cell {
    pub type Changes = super::raw::NotebookDocumentCellChanges;

    pub type ChangeStructure = super::raw::NotebookDocumentCellChangeStructure;

    pub type ContentChanges = super::raw::NotebookDocumentCellContentChanges;
}