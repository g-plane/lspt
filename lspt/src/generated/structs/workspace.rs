// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A workspace folder inside a client.
pub struct WorkspaceFolder {
    /// The associated URI for this workspace folder.
    pub uri: Uri,

    /// The name of the workspace folder. Used to refer to this
    /// workspace folder in the user interface.
    pub name: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Defines workspace specific capabilities of the server.
///
/// @since 3.18.0
pub struct WorkspaceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server supports workspace folder.
    ///
    /// @since 3.6.0
    pub workspace_folders: Option<WorkspaceFoldersServerCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server is interested in notifications/requests for operations on files.
    ///
    /// @since 3.16.0
    pub file_operations: Option<FileOperationOptions>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server supports the `workspace/textDocumentContent` request.
    ///
    /// @since 3.18.0
    /// @proposed
    pub text_document_content: Option<WorkspaceTextDocumentContent>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Workspace specific client capabilities.
pub struct WorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports applying batch edits
    /// to the workspace by supporting the request
    /// 'workspace/applyEdit'
    pub apply_edit: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to `WorkspaceEdit`s.
    pub workspace_edit: Option<WorkspaceEditClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `workspace/didChangeConfiguration` notification.
    pub did_change_configuration: Option<DidChangeConfigurationClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `workspace/didChangeWatchedFiles` notification.
    pub did_change_watched_files: Option<DidChangeWatchedFilesClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `workspace/symbol` request.
    pub symbol: Option<WorkspaceSymbolClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `workspace/executeCommand` request.
    pub execute_command: Option<ExecuteCommandClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client has support for workspace folders.
    ///
    /// @since 3.6.0
    pub workspace_folders: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports `workspace/configuration` requests.
    ///
    /// @since 3.6.0
    pub configuration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the semantic token requests scoped to the
    /// workspace.
    ///
    /// @since 3.16.0.
    pub semantic_tokens: Option<SemanticTokensWorkspaceClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the code lens requests scoped to the
    /// workspace.
    ///
    /// @since 3.16.0.
    pub code_lens: Option<CodeLensWorkspaceClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client has support for file notifications/requests for user operations on files.
    ///
    /// Since 3.16.0
    pub file_operations: Option<FileOperationClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the inline values requests scoped to the
    /// workspace.
    ///
    /// @since 3.17.0.
    pub inline_value: Option<InlineValueWorkspaceClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the inlay hint requests scoped to the
    /// workspace.
    ///
    /// @since 3.17.0.
    pub inlay_hint: Option<InlayHintWorkspaceClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the diagnostic requests scoped to the
    /// workspace.
    ///
    /// @since 3.17.0.
    pub diagnostics: Option<DiagnosticWorkspaceClientCapabilities>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the folding range requests scoped to the workspace.
    ///
    /// @since 3.18.0
    /// @proposed
    pub folding_range: Option<FoldingRangeWorkspaceClientCapabilities>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `workspace/textDocumentContent` request.
    ///
    /// @since 3.18.0
    /// @proposed
    pub text_document_content: Option<TextDocumentContentClientCapabilities>,
}
