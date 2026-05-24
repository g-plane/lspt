// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

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


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ChangeAnnotationsSupportOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client groups edits with equal labels into tree nodes,
    /// for instance all edits labelled with "Changes in Strings" would
    /// be a tree node.
    pub groups_on_label: Option<bool>,
}

pub type ClientCapabilities = WorkspaceEditClientCapabilities;

pub type Options = ChangeAnnotationsSupportOptions;