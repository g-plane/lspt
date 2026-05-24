// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a location inside a resource, such as a line
/// inside a text file.
pub struct Location {
    pub uri: Uri,

    pub range: Range,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a color range from a document.
pub struct ColorInformation {
    /// The range in the document where this color appears.
    pub range: Range,

    /// The actual color value for this color range.
    pub color: Color,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageActionItem {
    /// A short title like 'Retry', 'Open Log' etc.
    pub title: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A text edit applicable to a text document.
pub struct TextEdit {
    /// The range of the text document to be manipulated. To insert
    /// text into a document create a range where start === end.
    pub range: Range,

    /// The string to be inserted. For delete operations use an
    /// empty string.
    pub new_text: String,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents information about programming constructs like variables, classes,
/// interfaces etc.
pub struct SymbolInformation {
    /// The name of this symbol.
    pub name: String,

    /// The kind of this symbol.
    pub kind: SymbolKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this symbol.
    ///
    /// @since 3.16.0
    pub tags: Option<Vec<SymbolTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name of the symbol containing this symbol. This information is for
    /// user interface purposes (e.g. to render a qualifier in the user interface
    /// if necessary). It can't be used to re-infer a hierarchy for the document
    /// symbols.
    pub container_name: Option<String>,

    /// The location of this symbol. The location's range is used by a tool
    /// to reveal the location in the editor. If the symbol is selected in the
    /// tool the range's start information is used to position the cursor. So
    /// the range usually spans more than the actual symbol's name and does
    /// normally include things like visibility modifiers.
    ///
    /// The range doesn't have to denote a node range in the sense of an abstract
    /// syntax tree. It can therefore not be used to re-construct a hierarchy of
    /// the symbols.
    pub location: Location,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a reference to a command. Provides a title which
/// will be used to represent a command in the UI and, optionally,
/// an array of arguments which will be passed to the command handler
/// function when invoked.
pub struct Command {
    /// Title of the command, like `save`.
    pub title: String,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional tooltip.
    ///
    /// @since 3.18.0
    /// @proposed
    pub tooltip: Option<String>,

    /// The identifier of the actual command handler.
    pub command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Arguments that the command handler should be
    /// invoked with.
    pub arguments: Option<Vec<serde_json::Value>>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the connection of two locations. Provides additional metadata over normal {@link Location locations},
/// including an origin range.
pub struct LocationLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Span of the origin of this link.
    ///
    /// Used as the underlined span for mouse interaction. Defaults to the word range at
    /// the definition position.
    pub origin_selection_range: Option<Range>,

    /// The target resource identifier of this link.
    pub target_uri: Uri,

    /// The full target range of this link. If the target for example is a symbol then target range is the
    /// range enclosing this symbol not including leading/trailing whitespace but everything else
    /// like comments. This information is typically used to highlight the range in the editor.
    pub target_range: Range,

    /// The range that should be selected and revealed when this link is being followed, e.g the name of a function.
    /// Must be contained by the `targetRange`. See also `DocumentSymbol#range`
    pub target_selection_range: Range,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Copy)]
#[serde(rename_all = "camelCase")]
/// A range in a text document expressed as (zero-based) start and end positions.
///
/// If you want to specify a range that contains a line including the line ending
/// character(s) then use an end position denoting the start of the next line.
/// For example:
/// ```ts
/// {
///     start: { line: 5, character: 23 }
///     end : { line 6, character : 0 }
/// }
/// ```
pub struct Range {
    /// The range's start position.
    pub start: Position,

    /// The range's end position.
    pub end: Position,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a color in RGBA space.
pub struct Color {
    /// The red component of this color in the range [0-1].
    pub red: f32,

    /// The green component of this color in the range [0-1].
    pub green: f32,

    /// The blue component of this color in the range [0-1].
    pub blue: f32,

    /// The alpha component of this color in the range [0-1].
    pub alpha: f32,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, Copy, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
/// Position in a text document expressed as zero-based line and character
/// offset. Prior to 3.17 the offsets were always based on a UTF-16 string
/// representation. So a string of the form `a𐐀b` the character offset of the
/// character `a` is 0, the character offset of `𐐀` is 1 and the character
/// offset of b is 3 since `𐐀` is represented using two code units in UTF-16.
/// Since 3.17 clients and servers can agree on a different string encoding
/// representation (e.g. UTF-8). The client announces it's supported encoding
/// via the client capability [`general.positionEncodings`](https://microsoft.github.io/language-server-protocol/specifications/specification-current/#clientCapabilities).
/// The value is an array of position encodings the client supports, with
/// decreasing preference (e.g. the encoding at index `0` is the most preferred
/// one). To stay backwards compatible the only mandatory encoding is UTF-16
/// represented via the string `utf-16`. The server can pick one of the
/// encodings offered by the client and signals that encoding back to the
/// client via the initialize result's property
/// [`capabilities.positionEncoding`](https://microsoft.github.io/language-server-protocol/specifications/specification-current/#serverCapabilities). If the string value
/// `utf-16` is missing from the client's capability `general.positionEncodings`
/// servers can safely assume that the client supports UTF-16. If the server
/// omits the position encoding in its initialize result the encoding defaults
/// to the string value `utf-16`. Implementation considerations: since the
/// conversion from one encoding into another requires the content of the
/// file / line the conversion is best done where the file is read which is
/// usually on the server side.
///
/// Positions are line end character agnostic. So you can not specify a position
/// that denotes `\r|\n` or `\n|` where `|` represents the character offset.
///
/// @since 3.17.0 - support for negotiated position encoding.
pub struct Position {
    /// Line position in a document (zero-based).
    pub line: u32,

    /// Character offset on a line in a document (zero-based).
    ///
    /// The meaning of this offset is determined by the negotiated
    /// `PositionEncodingKind`.
    pub character: u32,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents information on a file/folder create.
///
/// @since 3.16.0
pub struct FileCreate {
    /// A file:// URI for the location of the file/folder being created.
    pub uri: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Additional information that describes document changes.
///
/// @since 3.16.0
pub struct ChangeAnnotation {
    /// A human-readable string describing the actual change. The string
    /// is rendered prominent in the user interface.
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A flag which indicates that user confirmation is needed
    /// before applying the change.
    pub needs_confirmation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string which is rendered less prominent in
    /// the user interface.
    pub description: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents information on a file/folder rename.
///
/// @since 3.16.0
pub struct FileRename {
    /// A file:// URI for the original location of the file/folder being renamed.
    pub old_uri: String,

    /// A file:// URI for the new location of the file/folder being renamed.
    pub new_uri: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents information on a file/folder delete.
///
/// @since 3.16.0
pub struct FileDelete {
    /// A file:// URI for the location of the file/folder being deleted.
    pub uri: String,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A `MarkupContent` literal represents a string value which content is interpreted base on its
/// kind flag. Currently the protocol supports `plaintext` and `markdown` as markup kinds.
///
/// If the kind is `markdown` then the value can contain fenced code blocks like in GitHub issues.
/// See https://help.github.com/articles/creating-and-highlighting-code-blocks/#syntax-highlighting
///
/// Here is an example how such a string can be constructed using JavaScript / TypeScript:
/// ```ts
/// let markdown: MarkdownContent = {
///  kind: MarkupKind.Markdown,
///  value: [
///    '# Header',
///    'Some text',
///    '```typescript',
///    'someCode();',
///    '```'
///  ].join('\n')
/// };
/// ```
///
/// *Please Note* that clients might sanitize the return markdown. A client could decide to
/// remove HTML from the markdown to avoid script execution.
pub struct MarkupContent {
    /// The type of the Markup
    pub kind: MarkupKind,

    /// The content itself
    pub value: String,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A previous result id in a workspace pull request.
///
/// @since 3.17.0
pub struct PreviousResultId {
    /// The URI for which the client knowns a
    /// result id.
    pub uri: Uri,

    /// The value of the previous result id.
    pub value: String,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A string value used as a snippet is a template which allows to insert text
/// and to control the editor cursor when insertion happens.
///
/// A snippet can define tab stops and placeholders with `$1`, `$2`
/// and `${3:foo}`. `$0` defines the final tab stop, it defaults to
/// the end of the snippet. Variables are defined with `$name` and
/// `${name:default value}`.
///
/// @since 3.18.0
/// @proposed
pub struct StringValue {
    /// The kind of string value.
    pub kind: String,

    /// The snippet string.
    pub value: String,
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


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An event describing a file change.
pub struct FileEvent {
    /// The file's uri.
    pub uri: Uri,

    #[serde(rename = "type")]
    /// The change type.
    pub ty: FileChangeType,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileSystemWatcher {
    /// The glob pattern to watch. See {@link GlobPattern glob pattern} for more detail.
    ///
    /// @since 3.17.0 support for relative patterns.
    pub glob_pattern: GlobPattern,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The kind of events of interest. If omitted it defaults
    /// to WatchKind.Create | WatchKind.Change | WatchKind.Delete
    /// which is 7.
    pub kind: Option<WatchKind>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A special text edit to provide an insert and a replace operation.
///
/// @since 3.16.0
pub struct InsertReplaceEdit {
    /// The string to be inserted.
    pub new_text: String,

    /// The range if the insert is requested
    pub insert: Range,

    /// The range if the replace is requested.
    pub replace: Range,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A base for all symbol information.
pub struct BaseSymbolInformation {
    /// The name of this symbol.
    pub name: String,

    /// The kind of this symbol.
    pub kind: SymbolKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this symbol.
    ///
    /// @since 3.16.0
    pub tags: Option<Vec<SymbolTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name of the symbol containing this symbol. This information is for
    /// user interface purposes (e.g. to render a qualifier in the user interface
    /// if necessary). It can't be used to re-infer a hierarchy for the document
    /// symbols.
    pub container_name: Option<String>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Location with only uri and does not include range.
///
/// @since 3.18.0
pub struct LocationUriOnly {
    pub uri: Uri,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A special text edit with an additional change annotation.
///
/// @since 3.16.0.
pub struct AnnotatedTextEdit {
    /// The range of the text document to be manipulated. To insert
    /// text into a document create a range where start === end.
    pub range: Range,

    /// The string to be inserted. For delete operations use an
    /// empty string.
    pub new_text: String,

    /// The actual identifier of the change annotation
    pub annotation_id: ChangeAnnotationIdentifier,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An interactive text edit.
///
/// @since 3.18.0
/// @proposed
pub struct SnippetTextEdit {
    /// The range of the text document to be manipulated.
    pub range: Range,

    /// The snippet to be inserted.
    pub snippet: StringValue,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The actual identifier of the snippet edit.
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A generic resource operation.
pub struct ResourceOperation {
    /// The resource operation kind.
    pub kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    pub annotation_id: Option<ChangeAnnotationIdentifier>,
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


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describes the currently selected completion item.
///
/// @since 3.18.0
/// @proposed
pub struct SelectedCompletionInfo {
    /// The range that will be replaced if this completion item is accepted.
    pub range: Range,

    /// The text the range will be replaced with if this completion is accepted.
    pub text: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Information about the client
///
/// @since 3.15.0
/// @since 3.18.0 ClientInfo type name added.
pub struct ClientInfo {
    /// The name of the client as defined by the client.
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client's version as defined by the client.
    pub version: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Defines the capabilities provided by the client.
pub struct ClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Workspace specific client capabilities.
    pub workspace: Option<WorkspaceClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text document specific client capabilities.
    pub text_document: Option<TextDocumentClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the notebook document support.
    ///
    /// @since 3.17.0
    pub notebook_document: Option<NotebookDocumentClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Window specific client capabilities.
    pub window: Option<WindowClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// General client capabilities.
    ///
    /// @since 3.16.0
    pub general: Option<GeneralClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Experimental client capabilities.
    pub experimental: Option<serde_json::Value>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Structure to capture a description for an error code.
///
/// @since 3.16.0
pub struct CodeDescription {
    /// An URI to open with more information about the diagnostic error.
    pub href: Uri,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Edit range variant that includes ranges for insert and replace operations.
///
/// @since 3.18.0
pub struct EditRangeWithInsertReplace {
    pub insert: Range,

    pub replace: Range,
}


#[deprecated = "use MarkupContent instead."]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
/// @deprecated use MarkupContent instead.
pub struct MarkedStringWithLanguage {
    pub language: String,

    pub value: String,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a parameter of a callable-signature. A parameter can
/// have a label and a doc-comment.
pub struct ParameterInformation {
    /// The label of this parameter information.
    ///
    /// Either a string or an inclusive start and exclusive end offsets within its containing
    /// signature label. (see SignatureInformation.label). The offsets are based on a UTF-16
    /// string representation as `Position` and `Range` does.
    ///
    /// To avoid ambiguities a server should use the [start, end] offset value instead of using
    /// a substring. Whether a client support this is controlled via `labelOffsetSupport` client
    /// capability.
    ///
    /// *Note*: a label of type string should be a substring of its containing signature label.
    /// Its intended use case is to highlight the parameter label part in the `SignatureInformation.label`.
    pub label: ParameterInformationLabel,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The human-readable doc-comment of this parameter. Will be shown
    /// in the UI but can be omitted.
    pub documentation: Option<ParameterInformationDocumentation>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A notebook cell text document filter denotes a cell text
/// document by different properties.
///
/// @since 3.17.0
pub struct NotebookCellTextDocumentFilter {
    /// A filter that matches against the notebook
    /// containing the notebook cell. If a string
    /// value is provided it matches against the
    /// notebook type. '*' matches every notebook.
    pub notebook: NotebookDocumentFilterNotebook,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A language id like `python`.
    ///
    /// Will be matched against the language id of the
    /// notebook cell document. '*' matches every language.
    pub language: Option<String>,
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


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A relative pattern is a helper to construct glob patterns that are matched
/// relatively to a base URI. The common value for a `baseUri` is a workspace
/// folder root, but it can be another absolute URI as well.
///
/// @since 3.17.0
pub struct RelativePattern {
    /// A workspace folder or a base URI to which this pattern will be matched
    /// against relatively.
    pub base_uri: RelativePatternBaseUri,

    /// The actual glob pattern;
    pub pattern: Pattern,
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


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCompletionItemOptionsKind {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The completion item kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    ///
    /// If this property is not present the client only supports
    /// the completion items kinds from `Text` to `Reference` as defined in
    /// the initial version of the protocol.
    pub value_set: Option<Vec<CompletionItemKind>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientSemanticTokensRequestFullDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client will send the `textDocument/semanticTokens/full/delta` request if
    /// the server provides a corresponding handler.
    pub delta: Option<bool>,
}

