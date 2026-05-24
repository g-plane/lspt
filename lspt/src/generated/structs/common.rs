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


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Cancellation data returned from a diagnostic request.
///
/// @since 3.17.0
pub struct DiagnosticServerCancellationData {
    pub retrigger_request: bool,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The data type of the ResponseError if the
/// initialize request fails.
pub struct InitializeError {
    /// Indicates whether the client execute the following retry logic:
    /// (1) show the message provided by the ResponseError to the user
    /// (2) user selects retry or cancel
    /// (3) if user selected retry the initialize method is sent again.
    pub retry: bool,
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


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressBegin {
    pub kind: String,

    /// Mandatory title of the progress operation. Used to briefly inform about
    /// the kind of operation being performed.
    ///
    /// Examples: "Indexing" or "Linking dependencies".
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Controls if a cancel button should show to allow the user to cancel the
    /// long running operation. Clients that don't support cancellation are allowed
    /// to ignore the setting.
    pub cancellable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional, more detailed associated progress message. Contains
    /// complementary information to the `title`.
    ///
    /// Examples: "3/25 files", "project/src/module2", "node_modules/some_dep".
    /// If unset, the previous progress message (if any) is still valid.
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional progress percentage to display (value 100 is considered 100%).
    /// If not provided infinite progress is assumed and clients are allowed
    /// to ignore the `percentage` value in subsequent in report notifications.
    ///
    /// The value should be steadily rising. Clients are free to ignore values
    /// that are not following this rule. The value range is [0, 100].
    pub percentage: Option<u32>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressReport {
    pub kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Controls enablement state of a cancel button.
    ///
    /// Clients that don't support cancellation or don't support controlling the button's
    /// enablement state are allowed to ignore the property.
    pub cancellable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional, more detailed associated progress message. Contains
    /// complementary information to the `title`.
    ///
    /// Examples: "3/25 files", "project/src/module2", "node_modules/some_dep".
    /// If unset, the previous progress message (if any) is still valid.
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional progress percentage to display (value 100 is considered 100%).
    /// If not provided infinite progress is assumed and clients are allowed
    /// to ignore the `percentage` value in subsequent in report notifications.
    ///
    /// The value should be steadily rising. Clients are free to ignore values
    /// that are not following this rule. The value range is [0, 100]
    pub percentage: Option<u32>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressEnd {
    pub kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional, a final message indicating to for example indicate the outcome
    /// of the operation.
    pub message: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
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


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A literal to identify a text document in the client.
pub struct TextDocumentIdentifier {
    /// The text document's uri.
    pub uri: Uri,
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
/// Create file operation.
pub struct CreateFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    pub annotation_id: Option<ChangeAnnotationIdentifier>,

    /// A create
    pub kind: String,

    /// The resource to create.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional options
    pub options: Option<CreateFileOptions>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Rename file operation
pub struct RenameFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    pub annotation_id: Option<ChangeAnnotationIdentifier>,

    /// A rename
    pub kind: String,

    /// The old (existing) location.
    pub old_uri: Uri,

    /// The new location.
    pub new_uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rename options.
    pub options: Option<RenameFileOptions>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Delete file operation
pub struct DeleteFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    pub annotation_id: Option<ChangeAnnotationIdentifier>,

    /// A delete
    pub kind: String,

    /// The file to delete.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Delete options.
    pub options: Option<DeleteFileOptions>,
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
/// Provide inline value as text.
///
/// @since 3.17.0
pub struct InlineValueText {
    /// The document range for which the inline value applies.
    pub range: Range,

    /// The text of the inline value.
    pub text: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provide inline value through a variable lookup.
/// If only a range is specified, the variable name will be extracted from the underlying document.
/// An optional variable name can be used to override the extracted name.
///
/// @since 3.17.0
pub struct InlineValueVariableLookup {
    /// The document range for which the inline value applies.
    /// The range is used to extract the variable name from the underlying document.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// If specified the name of the variable to look up.
    pub variable_name: Option<String>,

    /// How to perform the lookup.
    pub case_sensitive_lookup: bool,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provide an inline value through an expression evaluation.
/// If only a range is specified, the expression will be extracted from the underlying document.
/// An optional expression can be used to override the extracted expression.
///
/// @since 3.17.0
pub struct InlineValueEvaluatableExpression {
    /// The document range for which the inline value applies.
    /// The range is used to extract the evaluatable expression from the underlying document.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// If specified the expression overrides the extracted expression.
    pub expression: Option<String>,
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


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A full diagnostic report with a set of related documents.
///
/// @since 3.17.0
pub struct RelatedFullDocumentDiagnosticReport {
    /// A full document diagnostic report.
    pub kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional result id. If provided it will
    /// be sent on the next diagnostic request for the
    /// same document.
    pub result_id: Option<String>,

    /// The actual items.
    pub items: Vec<Diagnostic>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Diagnostics of related documents. This information is useful
    /// in programming languages where code in a file A can generate
    /// diagnostics in a file B which A depends on. An example of
    /// such a language is C/C++ where marco definitions in a file
    /// a.cpp and result in errors in a header file b.hpp.
    ///
    /// @since 3.17.0
    pub related_documents: Option<HashMap<Uri, RelatedDocumentDiagnosticReport>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An unchanged diagnostic report with a set of related documents.
///
/// @since 3.17.0
pub struct RelatedUnchangedDocumentDiagnosticReport {
    /// A document diagnostic report indicating
    /// no changes to the last result. A server can
    /// only return `unchanged` if result ids are
    /// provided.
    pub kind: String,

    /// A result id which will be sent on the next
    /// diagnostic request for the same document.
    pub result_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Diagnostics of related documents. This information is useful
    /// in programming languages where code in a file A can generate
    /// diagnostics in a file B which A depends on. An example of
    /// such a language is C/C++ where marco definitions in a file
    /// a.cpp and result in errors in a header file b.hpp.
    ///
    /// @since 3.17.0
    pub related_documents: Option<HashMap<Uri, RelatedDocumentDiagnosticReport>>,
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


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A text document identifier to denote a specific version of a text document.
pub struct VersionedTextDocumentIdentifier {
    /// The text document's uri.
    pub uri: Uri,

    /// The version number of this document.
    pub version: i32,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Save options.
pub struct SaveOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client is supposed to include the content on save.
    pub include_text: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a diagnostic, such as a compiler error or warning. Diagnostic objects
/// are only valid in the scope of a resource.
pub struct Diagnostic {
    /// The range at which the message applies
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostic's severity. To avoid interpretation mismatches when a
    /// server is used with different clients it is highly recommended that servers
    /// always provide a severity value.
    pub severity: Option<DiagnosticSeverity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostic's code, which usually appear in the user interface.
    pub code: Option<DiagnosticCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional property to describe the error code.
    /// Requires the code field (above) to be present/not null.
    ///
    /// @since 3.16.0
    pub code_description: Option<CodeDescription>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string describing the source of this
    /// diagnostic, e.g. 'typescript' or 'super lint'. It usually
    /// appears in the user interface.
    pub source: Option<String>,

    /// The diagnostic's message. It usually appears in the user interface
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional metadata about the diagnostic.
    ///
    /// @since 3.15.0
    pub tags: Option<Vec<DiagnosticTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An array of related diagnostic information, e.g. when symbol-names within
    /// a scope collide all definitions can be marked via this property.
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved between a `textDocument/publishDiagnostics`
    /// notification and `textDocument/codeAction` request.
    ///
    /// @since 3.16.0
    pub data: Option<serde_json::Value>,
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


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Value-object describing what options formatting should use.
pub struct FormattingOptions {
    /// Size of a tab in spaces.
    pub tab_size: u32,

    /// Prefer spaces over tabs.
    pub insert_spaces: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Trim trailing whitespace on a line.
    ///
    /// @since 3.15.0
    pub trim_trailing_whitespace: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Insert a newline character at the end of the file if one does not exist.
    ///
    /// @since 3.15.0
    pub insert_final_newline: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Trim all newlines after the final newline at the end of the file.
    ///
    /// @since 3.15.0
    pub trim_final_newlines: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct PrepareRenamePlaceholder {
    pub range: Range,

    pub placeholder: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct PrepareRenameDefaultBehavior {
    pub default_behavior: bool,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A text document identifier to optionally denote a specific version of a text document.
pub struct OptionalVersionedTextDocumentIdentifier {
    /// The text document's uri.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The version number of this document. If a versioned text document identifier
    /// is sent from the server to the client and the file is not open in the editor
    /// (the server has not received an open notification before) the server can send
    /// `null` to indicate that the version is unknown and the content on disk is the
    /// truth (as specified with document content ownership).
    pub version: Option<i32>,
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


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Options to create a file.
pub struct CreateFileOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Overwrite existing file. Overwrite wins over `ignoreIfExists`
    pub overwrite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ignore if exists.
    pub ignore_if_exists: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Rename file options
pub struct RenameFileOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Overwrite target if existing. Overwrite wins over `ignoreIfExists`
    pub overwrite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ignores if target exists.
    pub ignore_if_exists: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Delete file options
pub struct DeleteFileOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Delete the content recursively if a folder is denoted.
    pub recursive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ignore the operation if the file doesn't exist.
    pub ignore_if_not_exists: Option<bool>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A full document diagnostic report for a workspace diagnostic result.
///
/// @since 3.17.0
pub struct WorkspaceFullDocumentDiagnosticReport {
    /// A full document diagnostic report.
    pub kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional result id. If provided it will
    /// be sent on the next diagnostic request for the
    /// same document.
    pub result_id: Option<String>,

    /// The actual items.
    pub items: Vec<Diagnostic>,

    /// The URI for which diagnostic information is reported.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The version number for which the diagnostics are reported.
    /// If the document is not marked as open `null` can be provided.
    pub version: Option<i32>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An unchanged document diagnostic report for a workspace diagnostic result.
///
/// @since 3.17.0
pub struct WorkspaceUnchangedDocumentDiagnosticReport {
    /// A document diagnostic report indicating
    /// no changes to the last result. A server can
    /// only return `unchanged` if result ids are
    /// provided.
    pub kind: String,

    /// A result id which will be sent on the next
    /// diagnostic request for the same document.
    pub result_id: String,

    /// The URI for which diagnostic information is reported.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The version number for which the diagnostics are reported.
    /// If the document is not marked as open `null` can be provided.
    pub version: Option<i32>,
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
/// @since 3.18.0
pub struct TextDocumentContentChangePartial {
    /// The range of the document that changed.
    pub range: Range,

    /// The new text for the provided range.
    pub text: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct TextDocumentContentChangeWholeDocument {
    /// The new text of the whole document.
    pub text: String,
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


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a related message and source code location for a diagnostic. This should be
/// used to point to code locations that cause or related to a diagnostics, e.g when duplicating
/// a symbol in a scope.
pub struct DiagnosticRelatedInformation {
    /// The location of this related diagnostic information.
    pub location: Location,

    /// The message of this related diagnostic information.
    pub message: String,
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
pub struct WindowClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// It indicates whether the client supports server initiated
    /// progress using the `window/workDoneProgress/create` request.
    ///
    /// The capability also controls Whether client supports handling
    /// of progress notifications. If set servers are allowed to report a
    /// `workDoneProgress` property in the request specific server
    /// capabilities.
    ///
    /// @since 3.15.0
    pub work_done_progress: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the showMessage request.
    ///
    /// @since 3.16.0
    pub show_message: Option<ShowMessageRequestClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the showDocument request.
    ///
    /// @since 3.16.0
    pub show_document: Option<ShowDocumentClientCapabilities>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// General client capabilities.
///
/// @since 3.16.0
pub struct GeneralClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client capability that signals how the client
    /// handles stale requests (e.g. a request
    /// for which the client will not process the response
    /// anymore since the information is outdated).
    ///
    /// @since 3.17.0
    pub stale_request_support: Option<StaleRequestSupportOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client capabilities specific to regular expressions.
    ///
    /// @since 3.16.0
    pub regular_expressions: Option<RegularExpressionsClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client capabilities specific to the client's markdown parser.
    ///
    /// @since 3.16.0
    pub markdown: Option<MarkdownClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The position encodings supported by the client. Client and server
    /// have to agree on the same position encoding to ensure that offsets
    /// (e.g. character position in a line) are interpreted the same on both
    /// sides.
    ///
    /// To keep the protocol backwards compatible the following applies: if
    /// the value 'utf-16' is missing from the array of position encodings
    /// servers can assume that the client supports UTF-16. UTF-16 is
    /// therefore a mandatory encoding.
    ///
    /// If omitted it defaults to ['utf-16'].
    ///
    /// Implementation considerations: since the conversion from one encoding
    /// into another requires the content of the file / line the conversion
    /// is best done where the file is read which is usually on the server
    /// side.
    ///
    /// @since 3.17.0
    pub position_encodings: Option<Vec<PositionEncodingKind>>,
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
/// A document filter where `language` is required field.
///
/// @since 3.18.0
pub struct TextDocumentFilterLanguage {
    /// A language id, like `typescript`.
    pub language: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
    pub scheme: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A glob pattern, like **​/*.{ts,js}. See TextDocumentFilter for examples.
    ///
    /// @since 3.18.0 - support for relative patterns. Whether clients support
    /// relative patterns depends on the client capability
    /// `textDocuments.filters.relativePatternSupport`.
    pub pattern: Option<GlobPattern>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A document filter where `scheme` is required field.
///
/// @since 3.18.0
pub struct TextDocumentFilterScheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A language id, like `typescript`.
    pub language: Option<String>,

    /// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
    pub scheme: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A glob pattern, like **​/*.{ts,js}. See TextDocumentFilter for examples.
    ///
    /// @since 3.18.0 - support for relative patterns. Whether clients support
    /// relative patterns depends on the client capability
    /// `textDocuments.filters.relativePatternSupport`.
    pub pattern: Option<GlobPattern>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A document filter where `pattern` is required field.
///
/// @since 3.18.0
pub struct TextDocumentFilterPattern {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A language id, like `typescript`.
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
    pub scheme: Option<String>,

    /// A glob pattern, like **​/*.{ts,js}. See TextDocumentFilter for examples.
    ///
    /// @since 3.18.0 - support for relative patterns. Whether clients support
    /// relative patterns depends on the client capability
    /// `textDocuments.filters.relativePatternSupport`.
    pub pattern: GlobPattern,
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


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientSymbolKindOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The symbol kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    ///
    /// If this property is not present the client only supports
    /// the symbol kinds from `File` to `Array` as defined in
    /// the initial version of the protocol.
    pub value_set: Option<Vec<SymbolKind>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientSymbolTagOptions {
    /// The tags supported by the client.
    pub value_set: Vec<SymbolTag>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// General diagnostics capabilities for pull and push model.
pub struct DiagnosticsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the clients accepts diagnostics with related information.
    pub related_information: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the tag property to provide meta data about a diagnostic.
    /// Clients supporting tags have to handle unknown tags gracefully.
    ///
    /// @since 3.15.0
    pub tag_support: Option<ClientDiagnosticsTagOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports a codeDescription property
    ///
    /// @since 3.16.0
    pub code_description_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports the `data` property which is
    /// preserved between a `textDocument/publishDiagnostics` and
    /// `textDocument/codeAction` request.
    ///
    /// @since 3.16.0
    pub data_support: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientDiagnosticsTagOptions {
    /// The tags supported by the client.
    pub value_set: Vec<DiagnosticTag>,
}

