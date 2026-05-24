// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A workspace edit represents changes to many resources managed in the workspace. The edit
/// should either provide `changes` or `documentChanges`. If documentChanges are present
/// they are preferred over `changes` if the client can handle versioned document edits.
///
/// Since version 3.13.0 a workspace edit can contain resource operations as well. If resource
/// operations are present clients need to execute the operations in the order in which they
/// are provided. So a workspace edit for example can consist of the following two changes:
/// (1) a create file a.txt and (2) a text document edit which insert text into file a.txt.
///
/// An invalid sequence (e.g. (1) delete file a.txt and (2) insert text into file a.txt) will
/// cause failure of the operation. How the client recovers from the failure is described by
/// the client capability: `workspace.workspaceEdit.failureHandling`
pub struct WorkspaceEdit {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Holds changes to existing resources.
    pub changes: Option<HashMap<Uri, Vec<TextEdit>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Depending on the client capability `workspace.workspaceEdit.resourceOperations` document changes
    /// are either an array of `TextDocumentEdit`s to express changes to n different text documents
    /// where each text document edit addresses a specific version of a text document. Or it can contain
    /// above `TextDocumentEdit`s mixed with create, rename and delete file / folder operations.
    ///
    /// Whether a client supports versioned document edits is expressed via
    /// `workspace.workspaceEdit.documentChanges` client capability.
    ///
    /// If a client neither supports `documentChanges` nor `workspace.workspaceEdit.resourceOperations` then
    /// only plain `TextEdit`s using the `changes` property are supported.
    pub document_changes: Option<Vec<WorkspaceEditDocumentChangesItem>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A map of change annotations that can be referenced in `AnnotatedTextEdit`s or create, rename and
    /// delete file / folder operations.
    ///
    /// Whether clients honor this property depends on the client capability `workspace.changeAnnotationSupport`.
    ///
    /// @since 3.16.0
    pub change_annotations: Option<HashMap<ChangeAnnotationIdentifier, ChangeAnnotation>>,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Additional data about a workspace edit.
///
/// @since 3.18.0
/// @proposed
pub struct WorkspaceEditMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Signal to the editor that this edit is a refactoring.
    pub is_refactoring: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceEditClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports versioned document changes in `WorkspaceEdit`s
    pub document_changes: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The resource operations the client supports. Clients should at least
    /// support 'create', 'rename' and 'delete' files and folders.
    ///
    /// @since 3.13.0
    pub resource_operations: Option<Vec<ResourceOperationKind>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The failure handling strategy of a client if applying the workspace edit
    /// fails.
    ///
    /// @since 3.13.0
    pub failure_handling: Option<FailureHandlingKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client normalizes line endings to the client specific
    /// setting.
    /// If set to `true` the client will normalize line ending characters
    /// in a workspace edit to the client-specified new line
    /// character.
    ///
    /// @since 3.16.0
    pub normalizes_line_endings: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client in general supports change annotations on text edits,
    /// create file, rename file and delete file changes.
    ///
    /// @since 3.16.0
    pub change_annotation_support: Option<ChangeAnnotationsSupportOptions>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports `WorkspaceEditMetadata` in `WorkspaceEdit`s.
    ///
    /// @since 3.18.0
    /// @proposed
    pub metadata_support: Option<bool>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports snippets as text edits.
    ///
    /// @since 3.18.0
    /// @proposed
    pub snippet_edit_support: Option<bool>,
}

pub type ClientCapabilities = WorkspaceEditClientCapabilities;