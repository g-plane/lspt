// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The process Id of the parent process that started
    /// the server.
    ///
    /// Is `null` if the process has not been started by another process.
    /// If the parent process is not alive then the server should exit.
    pub process_id: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Information about the client
    ///
    /// @since 3.15.0
    pub client_info: Option<ClientInfo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The locale the client is currently showing the user interface
    /// in. This must not necessarily be the locale of the operating
    /// system.
    ///
    /// Uses IETF language tags as the value's syntax
    /// (See https://en.wikipedia.org/wiki/IETF_language_tag)
    ///
    /// @since 3.16.0
    pub locale: Option<String>,

    /// The capabilities provided by the client (editor or tool)
    pub capabilities: ClientCapabilities,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// User provided initialization options.
    pub initialization_options: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The initial trace setting. If omitted trace is disabled ('off').
    pub trace: Option<TraceValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The workspace folders configured in the client when the server starts.
    ///
    /// This property is only available if the client supports workspace folders.
    /// It can be `null` if the client supports workspace folders but none are
    /// configured.
    ///
    /// @since 3.6.0
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result returned from an initialize request.
pub struct InitializeResult {
    /// The capabilities the language server provides.
    pub capabilities: ServerCapabilities,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Information about the server.
    ///
    /// @since 3.15.0
    pub server_info: Option<ServerInfo>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFoldersInitializeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The workspace folders configured in the client when the server starts.
    ///
    /// This property is only available if the client supports workspace folders.
    /// It can be `null` if the client supports workspace folders but none are
    /// configured.
    ///
    /// @since 3.6.0
    pub workspace_folders: Option<Vec<WorkspaceFolder>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Defines the capabilities provided by a language
/// server.
pub struct ServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The position encoding the server picked from the encodings offered
    /// by the client via the client capability `general.positionEncodings`.
    ///
    /// If the client didn't provide any position encodings the only valid
    /// value that a server can return is 'utf-16'.
    ///
    /// If omitted it defaults to 'utf-16'.
    ///
    /// @since 3.17.0
    pub position_encoding: Option<PositionEncodingKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Defines how text documents are synced. Is either a detailed structure
    /// defining each notification or for backwards compatibility the
    /// TextDocumentSyncKind number.
    pub text_document_sync: Option<TextDocumentSync>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Defines how notebook documents are synced.
    ///
    /// @since 3.17.0
    pub notebook_document_sync: Option<NotebookDocumentSync>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides completion support.
    pub completion_provider: Option<CompletionOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides hover support.
    pub hover_provider: Option<HoverProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides signature help support.
    pub signature_help_provider: Option<SignatureHelpOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides Goto Declaration support.
    pub declaration_provider: Option<DeclarationProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides goto definition support.
    pub definition_provider: Option<DefinitionProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides Goto Type Definition support.
    pub type_definition_provider: Option<TypeDefinitionProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides Goto Implementation support.
    pub implementation_provider: Option<ImplementationProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides find references support.
    pub references_provider: Option<ReferencesProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document highlight support.
    pub document_highlight_provider: Option<DocumentHighlightProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document symbol support.
    pub document_symbol_provider: Option<DocumentSymbolProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides code actions. CodeActionOptions may only be
    /// specified if the client states that it supports
    /// `codeActionLiteralSupport` in its initial `initialize` request.
    pub code_action_provider: Option<CodeActionProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides code lens.
    pub code_lens_provider: Option<CodeLensOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document link support.
    pub document_link_provider: Option<DocumentLinkOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides color provider support.
    pub color_provider: Option<ColorProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides workspace symbol support.
    pub workspace_symbol_provider: Option<WorkspaceSymbolProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document formatting.
    pub document_formatting_provider: Option<DocumentFormattingProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document range formatting.
    pub document_range_formatting_provider: Option<DocumentRangeFormattingProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document formatting on typing.
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides rename support. RenameOptions may only be
    /// specified if the client states that it supports
    /// `prepareSupport` in its initial `initialize` request.
    pub rename_provider: Option<RenameProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides folding provider support.
    pub folding_range_provider: Option<FoldingRangeProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides selection range support.
    pub selection_range_provider: Option<SelectionRangeProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides execute command support.
    pub execute_command_provider: Option<ExecuteCommandOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides call hierarchy support.
    ///
    /// @since 3.16.0
    pub call_hierarchy_provider: Option<CallHierarchyProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides linked editing range support.
    ///
    /// @since 3.16.0
    pub linked_editing_range_provider: Option<LinkedEditingRangeProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides semantic tokens support.
    ///
    /// @since 3.16.0
    pub semantic_tokens_provider: Option<SemanticTokensProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides moniker support.
    ///
    /// @since 3.16.0
    pub moniker_provider: Option<MonikerProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides type hierarchy support.
    ///
    /// @since 3.17.0
    pub type_hierarchy_provider: Option<TypeHierarchyProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides inline values.
    ///
    /// @since 3.17.0
    pub inline_value_provider: Option<InlineValueProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides inlay hints.
    ///
    /// @since 3.17.0
    pub inlay_hint_provider: Option<InlayHintProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server has support for pull model diagnostics.
    ///
    /// @since 3.17.0
    pub diagnostic_provider: Option<DiagnosticProvider>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline completion options used during static registration.
    ///
    /// @since 3.18.0
    /// @proposed
    pub inline_completion_provider: Option<InlineCompletionProvider>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Workspace specific server capabilities.
    pub workspace: Option<WorkspaceOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Experimental server capabilities.
    pub experimental: Option<serde_json::Value>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Information about the server
///
/// @since 3.15.0
/// @since 3.18.0 ServerInfo type name added.
pub struct ServerInfo {
    /// The name of the server as defined by the server.
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server's version as defined by the server.
    pub version: Option<String>,
}

pub type Params = InitializeParams;

pub type Result = InitializeResult;

pub type WorkspaceFoldersParams = WorkspaceFoldersInitializeParams;