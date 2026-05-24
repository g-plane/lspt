// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A literal to identify a text document in the client.
pub struct TextDocumentIdentifier {
    /// The text document's uri.
    pub uri: Uri,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describes textual changes on a text document. A TextDocumentEdit describes all changes
/// on a document version Si and after they are applied move the document to version Si+1.
/// So the creator of a TextDocumentEdit doesn't need to sort the array of edits or do any
/// kind of ordering. However the edits must be non overlapping.
pub struct TextDocumentEdit {
    /// The text document to change.
    pub text_document: OptionalVersionedTextDocumentIdentifier,

    /// The edits to be applied.
    ///
    /// @since 3.16.0 - support for AnnotatedTextEdit. This is guarded using a
    /// client capability.
    ///
    /// @since 3.18.0 - support for SnippetTextEdit. This is guarded using a
    /// client capability.
    pub edits: Vec<TextDocumentEditEditsItem>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An item to transfer a text document from the client to the
/// server.
pub struct TextDocumentItem {
    /// The text document's uri.
    pub uri: Uri,

    /// The text document's language identifier.
    pub language_id: LanguageKind,

    /// The version number of this document (it will increase after each
    /// change, including undo/redo).
    pub version: i32,

    /// The content of the opened text document.
    pub text: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Text document specific client capabilities.
pub struct TextDocumentClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Defines which synchronization capabilities the client supports.
    pub synchronization: Option<TextDocumentSyncClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Defines which filters the client supports.
    ///
    /// @since 3.18.0
    pub filters: Option<TextDocumentFilterClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/completion` request.
    pub completion: Option<CompletionClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/hover` request.
    pub hover: Option<HoverClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/signatureHelp` request.
    pub signature_help: Option<SignatureHelpClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/declaration` request.
    ///
    /// @since 3.14.0
    pub declaration: Option<DeclarationClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/definition` request.
    pub definition: Option<DefinitionClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/typeDefinition` request.
    ///
    /// @since 3.6.0
    pub type_definition: Option<TypeDefinitionClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/implementation` request.
    ///
    /// @since 3.6.0
    pub implementation: Option<ImplementationClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/references` request.
    pub references: Option<ReferenceClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/documentHighlight` request.
    pub document_highlight: Option<DocumentHighlightClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/documentSymbol` request.
    pub document_symbol: Option<DocumentSymbolClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/codeAction` request.
    pub code_action: Option<CodeActionClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/codeLens` request.
    pub code_lens: Option<CodeLensClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/documentLink` request.
    pub document_link: Option<DocumentLinkClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/documentColor` and the
    /// `textDocument/colorPresentation` request.
    ///
    /// @since 3.6.0
    pub color_provider: Option<DocumentColorClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/formatting` request.
    pub formatting: Option<DocumentFormattingClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/rangeFormatting` request.
    pub range_formatting: Option<DocumentRangeFormattingClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/onTypeFormatting` request.
    pub on_type_formatting: Option<DocumentOnTypeFormattingClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/rename` request.
    pub rename: Option<RenameClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/foldingRange` request.
    ///
    /// @since 3.10.0
    pub folding_range: Option<FoldingRangeClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/selectionRange` request.
    ///
    /// @since 3.15.0
    pub selection_range: Option<SelectionRangeClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/publishDiagnostics` notification.
    pub publish_diagnostics: Option<PublishDiagnosticsClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the various call hierarchy requests.
    ///
    /// @since 3.16.0
    pub call_hierarchy: Option<CallHierarchyClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the various semantic token request.
    ///
    /// @since 3.16.0
    pub semantic_tokens: Option<SemanticTokensClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/linkedEditingRange` request.
    ///
    /// @since 3.16.0
    pub linked_editing_range: Option<LinkedEditingRangeClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client capabilities specific to the `textDocument/moniker` request.
    ///
    /// @since 3.16.0
    pub moniker: Option<MonikerClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the various type hierarchy requests.
    ///
    /// @since 3.17.0
    pub type_hierarchy: Option<TypeHierarchyClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/inlineValue` request.
    ///
    /// @since 3.17.0
    pub inline_value: Option<InlineValueClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `textDocument/inlayHint` request.
    ///
    /// @since 3.17.0
    pub inlay_hint: Option<InlayHintClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the diagnostic pull model.
    ///
    /// @since 3.17.0
    pub diagnostic: Option<DiagnosticClientCapabilities>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client capabilities specific to inline completions.
    ///
    /// @since 3.18.0
    /// @proposed
    pub inline_completion: Option<InlineCompletionClientCapabilities>,
}

pub type Identifier = TextDocumentIdentifier;

pub type ClientCapabilities = TextDocumentClientCapabilities;