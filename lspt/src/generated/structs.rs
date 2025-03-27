// DO NOT EDIT THIS GENERATED FILE.

use crate::{HashMap, Union2, Union3, Union4, Uri};
use serde::{Deserialize, Serialize};
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

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
pub struct ImplementationRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
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
/// The parameters of a `workspace/didChangeWorkspaceFolders` notification.
pub struct DidChangeWorkspaceFoldersParams {
    /// The actual workspace folder change event.
    pub event: WorkspaceFoldersChangeEvent,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a configuration request.
pub struct ConfigurationParams {
    pub items: Vec<ConfigurationItem>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link DocumentColorRequest}.
pub struct DocumentColorParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
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
pub struct DocumentColorRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link ColorPresentationRequest}.
pub struct ColorPresentationParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The color to request presentations for.
    pub color: Color,

    /// The range where the color would be inserted. Serves as a context.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorPresentation {
    /// The label of this color presentation. It will be shown on the color
    /// picker header. By default this is also the text that is inserted when selecting
    /// this color presentation.
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An {@link TextEdit edit} which is applied to a document when selecting
    /// this presentation for the color.  When `falsy` the {@link ColorPresentation.label label}
    /// is used.
    pub text_edit: Option<TextEdit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional array of additional {@link TextEdit text edits} that are applied when
    /// selecting this color presentation. Edits must not overlap with the main {@link ColorPresentation.textEdit edit} nor with themselves.
    pub additional_text_edits: Option<Vec<TextEdit>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link FoldingRangeRequest}.
pub struct FoldingRangeParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a folding range. To be valid, start and end line must be bigger than zero and smaller
/// than the number of lines in the document. Clients are free to ignore invalid ranges.
pub struct FoldingRange {
    /// The zero-based start line of the range to fold. The folded area starts after the line's last character.
    /// To be valid, the end must be zero or larger and smaller than the number of lines in the document.
    pub start_line: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The zero-based character offset from where the folded range starts. If not defined, defaults to the length of the start line.
    pub start_character: Option<u32>,

    /// The zero-based end line of the range to fold. The folded area ends with the line's last character.
    /// To be valid, the end must be zero or larger and smaller than the number of lines in the document.
    pub end_line: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The zero-based character offset before the folded range ends. If not defined, defaults to the length of the end line.
    pub end_character: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Describes the kind of the folding range such as 'comment' or 'region'. The kind
    /// is used to categorize folding ranges and used by commands like 'Fold all comments'.
    /// See {@link FoldingRangeKind} for an enumeration of standardized kinds.
    pub kind: Option<FoldingRangeKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The text that the client should show when the specified range is
    /// collapsed. If not defined or not supported by the client, a default
    /// will be chosen by the client.
    ///
    /// @since 3.17.0
    pub collapsed_text: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A parameter literal used in selection range requests.
pub struct SelectionRangeParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The positions inside the text document.
    pub positions: Vec<Position>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A selection range represents a part of a selection hierarchy. A selection range
/// may have a parent selection range that contains it.
pub struct SelectionRange {
    /// The {@link Range range} of this selection range.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The parent selection range containing this range. Therefore `parent.range` must contain `this.range`.
    pub parent: Option<Box<SelectionRange>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRangeRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressCreateParams {
    /// The token to be used to report progress.
    pub token: ProgressToken,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressCancelParams {
    /// The token to be used to report progress.
    pub token: ProgressToken,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameter of a `textDocument/prepareCallHierarchy` request.
///
/// @since 3.16.0
pub struct CallHierarchyPrepareParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents programming constructs like functions or constructors in the context
/// of call hierarchy.
///
/// @since 3.16.0
pub struct CallHierarchyItem {
    /// The name of this item.
    pub name: String,

    /// The kind of this item.
    pub kind: SymbolKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this item.
    pub tags: Option<Vec<SymbolTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// More detail for this item, e.g. the signature of a function.
    pub detail: Option<String>,

    /// The resource identifier of this item.
    pub uri: Uri,

    /// The range enclosing this symbol not including leading/trailing whitespace but everything else, e.g. comments and code.
    pub range: Range,

    /// The range that should be selected and revealed when this symbol is being picked, e.g. the name of a function.
    /// Must be contained by the {@link CallHierarchyItem.range `range`}.
    pub selection_range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved between a call hierarchy prepare and
    /// incoming calls or outgoing calls requests.
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Call hierarchy options used during static or dynamic registration.
///
/// @since 3.16.0
pub struct CallHierarchyRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameter of a `callHierarchy/incomingCalls` request.
///
/// @since 3.16.0
pub struct CallHierarchyIncomingCallsParams {
    pub item: CallHierarchyItem,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents an incoming call, e.g. a caller of a method or constructor.
///
/// @since 3.16.0
pub struct CallHierarchyIncomingCall {
    /// The item that makes the call.
    pub from: CallHierarchyItem,

    /// The ranges at which the calls appear. This is relative to the caller
    /// denoted by {@link CallHierarchyIncomingCall.from `this.from`}.
    pub from_ranges: Vec<Range>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameter of a `callHierarchy/outgoingCalls` request.
///
/// @since 3.16.0
pub struct CallHierarchyOutgoingCallsParams {
    pub item: CallHierarchyItem,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents an outgoing call, e.g. calling a getter from a method or a method from a constructor etc.
///
/// @since 3.16.0
pub struct CallHierarchyOutgoingCall {
    /// The item that is called.
    pub to: CallHierarchyItem,

    /// The range at which this item is called. This is the range relative to the caller, e.g the item
    /// passed to {@link CallHierarchyItemProvider.provideCallHierarchyOutgoingCalls `provideCallHierarchyOutgoingCalls`}
    /// and not {@link CallHierarchyOutgoingCall.to `this.to`}.
    pub from_ranges: Vec<Range>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokens {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional result id. If provided and clients support delta updating
    /// the client will include the result id in the next semantic token request.
    /// A server can then instead of computing all semantic tokens again simply
    /// send a delta.
    pub result_id: Option<String>,

    /// The actual tokens.
    pub data: Vec<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensPartialResult {
    pub data: Vec<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    /// The legend used by the server
    pub legend: SemanticTokensLegend,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Server supports providing semantic tokens for a specific range
    /// of a document.
    pub range: Option<Union2<bool, serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Server supports providing semantic tokens for a full document.
    pub full: Option<Union2<bool, SemanticTokensFullDelta>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensDeltaParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The result id of a previous response. The result Id can either point to a full response
    /// or a delta response depending on what was received last.
    pub previous_result_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_id: Option<String>,

    /// The semantic token edits to transform a previous result into a new result.
    pub edits: Vec<SemanticTokensEdit>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensDeltaPartialResult {
    pub edits: Vec<SemanticTokensEdit>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensRangeParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The range the semantic tokens are requested for.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Params to show a resource in the UI.
///
/// @since 3.16.0
pub struct ShowDocumentParams {
    /// The uri to show.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates to show the resource in an external program.
    /// To show, for example, `https://code.visualstudio.com/`
    /// in the default WEB browser set `external` to `true`.
    pub external: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional property to indicate whether the editor
    /// showing the document should take focus or not.
    /// Clients might ignore this property if an external
    /// program is started.
    pub take_focus: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional selection range if the document is a text
    /// document. Clients might ignore the property if an
    /// external program is started or the file is not a text
    /// file.
    pub selection: Option<Range>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result of a showDocument request.
///
/// @since 3.16.0
pub struct ShowDocumentResult {
    /// A boolean indicating if the show was successful.
    pub success: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangeParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result of a linked editing range request.
///
/// @since 3.16.0
pub struct LinkedEditingRanges {
    /// A list of ranges that can be edited together. The ranges must have
    /// identical length and contain identical text content. The ranges cannot overlap.
    pub ranges: Vec<Range>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional word pattern (regular expression) that describes valid contents for
    /// the given ranges. If no pattern is provided, the client configuration's word
    /// pattern will be used.
    pub word_pattern: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangeRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in notifications/requests for user-initiated creation of
/// files.
///
/// @since 3.16.0
pub struct CreateFilesParams {
    /// An array of all files/folders created in this operation.
    pub files: Vec<FileCreate>,
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
    pub document_changes: Option<Vec<Union4<TextDocumentEdit, CreateFile, RenameFile, DeleteFile>>>,

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
/// The options to register for file operations.
///
/// @since 3.16.0
pub struct FileOperationRegistrationOptions {
    /// The actual filters.
    pub filters: Vec<FileOperationFilter>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in notifications/requests for user-initiated renames of
/// files.
///
/// @since 3.16.0
pub struct RenameFilesParams {
    /// An array of all files/folders renamed in this operation. When a folder is renamed, only
    /// the folder will be included, and not its children.
    pub files: Vec<FileRename>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in notifications/requests for user-initiated deletes of
/// files.
///
/// @since 3.16.0
pub struct DeleteFilesParams {
    /// An array of all files/folders deleted in this operation.
    pub files: Vec<FileDelete>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonikerParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Moniker definition to match LSIF 0.5 moniker definition.
///
/// @since 3.16.0
pub struct Moniker {
    /// The scheme of the moniker. For example tsc or .Net
    pub scheme: String,

    /// The identifier of the moniker. The value is opaque in LSIF however
    /// schema owners are allowed to define the structure if they want.
    pub identifier: String,

    /// The scope in which the moniker is unique
    pub unique: UniquenessLevel,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The moniker kind if known.
    pub kind: Option<MonikerKind>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonikerRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameter of a `textDocument/prepareTypeHierarchy` request.
///
/// @since 3.17.0
pub struct TypeHierarchyPrepareParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.17.0
pub struct TypeHierarchyItem {
    /// The name of this item.
    pub name: String,

    /// The kind of this item.
    pub kind: SymbolKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this item.
    pub tags: Option<Vec<SymbolTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// More detail for this item, e.g. the signature of a function.
    pub detail: Option<String>,

    /// The resource identifier of this item.
    pub uri: Uri,

    /// The range enclosing this symbol not including leading/trailing whitespace
    /// but everything else, e.g. comments and code.
    pub range: Range,

    /// The range that should be selected and revealed when this symbol is being
    /// picked, e.g. the name of a function. Must be contained by the
    /// {@link TypeHierarchyItem.range `range`}.
    pub selection_range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved between a type hierarchy prepare and
    /// supertypes or subtypes requests. It could also be used to identify the
    /// type hierarchy in the server, helping improve the performance on
    /// resolving supertypes and subtypes.
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Type hierarchy options used during static or dynamic registration.
///
/// @since 3.17.0
pub struct TypeHierarchyRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameter of a `typeHierarchy/supertypes` request.
///
/// @since 3.17.0
pub struct TypeHierarchySupertypesParams {
    pub item: TypeHierarchyItem,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameter of a `typeHierarchy/subtypes` request.
///
/// @since 3.17.0
pub struct TypeHierarchySubtypesParams {
    pub item: TypeHierarchyItem,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A parameter literal used in inline value requests.
///
/// @since 3.17.0
pub struct InlineValueParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The document range for which inline values should be computed.
    pub range: Range,

    /// Additional information about the context in which inline values were
    /// requested.
    pub context: InlineValueContext,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inline value options used during static or dynamic registration.
///
/// @since 3.17.0
pub struct InlineValueRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A parameter literal used in inlay hint requests.
///
/// @since 3.17.0
pub struct InlayHintParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The document range for which inlay hints should be computed.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inlay hint information.
///
/// @since 3.17.0
pub struct InlayHint {
    /// The position of this hint.
    ///
    /// If multiple hints have the same position, they will be shown in the order
    /// they appear in the response.
    pub position: Position,

    /// The label of this hint. A human readable string or an array of
    /// InlayHintLabelPart label parts.
    ///
    /// *Note* that neither the string nor the label part can be empty.
    pub label: Union2<String, Vec<InlayHintLabelPart>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The kind of this hint. Can be omitted in which case the client
    /// should fall back to a reasonable default.
    pub kind: Option<InlayHintKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional text edits that are performed when accepting this inlay hint.
    ///
    /// *Note* that edits are expected to change the document so that the inlay
    /// hint (or its nearest variant) is now part of the document and the inlay
    /// hint itself is now obsolete.
    pub text_edits: Option<Vec<TextEdit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The tooltip text when you hover over this item.
    pub tooltip: Option<Union2<String, MarkupContent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Render padding before the hint.
    ///
    /// Note: Padding should use the editor's background color, not the
    /// background color of the hint itself. That means padding can be used
    /// to visually align/separate an inlay hint.
    pub padding_left: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Render padding after the hint.
    ///
    /// Note: Padding should use the editor's background color, not the
    /// background color of the hint itself. That means padding can be used
    /// to visually align/separate an inlay hint.
    pub padding_right: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on an inlay hint between
    /// a `textDocument/inlayHint` and a `inlayHint/resolve` request.
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inlay hint options used during static or dynamic registration.
///
/// @since 3.17.0
pub struct InlayHintRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for an inlay hint item.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters of the document diagnostic request.
///
/// @since 3.17.0
pub struct DocumentDiagnosticParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The additional identifier  provided during registration.
    pub identifier: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The result id of a previous response if provided.
    pub previous_result_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A partial result for a document diagnostic report.
///
/// @since 3.17.0
pub struct DocumentDiagnosticReportPartialResult {
    pub related_documents: HashMap<Uri, Union2<FullDocumentDiagnosticReport, UnchangedDocumentDiagnosticReport>>,
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
/// Diagnostic registration options.
///
/// @since 3.17.0
pub struct DiagnosticRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional identifier under which the diagnostics are
    /// managed by the client.
    pub identifier: Option<String>,

    /// Whether the language has inter file dependencies meaning that
    /// editing code in one file can result in a different diagnostic
    /// set in another file. Inter file dependencies are common for
    /// most programming languages and typically uncommon for linters.
    pub inter_file_dependencies: bool,

    /// The server provides support for workspace diagnostics as well.
    pub workspace_diagnostics: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters of the workspace diagnostic request.
///
/// @since 3.17.0
pub struct WorkspaceDiagnosticParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The additional identifier provided during registration.
    pub identifier: Option<String>,

    /// The currently known diagnostic reports with their
    /// previous result ids.
    pub previous_result_ids: Vec<PreviousResultId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A workspace diagnostic report.
///
/// @since 3.17.0
pub struct WorkspaceDiagnosticReport {
    pub items: Vec<WorkspaceDocumentDiagnosticReport>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A partial result for a workspace diagnostic report.
///
/// @since 3.17.0
pub struct WorkspaceDiagnosticReportPartialResult {
    pub items: Vec<WorkspaceDocumentDiagnosticReport>,
}

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
    pub notebook_selector: Vec<Union2<NotebookDocumentFilterWithNotebook, NotebookDocumentFilterWithCells>>,

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

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A parameter literal used in inline completion requests.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    /// Additional information about the context in which inline completions were
    /// requested.
    pub context: InlineCompletionContext,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a collection of {@link InlineCompletionItem inline completion items} to be presented in the editor.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionList {
    /// The inline completion items
    pub items: Vec<InlineCompletionItem>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An inline completion item represents a text snippet that is proposed inline to complete text that is being typed.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionItem {
    /// The text to replace the range with. Must be set.
    pub insert_text: Union2<String, StringValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A text that is used to decide if this inline completion should be shown. When `falsy` the {@link InlineCompletionItem.insertText} is used.
    pub filter_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The range to replace. Must begin and end on the same line.
    pub range: Option<Range>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional {@link Command} that is executed *after* inserting this completion.
    pub command: Option<Command>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inline completion options used during static or dynamic registration.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentParams {
    /// The uri of the text document.
    pub uri: Uri,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Result of the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentResult {
    /// The text content of the text document. Please note, that the content of
    /// any subsequent open notifications for the text document might differ
    /// from the returned content due to whitespace and line ending
    /// normalizations done on the client
    pub text: String,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Text document content provider registration options.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentRegistrationOptions {
    /// The schemes for which the server provides content.
    pub schemes: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for the `workspace/textDocumentContent/refresh` request.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentRefreshParams {
    /// The uri of the text document to refresh.
    pub uri: Uri,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationParams {
    pub registrations: Vec<Registration>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnregistrationParams {
    pub unregisterations: Vec<Unregistration>,
}

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

    #[deprecated = "in favour of rootUri."]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The rootPath of the workspace. Is null
    /// if no folder is open.
    ///
    /// @deprecated in favour of rootUri.
    pub root_path: Option<String>,

    #[deprecated = "in favour of workspaceFolders."]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The rootUri of the workspace. Is null if no
    /// folder is open. If both `rootPath` and `rootUri` are set
    /// `rootUri` wins.
    ///
    /// @deprecated in favour of workspaceFolders.
    pub root_uri: Option<Uri>,

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
pub struct InitializedParams {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a change configuration notification.
pub struct DidChangeConfigurationParams {
    /// The actual changed settings
    pub settings: serde_json::Value,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeConfigurationRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<Union2<String, Vec<String>>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a notification message.
pub struct ShowMessageParams {
    #[serde(rename = "type")]
    /// The message type. See {@link MessageType}
    pub ty: MessageType,

    /// The actual message.
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShowMessageRequestParams {
    #[serde(rename = "type")]
    /// The message type. See {@link MessageType}
    pub ty: MessageType,

    /// The actual message.
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The message action items to present.
    pub actions: Option<Vec<MessageActionItem>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageActionItem {
    /// A short title like 'Retry', 'Open Log' etc.
    pub title: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The log message parameters.
pub struct LogMessageParams {
    #[serde(rename = "type")]
    /// The message type. See {@link MessageType}
    pub ty: MessageType,

    /// The actual message.
    pub message: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in an open text document notification
pub struct DidOpenTextDocumentParams {
    /// The document that was opened.
    pub text_document: TextDocumentItem,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The change text document notification's parameters.
pub struct DidChangeTextDocumentParams {
    /// The document that did change. The version number points
    /// to the version after all provided content changes have
    /// been applied.
    pub text_document: VersionedTextDocumentIdentifier,

    /// The actual content changes. The content changes describe single state changes
    /// to the document. So if there are two content changes c1 (at array index 0) and
    /// c2 (at array index 1) for a document in state S then c1 moves the document from
    /// S to S' and c2 from S' to S''. So c1 is computed on the state S and c2 is computed
    /// on the state S'.
    ///
    /// To mirror the content of a document using change events use the following approach:
    /// - start with the same initial content
    /// - apply the 'textDocument/didChange' notifications in the order you receive them.
    /// - apply the `TextDocumentContentChangeEvent`s in a single notification in the order
    ///   you receive them.
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describe options to be used when registered for text document change events.
pub struct TextDocumentChangeRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    /// How documents are synced to the server.
    pub sync_kind: TextDocumentSyncKind,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in a close text document notification
pub struct DidCloseTextDocumentParams {
    /// The document that was closed.
    pub text_document: TextDocumentIdentifier,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in a save text document notification
pub struct DidSaveTextDocumentParams {
    /// The document that was saved.
    pub text_document: TextDocumentIdentifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional the content when saved. Depends on the includeText value
    /// when the save notification was requested.
    pub text: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Save registration options.
pub struct TextDocumentSaveRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client is supposed to include the content on save.
    pub include_text: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in a will save text document notification.
pub struct WillSaveTextDocumentParams {
    /// The document that will be saved.
    pub text_document: TextDocumentIdentifier,

    /// The 'TextDocumentSaveReason'.
    pub reason: TextDocumentSaveReason,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The watched files change notification's parameters.
pub struct DidChangeWatchedFilesParams {
    /// The actual file events.
    pub changes: Vec<FileEvent>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describe options to be used when registered for text document change events.
pub struct DidChangeWatchedFilesRegistrationOptions {
    /// The watchers to register.
    pub watchers: Vec<FileSystemWatcher>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The publish diagnostic notification's parameters.
pub struct PublishDiagnosticsParams {
    /// The URI for which diagnostic information is reported.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional the version number of the document the diagnostics are published for.
    ///
    /// @since 3.15.0
    pub version: Option<i32>,

    /// An array of diagnostic information items.
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Completion parameters
pub struct CompletionParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The completion context. This is only available it the client specifies
    /// to send this using the client capability `textDocument.completion.contextSupport === true`
    pub context: Option<CompletionContext>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A completion item represents a text snippet that is
/// proposed to complete text that is being typed.
pub struct CompletionItem {
    /// The label of this completion item.
    ///
    /// The label property is also by default the text that
    /// is inserted when selecting this completion.
    ///
    /// If label details are provided the label itself should
    /// be an unqualified name of the completion item.
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional details for the label
    ///
    /// @since 3.17.0
    pub label_details: Option<CompletionItemLabelDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The kind of this completion item. Based of the kind
    /// an icon is chosen by the editor.
    pub kind: Option<CompletionItemKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this completion item.
    ///
    /// @since 3.15.0
    pub tags: Option<Vec<CompletionItemTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string with additional information
    /// about this item, like type or symbol information.
    pub detail: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string that represents a doc-comment.
    pub documentation: Option<Union2<String, MarkupContent>>,

    #[deprecated = "Use `tags` instead."]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if this item is deprecated.
    /// @deprecated Use `tags` instead.
    pub deprecated: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Select this item when showing.
    ///
    /// *Note* that only one completion item can be selected and that the
    /// tool / client decides which item that is. The rule is that the *first*
    /// item of those that match best is selected.
    pub preselect: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A string that should be used when comparing this item
    /// with other items. When `falsy` the {@link CompletionItem.label label}
    /// is used.
    pub sort_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A string that should be used when filtering a set of
    /// completion items. When `falsy` the {@link CompletionItem.label label}
    /// is used.
    pub filter_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A string that should be inserted into a document when selecting
    /// this completion. When `falsy` the {@link CompletionItem.label label}
    /// is used.
    ///
    /// The `insertText` is subject to interpretation by the client side.
    /// Some tools might not take the string literally. For example
    /// VS Code when code complete is requested in this example
    /// `con<cursor position>` and a completion item with an `insertText` of
    /// `console` is provided it will only insert `sole`. Therefore it is
    /// recommended to use `textEdit` instead since it avoids additional client
    /// side interpretation.
    pub insert_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The format of the insert text. The format applies to both the
    /// `insertText` property and the `newText` property of a provided
    /// `textEdit`. If omitted defaults to `InsertTextFormat.PlainText`.
    ///
    /// Please note that the insertTextFormat doesn't apply to
    /// `additionalTextEdits`.
    pub insert_text_format: Option<InsertTextFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// How whitespace and indentation is handled during completion
    /// item insertion. If not provided the clients default value depends on
    /// the `textDocument.completion.insertTextMode` client capability.
    ///
    /// @since 3.16.0
    pub insert_text_mode: Option<InsertTextMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An {@link TextEdit edit} which is applied to a document when selecting
    /// this completion. When an edit is provided the value of
    /// {@link CompletionItem.insertText insertText} is ignored.
    ///
    /// Most editors support two different operations when accepting a completion
    /// item. One is to insert a completion text and the other is to replace an
    /// existing text with a completion text. Since this can usually not be
    /// predetermined by a server it can report both ranges. Clients need to
    /// signal support for `InsertReplaceEdits` via the
    /// `textDocument.completion.insertReplaceSupport` client capability
    /// property.
    ///
    /// *Note 1:* The text edit's range as well as both ranges from an insert
    /// replace edit must be a [single line] and they must contain the position
    /// at which completion has been requested.
    /// *Note 2:* If an `InsertReplaceEdit` is returned the edit's insert range
    /// must be a prefix of the edit's replace range, that means it must be
    /// contained and starting at the same position.
    ///
    /// @since 3.16.0 additional type `InsertReplaceEdit`
    pub text_edit: Option<Union2<TextEdit, InsertReplaceEdit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The edit text used if the completion item is part of a CompletionList and
    /// CompletionList defines an item default for the text edit range.
    ///
    /// Clients will only honor this property if they opt into completion list
    /// item defaults using the capability `completionList.itemDefaults`.
    ///
    /// If not provided and a list's default range is provided the label
    /// property is used as a text.
    ///
    /// @since 3.17.0
    pub text_edit_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional array of additional {@link TextEdit text edits} that are applied when
    /// selecting this completion. Edits must not overlap (including the same insert position)
    /// with the main {@link CompletionItem.textEdit edit} nor with themselves.
    ///
    /// Additional text edits should be used to change text unrelated to the current cursor position
    /// (for example adding an import statement at the top of the file if the completion item will
    /// insert an unqualified type).
    pub additional_text_edits: Option<Vec<TextEdit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional set of characters that when pressed while this completion is active will accept it first and
    /// then type that character. *Note* that all commit characters should have `length=1` and that superfluous
    /// characters will be ignored.
    pub commit_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional {@link Command command} that is executed *after* inserting this completion. *Note* that
    /// additional modifications to the current document should be described with the
    /// {@link CompletionItem.additionalTextEdits additionalTextEdits}-property.
    pub command: Option<Command>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on a completion item between a
    /// {@link CompletionRequest} and a {@link CompletionResolveRequest}.
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a collection of {@link CompletionItem completion items} to be presented
/// in the editor.
pub struct CompletionList {
    /// This list it not complete. Further typing results in recomputing this list.
    ///
    /// Recomputed lists have all their items replaced (not appended) in the
    /// incomplete completion sessions.
    pub is_incomplete: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// In many cases the items of an actual completion result share the same
    /// value for properties like `commitCharacters` or the range of a text
    /// edit. A completion list can therefore define item defaults which will
    /// be used if a completion item itself doesn't specify the value.
    ///
    /// If a completion list specifies a default value and a completion item
    /// also specifies a corresponding value, the rules for combining these are
    /// defined by `applyKinds` (if the client supports it), defaulting to
    /// ApplyKind.Replace.
    ///
    /// Servers are only allowed to return default values if the client
    /// signals support for this via the `completionList.itemDefaults`
    /// capability.
    ///
    /// @since 3.17.0
    pub item_defaults: Option<CompletionItemDefaults>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies how fields from a completion item should be combined with those
    /// from `completionList.itemDefaults`.
    ///
    /// If unspecified, all fields will be treated as ApplyKind.Replace.
    ///
    /// If a field's value is ApplyKind.Replace, the value from a completion item
    /// (if provided and not `null`) will always be used instead of the value
    /// from `completionItem.itemDefaults`.
    ///
    /// If a field's value is ApplyKind.Merge, the values will be merged using
    /// the rules defined against each field below.
    ///
    /// Servers are only allowed to return `applyKind` if the client
    /// signals support for this via the `completionList.applyKindSupport`
    /// capability.
    ///
    /// @since 3.18.0
    pub apply_kind: Option<CompletionItemApplyKinds>,

    /// The completion items.
    pub items: Vec<CompletionItem>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link CompletionRequest}.
pub struct CompletionRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Most tools trigger completion request automatically without explicitly requesting
    /// it using a keyboard shortcut (e.g. Ctrl+Space). Typically they do so when the user
    /// starts to type an identifier. For example if the user types `c` in a JavaScript file
    /// code complete will automatically pop up present `console` besides others as a
    /// completion item. Characters that make up identifiers don't need to be listed here.
    ///
    /// If code complete should automatically be trigger on characters not being valid inside
    /// an identifier (for example `.` in JavaScript) list them in `triggerCharacters`.
    pub trigger_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The list of all possible characters that commit a completion. This field can be used
    /// if clients don't support individual commit characters per completion item. See
    /// `ClientCapabilities.textDocument.completion.completionItem.commitCharactersSupport`
    ///
    /// If a server provides both `allCommitCharacters` and commit characters on an individual
    /// completion item the ones on the completion item win.
    ///
    /// @since 3.2.0
    pub all_commit_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a completion item.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server supports the following `CompletionItem` specific
    /// capabilities.
    ///
    /// @since 3.17.0
    pub completion_item: Option<ServerCompletionItemOptions>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link HoverRequest}.
pub struct HoverParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result of a hover request.
pub struct Hover {
    /// The hover's content
    pub contents: Union3<MarkupContent, MarkedString, Vec<MarkedString>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional range inside the text document that is used to
    /// visualize the hover, e.g. by changing the background color.
    pub range: Option<Range>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link HoverRequest}.
pub struct HoverRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link SignatureHelpRequest}.
pub struct SignatureHelpParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The signature help context. This is only available if the client specifies
    /// to send this using the client capability `textDocument.signatureHelp.contextSupport === true`
    ///
    /// @since 3.15.0
    pub context: Option<SignatureHelpContext>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Signature help represents the signature of something
/// callable. There can be multiple signature but only one
/// active and only one active parameter.
pub struct SignatureHelp {
    /// One or more signatures.
    pub signatures: Vec<SignatureInformation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The active signature. If omitted or the value lies outside the
    /// range of `signatures` the value defaults to zero or is ignored if
    /// the `SignatureHelp` has no signatures.
    ///
    /// Whenever possible implementors should make an active decision about
    /// the active signature and shouldn't rely on a default value.
    ///
    /// In future version of the protocol this property might become
    /// mandatory to better express this.
    pub active_signature: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The active parameter of the active signature.
    ///
    /// If `null`, no parameter of the signature is active (for example a named
    /// argument that does not match any declared parameters). This is only valid
    /// if the client specifies the client capability
    /// `textDocument.signatureHelp.noActiveParameterSupport === true`
    ///
    /// If omitted or the value lies outside the range of
    /// `signatures[activeSignature].parameters` defaults to 0 if the active
    /// signature has parameters.
    ///
    /// If the active signature has no parameters it is ignored.
    ///
    /// In future version of the protocol this property might become
    /// mandatory (but still nullable) to better express the active parameter if
    /// the active signature does have any.
    pub active_parameter: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link SignatureHelpRequest}.
pub struct SignatureHelpRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of characters that trigger signature help automatically.
    pub trigger_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of characters that re-trigger signature help.
    ///
    /// These trigger characters are only active when signature help is already showing. All trigger characters
    /// are also counted as re-trigger characters.
    ///
    /// @since 3.15.0
    pub retrigger_characters: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link DefinitionRequest}.
pub struct DefinitionParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link DefinitionRequest}.
pub struct DefinitionRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link ReferencesRequest}.
pub struct ReferenceParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    pub context: ReferenceContext,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link ReferencesRequest}.
pub struct ReferenceRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link DocumentHighlightRequest}.
pub struct DocumentHighlightParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A document highlight is a range inside a text document which deserves
/// special attention. Usually a document highlight is visualized by changing
/// the background color of its range.
pub struct DocumentHighlight {
    /// The range this highlight applies to.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The highlight kind, default is {@link DocumentHighlightKind.Text text}.
    pub kind: Option<DocumentHighlightKind>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link DocumentHighlightRequest}.
pub struct DocumentHighlightRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link DocumentSymbolRequest}.
pub struct DocumentSymbolParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
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

    #[deprecated = "Use tags instead"]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if this symbol is deprecated.
    ///
    /// @deprecated Use tags instead
    pub deprecated: Option<bool>,

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents programming constructs like variables, classes, interfaces etc.
/// that appear in a document. Document symbols can be hierarchical and they
/// have two ranges: one that encloses its definition and one that points to
/// its most interesting range, e.g. the range of an identifier.
pub struct DocumentSymbol {
    /// The name of this symbol. Will be displayed in the user interface and therefore must not be
    /// an empty string or a string only consisting of white spaces.
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// More detail for this symbol, e.g the signature of a function.
    pub detail: Option<String>,

    /// The kind of this symbol.
    pub kind: SymbolKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this document symbol.
    ///
    /// @since 3.16.0
    pub tags: Option<Vec<SymbolTag>>,

    #[deprecated = "Use tags instead"]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates if this symbol is deprecated.
    ///
    /// @deprecated Use tags instead
    pub deprecated: Option<bool>,

    /// The range enclosing this symbol not including leading/trailing whitespace but everything else
    /// like comments. This information is typically used to determine if the clients cursor is
    /// inside the symbol to reveal in the symbol in the UI.
    pub range: Range,

    /// The range that should be selected and revealed when this symbol is being picked, e.g the name of a function.
    /// Must be contained by the `range`.
    pub selection_range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Children of this symbol, e.g. properties of a class.
    pub children: Option<Vec<DocumentSymbol>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link DocumentSymbolRequest}.
pub struct DocumentSymbolRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string that is shown when multiple outlines trees
    /// are shown for the same document.
    ///
    /// @since 3.16.0
    pub label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link CodeActionRequest}.
pub struct CodeActionParams {
    /// The document in which the command was invoked.
    pub text_document: TextDocumentIdentifier,

    /// The range for which the command was invoked.
    pub range: Range,

    /// Context carrying additional information.
    pub context: CodeActionContext,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
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
/// A code action represents a change that can be performed in code, e.g. to fix a problem or
/// to refactor code.
///
/// A CodeAction must set either `edit` and/or a `command`. If both are supplied, the `edit` is applied first, then the `command` is executed.
pub struct CodeAction {
    /// A short, human-readable, title for this code action.
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The kind of the code action.
    ///
    /// Used to filter code actions.
    pub kind: Option<CodeActionKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostics that this code action resolves.
    pub diagnostics: Option<Vec<Diagnostic>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Marks this as a preferred action. Preferred actions are used by the `auto fix` command and can be targeted
    /// by keybindings.
    ///
    /// A quick fix should be marked preferred if it properly addresses the underlying error.
    /// A refactoring should be marked preferred if it is the most reasonable choice of actions to take.
    ///
    /// @since 3.15.0
    pub is_preferred: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Marks that the code action cannot currently be applied.
    ///
    /// Clients should follow the following guidelines regarding disabled code actions:
    ///
    ///   - Disabled code actions are not shown in automatic [lightbulbs](https://code.visualstudio.com/docs/editor/editingevolved#_code-action)
    ///     code action menus.
    ///
    ///   - Disabled actions are shown as faded out in the code action menu when the user requests a more specific type
    ///     of code action, such as refactorings.
    ///
    ///   - If the user has a [keybinding](https://code.visualstudio.com/docs/editor/refactoring#_keybindings-for-code-actions)
    ///     that auto applies a code action and only disabled code actions are returned, the client should show the user an
    ///     error message with `reason` in the editor.
    ///
    /// @since 3.16.0
    pub disabled: Option<CodeActionDisabled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The workspace edit this code action performs.
    pub edit: Option<WorkspaceEdit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A command this code action executes. If a code action
    /// provides an edit and a command, first the edit is
    /// executed and then the command.
    pub command: Option<Command>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on a code action between
    /// a `textDocument/codeAction` and a `codeAction/resolve` request.
    ///
    /// @since 3.16.0
    pub data: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this code action.
    ///
    /// @since 3.18.0 - proposed
    pub tags: Option<Vec<CodeActionTag>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link CodeActionRequest}.
pub struct CodeActionRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// CodeActionKinds that this server may return.
    ///
    /// The list of kinds may be generic, such as `CodeActionKind.Refactor`, or the server
    /// may list out every specific kind they provide.
    pub code_action_kinds: Option<Vec<CodeActionKind>>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Static documentation for a class of code actions.
    ///
    /// Documentation from the provider should be shown in the code actions menu if either:
    ///
    /// - Code actions of `kind` are requested by the editor. In this case, the editor will show the documentation that
    ///   most closely matches the requested code action kind. For example, if a provider has documentation for
    ///   both `Refactor` and `RefactorExtract`, when the user requests code actions for `RefactorExtract`,
    ///   the editor will use the documentation for `RefactorExtract` instead of the documentation for `Refactor`.
    ///
    /// - Any code actions of `kind` are returned by the provider.
    ///
    /// At most one documentation entry should be shown per provider.
    ///
    /// @since 3.18.0
    /// @proposed
    pub documentation: Option<Vec<CodeActionKindDocumentation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a code action.
    ///
    /// @since 3.16.0
    pub resolve_provider: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link WorkspaceSymbolRequest}.
pub struct WorkspaceSymbolParams {
    /// A query string to filter symbols by. Clients may send an empty
    /// string here to request all symbols.
    ///
    /// The `query`-parameter should be interpreted in a *relaxed way* as editors
    /// will apply their own highlighting and scoring on the results. A good rule
    /// of thumb is to match case-insensitive and to simply check that the
    /// characters of *query* appear in their order in a candidate symbol.
    /// Servers shouldn't use prefix, substring, or similar strict matching.
    pub query: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A special workspace symbol that supports locations without a range.
///
/// See also SymbolInformation.
///
/// @since 3.17.0
pub struct WorkspaceSymbol {
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

    /// The location of the symbol. Whether a server is allowed to
    /// return a location without a range depends on the client
    /// capability `workspace.symbol.resolveSupport`.
    ///
    /// See SymbolInformation#location for more details.
    pub location: Union2<Location, LocationUriOnly>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on a workspace symbol between a
    /// workspace symbol request and a workspace symbol resolve request.
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link WorkspaceSymbolRequest}.
pub struct WorkspaceSymbolRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a workspace symbol.
    ///
    /// @since 3.17.0
    pub resolve_provider: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link CodeLensRequest}.
pub struct CodeLensParams {
    /// The document to request code lens for.
    pub text_document: TextDocumentIdentifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A code lens represents a {@link Command command} that should be shown along with
/// source text, like the number of references, a way to run tests, etc.
///
/// A code lens is _unresolved_ when no command is associated to it. For performance
/// reasons the creation of a code lens and resolving should be done in two stages.
pub struct CodeLens {
    /// The range in which this code lens is valid. Should only span a single line.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The command this code lens represents.
    pub command: Option<Command>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on a code lens item between
    /// a {@link CodeLensRequest} and a {@link CodeLensResolveRequest}
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link CodeLensRequest}.
pub struct CodeLensRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Code lens has a resolve provider as well.
    pub resolve_provider: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link DocumentLinkRequest}.
pub struct DocumentLinkParams {
    /// The document to provide document links for.
    pub text_document: TextDocumentIdentifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A document link is a range in a text document that links to an internal or external resource, like another
/// text document or a web site.
pub struct DocumentLink {
    /// The range this link applies to.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The uri this link points to. If missing a resolve request is sent later.
    pub target: Option<Uri>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The tooltip text when you hover over this link.
    ///
    /// If a tooltip is provided, is will be displayed in a string that includes instructions on how to
    /// trigger the link, such as `{0} (ctrl + click)`. The specific instructions vary depending on OS,
    /// user settings, and localization.
    ///
    /// @since 3.15.0
    pub tooltip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on a document link between a
    /// DocumentLinkRequest and a DocumentLinkResolveRequest.
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link DocumentLinkRequest}.
pub struct DocumentLinkRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Document links have a resolve provider as well.
    pub resolve_provider: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link DocumentFormattingRequest}.
pub struct DocumentFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The format options.
    pub options: FormattingOptions,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link DocumentFormattingRequest}.
pub struct DocumentFormattingRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link DocumentRangeFormattingRequest}.
pub struct DocumentRangeFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The range to format
    pub range: Range,

    /// The format options
    pub options: FormattingOptions,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link DocumentRangeFormattingRequest}.
pub struct DocumentRangeFormattingRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the server supports formatting multiple ranges at once.
    ///
    /// @since 3.18.0
    /// @proposed
    pub ranges_support: Option<bool>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link DocumentRangesFormattingRequest}.
///
/// @since 3.18.0
/// @proposed
pub struct DocumentRangesFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The ranges to format
    pub ranges: Vec<Range>,

    /// The format options
    pub options: FormattingOptions,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link DocumentOnTypeFormattingRequest}.
pub struct DocumentOnTypeFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The position around which the on type formatting should happen.
    /// This is not necessarily the exact position where the character denoted
    /// by the property `ch` got typed.
    pub position: Position,

    /// The character that has been typed that triggered the formatting
    /// on type request. That is not necessarily the last character that
    /// got inserted into the document since the client could auto insert
    /// characters as well (e.g. like automatic brace completion).
    pub ch: String,

    /// The formatting options.
    pub options: FormattingOptions,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link DocumentOnTypeFormattingRequest}.
pub struct DocumentOnTypeFormattingRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    /// A character on which formatting should be triggered, like `{`.
    pub first_trigger_character: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// More trigger characters.
    pub more_trigger_character: Option<Vec<String>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link RenameRequest}.
pub struct RenameParams {
    /// The document to rename.
    pub text_document: TextDocumentIdentifier,

    /// The position at which this request was sent.
    pub position: Position,

    /// The new name of the symbol. If the given name is not valid the
    /// request must return a {@link ResponseError} with an
    /// appropriate message set.
    pub new_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link RenameRequest}.
pub struct RenameRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Renames should be checked and tested before being executed.
    ///
    /// @since version 3.12.0
    pub prepare_provider: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareRenameParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link ExecuteCommandRequest}.
pub struct ExecuteCommandParams {
    /// The identifier of the actual command handler.
    pub command: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Arguments that the command should be invoked with.
    pub arguments: Option<Vec<serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link ExecuteCommandRequest}.
pub struct ExecuteCommandRegistrationOptions {
    /// The commands to be executed on the server
    pub commands: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters passed via an apply workspace edit request.
pub struct ApplyWorkspaceEditParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional label of the workspace edit. This label is
    /// presented in the user interface for example on an undo
    /// stack to undo the workspace edit.
    pub label: Option<String>,

    /// The edits to apply.
    pub edit: WorkspaceEdit,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional data about the edit.
    ///
    /// @since 3.18.0
    /// @proposed
    pub metadata: Option<WorkspaceEditMetadata>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result returned from the apply workspace edit request.
///
/// @since 3.17 renamed from ApplyWorkspaceEditResponse
pub struct ApplyWorkspaceEditResult {
    /// Indicates whether the edit was applied or not.
    pub applied: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional textual description for why the edit was not applied.
    /// This may be used by the server for diagnostic logging or to provide
    /// a suitable error for a request that triggered the edit.
    pub failure_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Depending on the client's failure handling strategy `failedChange` might
    /// contain the index of the change that failed. This property is only available
    /// if the client signals a `failureHandlingStrategy` in its client capabilities.
    pub failed_change: Option<u32>,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTraceParams {
    pub value: TraceValue,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogTraceParams {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelParams {
    /// The request id to cancel.
    pub id: Union2<i32, String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressParams {
    /// The progress token provided by the client or server.
    pub token: ProgressToken,

    /// The progress data.
    pub value: serde_json::Value,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImplementationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDefinitionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The workspace folder change event.
pub struct WorkspaceFoldersChangeEvent {
    /// The array of added workspace folders
    pub added: Vec<WorkspaceFolder>,

    /// The array of the removed workspace folders
    pub removed: Vec<WorkspaceFolder>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The scope to get the configuration section for.
    pub scope_uri: Option<Uri>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The configuration section asked for.
    pub section: Option<String>,
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
    pub red: i32,

    /// The green component of this color in the range [0-1].
    pub green: i32,

    /// The blue component of this color in the range [0-1].
    pub blue: i32,

    /// The alpha component of this color in the range [0-1].
    pub alpha: i32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentColorOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeclarationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
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
pub struct SelectionRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Call hierarchy options used during static registration.
///
/// @since 3.16.0
pub struct CallHierarchyOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensOptions {
    /// The legend used by the server
    pub legend: SemanticTokensLegend,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Server supports providing semantic tokens for a specific range
    /// of a document.
    pub range: Option<Union2<bool, serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Server supports providing semantic tokens for a full document.
    pub full: Option<Union2<bool, SemanticTokensFullDelta>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensEdit {
    /// The start offset of the edit.
    pub start: u32,

    /// The count of elements to remove.
    pub delete_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The elements to insert.
    pub data: Option<Vec<u32>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkedEditingRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
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
    pub edits: Vec<Union2<TextEdit, AnnotatedTextEdit>>,
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
/// A filter to describe in which file operation requests or notifications
/// the server is interested in receiving.
///
/// @since 3.16.0
pub struct FileOperationFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A Uri scheme like `file` or `untitled`.
    pub scheme: Option<String>,

    /// The actual file operation pattern.
    pub pattern: FileOperationPattern,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonikerOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Type hierarchy options used during static registration.
///
/// @since 3.17.0
pub struct TypeHierarchyOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.17.0
pub struct InlineValueContext {
    /// The stack frame (as a DAP Id) where the execution has stopped.
    pub frame_id: i32,

    /// The document range where execution has stopped.
    /// Typically the end position of the range denotes the line where the inline values are shown.
    pub stopped_location: Range,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inline value options used during static registration.
///
/// @since 3.17.0
pub struct InlineValueOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An inlay hint label part allows for interactive and composite labels
/// of inlay hints.
///
/// @since 3.17.0
pub struct InlayHintLabelPart {
    /// The value of this label part.
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The tooltip text when you hover over this label part. Depending on
    /// the client capability `inlayHint.resolveSupport` clients might resolve
    /// this property late using the resolve request.
    pub tooltip: Option<Union2<String, MarkupContent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional source code location that represents this
    /// label part.
    ///
    /// The editor will use this location for the hover and for code navigation
    /// features: This part will become a clickable link that resolves to the
    /// definition of the symbol at the given location (not necessarily the
    /// location itself), it shows the hover that shows at the given location,
    /// and it shows a context menu with further code navigation commands.
    ///
    /// Depending on the client capability `inlayHint.resolveSupport` clients
    /// might resolve this property late using the resolve request.
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional command for this label part.
    ///
    /// Depending on the client capability `inlayHint.resolveSupport` clients
    /// might resolve this property late using the resolve request.
    pub command: Option<Command>,
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
/// Inlay hint options used during static registration.
///
/// @since 3.17.0
pub struct InlayHintOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for an inlay hint item.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
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
    pub related_documents: Option<HashMap<Uri, Union2<FullDocumentDiagnosticReport, UnchangedDocumentDiagnosticReport>>>,
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
    pub related_documents: Option<HashMap<Uri, Union2<FullDocumentDiagnosticReport, UnchangedDocumentDiagnosticReport>>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A diagnostic report with a full set of problems.
///
/// @since 3.17.0
pub struct FullDocumentDiagnosticReport {
    /// A full document diagnostic report.
    pub kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional result id. If provided it will
    /// be sent on the next diagnostic request for the
    /// same document.
    pub result_id: Option<String>,

    /// The actual items.
    pub items: Vec<Diagnostic>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A diagnostic report indicating that the last returned
/// report is still accurate.
///
/// @since 3.17.0
pub struct UnchangedDocumentDiagnosticReport {
    /// A document diagnostic report indicating
    /// no changes to the last result. A server can
    /// only return `unchanged` if result ids are
    /// provided.
    pub kind: String,

    /// A result id which will be sent on the next
    /// diagnostic request for the same document.
    pub result_id: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Diagnostic options.
///
/// @since 3.17.0
pub struct DiagnosticOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional identifier under which the diagnostics are
    /// managed by the client.
    pub identifier: Option<String>,

    /// Whether the language has inter file dependencies meaning that
    /// editing code in one file can result in a different diagnostic
    /// set in another file. Inter file dependencies are common for
    /// most programming languages and typically uncommon for linters.
    pub inter_file_dependencies: bool,

    /// The server provides support for workspace diagnostics as well.
    pub workspace_diagnostics: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
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
    pub notebook_selector: Vec<Union2<NotebookDocumentFilterWithNotebook, NotebookDocumentFilterWithCells>>,

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

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provides information about the context in which an inline completion was requested.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionContext {
    /// Describes how the inline completion was triggered.
    pub trigger_kind: InlineCompletionTriggerKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Provides information about the currently selected item in the autocomplete widget if it is visible.
    pub selected_completion_info: Option<SelectedCompletionInfo>,
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

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inline completion options used during static registration.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Text document content provider options.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentOptions {
    /// The schemes for which the server provides content.
    pub schemes: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// General parameters to register for a notification or to register a provider.
pub struct Registration {
    /// The id used to register the request. The id can be used to deregister
    /// the request again.
    pub id: String,

    /// The method / capability to register for.
    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Options necessary for the registration.
    pub register_options: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// General parameters to unregister a request or notification.
pub struct Unregistration {
    /// The id used to unregister the request or notification. Usually an id
    /// provided during the register request.
    pub id: String,

    /// The method to unregister for.
    pub method: String,
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
    pub text_document_sync: Option<Union2<TextDocumentSyncOptions, TextDocumentSyncKind>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Defines how notebook documents are synced.
    ///
    /// @since 3.17.0
    pub notebook_document_sync: Option<Union2<NotebookDocumentSyncOptions, NotebookDocumentSyncRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides completion support.
    pub completion_provider: Option<CompletionOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides hover support.
    pub hover_provider: Option<Union2<bool, HoverOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides signature help support.
    pub signature_help_provider: Option<SignatureHelpOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides Goto Declaration support.
    pub declaration_provider: Option<Union3<bool, DeclarationOptions, DeclarationRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides goto definition support.
    pub definition_provider: Option<Union2<bool, DefinitionOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides Goto Type Definition support.
    pub type_definition_provider: Option<Union3<bool, TypeDefinitionOptions, TypeDefinitionRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides Goto Implementation support.
    pub implementation_provider: Option<Union3<bool, ImplementationOptions, ImplementationRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides find references support.
    pub references_provider: Option<Union2<bool, ReferenceOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document highlight support.
    pub document_highlight_provider: Option<Union2<bool, DocumentHighlightOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document symbol support.
    pub document_symbol_provider: Option<Union2<bool, DocumentSymbolOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides code actions. CodeActionOptions may only be
    /// specified if the client states that it supports
    /// `codeActionLiteralSupport` in its initial `initialize` request.
    pub code_action_provider: Option<Union2<bool, CodeActionOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides code lens.
    pub code_lens_provider: Option<CodeLensOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document link support.
    pub document_link_provider: Option<DocumentLinkOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides color provider support.
    pub color_provider: Option<Union3<bool, DocumentColorOptions, DocumentColorRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides workspace symbol support.
    pub workspace_symbol_provider: Option<Union2<bool, WorkspaceSymbolOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document formatting.
    pub document_formatting_provider: Option<Union2<bool, DocumentFormattingOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document range formatting.
    pub document_range_formatting_provider: Option<Union2<bool, DocumentRangeFormattingOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides document formatting on typing.
    pub document_on_type_formatting_provider: Option<DocumentOnTypeFormattingOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides rename support. RenameOptions may only be
    /// specified if the client states that it supports
    /// `prepareSupport` in its initial `initialize` request.
    pub rename_provider: Option<Union2<bool, RenameOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides folding provider support.
    pub folding_range_provider: Option<Union3<bool, FoldingRangeOptions, FoldingRangeRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides selection range support.
    pub selection_range_provider: Option<Union3<bool, SelectionRangeOptions, SelectionRangeRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides execute command support.
    pub execute_command_provider: Option<ExecuteCommandOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides call hierarchy support.
    ///
    /// @since 3.16.0
    pub call_hierarchy_provider: Option<Union3<bool, CallHierarchyOptions, CallHierarchyRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides linked editing range support.
    ///
    /// @since 3.16.0
    pub linked_editing_range_provider: Option<Union3<bool, LinkedEditingRangeOptions, LinkedEditingRangeRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides semantic tokens support.
    ///
    /// @since 3.16.0
    pub semantic_tokens_provider: Option<Union2<SemanticTokensOptions, SemanticTokensRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides moniker support.
    ///
    /// @since 3.16.0
    pub moniker_provider: Option<Union3<bool, MonikerOptions, MonikerRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides type hierarchy support.
    ///
    /// @since 3.17.0
    pub type_hierarchy_provider: Option<Union3<bool, TypeHierarchyOptions, TypeHierarchyRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides inline values.
    ///
    /// @since 3.17.0
    pub inline_value_provider: Option<Union3<bool, InlineValueOptions, InlineValueRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides inlay hints.
    ///
    /// @since 3.17.0
    pub inlay_hint_provider: Option<Union3<bool, InlayHintOptions, InlayHintRegistrationOptions>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server has support for pull model diagnostics.
    ///
    /// @since 3.17.0
    pub diagnostic_provider: Option<Union2<DiagnosticOptions, DiagnosticRegistrationOptions>>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Inline completion options used during static registration.
    ///
    /// @since 3.18.0
    /// @proposed
    pub inline_completion_provider: Option<Union2<bool, InlineCompletionOptions>>,

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
    pub code: Option<Union2<i32, String>>,

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
/// Contains additional information about the context in which a completion request is triggered.
pub struct CompletionContext {
    /// How the completion was triggered.
    pub trigger_kind: CompletionTriggerKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The trigger character (a single character) that has trigger code complete.
    /// Is undefined if `triggerKind !== CompletionTriggerKind.TriggerCharacter`
    pub trigger_character: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Additional details for a completion item label.
///
/// @since 3.17.0
pub struct CompletionItemLabelDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional string which is rendered less prominently directly after {@link CompletionItem.label label},
    /// without any spacing. Should be used for function signatures and type annotations.
    pub detail: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional string which is rendered less prominently after {@link CompletionItem.detail}. Should be used
    /// for fully qualified names and file paths.
    pub description: Option<String>,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// In many cases the items of an actual completion result share the same
/// value for properties like `commitCharacters` or the range of a text
/// edit. A completion list can therefore define item defaults which will
/// be used if a completion item itself doesn't specify the value.
///
/// If a completion list specifies a default value and a completion item
/// also specifies a corresponding value, the rules for combining these are
/// defined by `applyKinds` (if the client supports it), defaulting to
/// ApplyKind.Replace.
///
/// Servers are only allowed to return default values if the client
/// signals support for this via the `completionList.itemDefaults`
/// capability.
///
/// @since 3.17.0
pub struct CompletionItemDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A default commit character set.
    ///
    /// @since 3.17.0
    pub commit_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A default edit range.
    ///
    /// @since 3.17.0
    pub edit_range: Option<Union2<Range, EditRangeWithInsertReplace>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A default insert text format.
    ///
    /// @since 3.17.0
    pub insert_text_format: Option<InsertTextFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A default insert text mode.
    ///
    /// @since 3.17.0
    pub insert_text_mode: Option<InsertTextMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A default data value.
    ///
    /// @since 3.17.0
    pub data: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Specifies how fields from a completion item should be combined with those
/// from `completionList.itemDefaults`.
///
/// If unspecified, all fields will be treated as ApplyKind.Replace.
///
/// If a field's value is ApplyKind.Replace, the value from a completion item (if
/// provided and not `null`) will always be used instead of the value from
/// `completionItem.itemDefaults`.
///
/// If a field's value is ApplyKind.Merge, the values will be merged using the rules
/// defined against each field below.
///
/// Servers are only allowed to return `applyKind` if the client
/// signals support for this via the `completionList.applyKindSupport`
/// capability.
///
/// @since 3.18.0
pub struct CompletionItemApplyKinds {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies whether commitCharacters on a completion will replace or be
    /// merged with those in `completionList.itemDefaults.commitCharacters`.
    ///
    /// If ApplyKind.Replace, the commit characters from the completion item will
    /// always be used unless not provided, in which case those from
    /// `completionList.itemDefaults.commitCharacters` will be used. An
    /// empty list can be used if a completion item does not have any commit
    /// characters and also should not use those from
    /// `completionList.itemDefaults.commitCharacters`.
    ///
    /// If ApplyKind.Merge the commitCharacters for the completion will be the
    /// union of all values in both `completionList.itemDefaults.commitCharacters`
    /// and the completion's own `commitCharacters`.
    ///
    /// @since 3.18.0
    pub commit_characters: Option<ApplyKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies whether the `data` field on a completion will replace or
    /// be merged with data from `completionList.itemDefaults.data`.
    ///
    /// If ApplyKind.Replace, the data from the completion item will be used if
    /// provided (and not `null`), otherwise
    /// `completionList.itemDefaults.data` will be used. An empty object can
    /// be used if a completion item does not have any data but also should
    /// not use the value from `completionList.itemDefaults.data`.
    ///
    /// If ApplyKind.Merge, a shallow merge will be performed between
    /// `completionList.itemDefaults.data` and the completion's own data
    /// using the following rules:
    ///
    /// - If a completion's `data` field is not provided (or `null`), the
    ///   entire `data` field from `completionList.itemDefaults.data` will be
    ///   used as-is.
    /// - If a completion's `data` field is provided, each field will
    ///   overwrite the field of the same name in
    ///   `completionList.itemDefaults.data` but no merging of nested fields
    ///   within that value will occur.
    ///
    /// @since 3.18.0
    pub data: Option<ApplyKind>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Completion options.
pub struct CompletionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Most tools trigger completion request automatically without explicitly requesting
    /// it using a keyboard shortcut (e.g. Ctrl+Space). Typically they do so when the user
    /// starts to type an identifier. For example if the user types `c` in a JavaScript file
    /// code complete will automatically pop up present `console` besides others as a
    /// completion item. Characters that make up identifiers don't need to be listed here.
    ///
    /// If code complete should automatically be trigger on characters not being valid inside
    /// an identifier (for example `.` in JavaScript) list them in `triggerCharacters`.
    pub trigger_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The list of all possible characters that commit a completion. This field can be used
    /// if clients don't support individual commit characters per completion item. See
    /// `ClientCapabilities.textDocument.completion.completionItem.commitCharactersSupport`
    ///
    /// If a server provides both `allCommitCharacters` and commit characters on an individual
    /// completion item the ones on the completion item win.
    ///
    /// @since 3.2.0
    pub all_commit_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a completion item.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server supports the following `CompletionItem` specific
    /// capabilities.
    ///
    /// @since 3.17.0
    pub completion_item: Option<ServerCompletionItemOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Hover options.
pub struct HoverOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Additional information about the context in which a signature help request was triggered.
///
/// @since 3.15.0
pub struct SignatureHelpContext {
    /// Action that caused signature help to be triggered.
    pub trigger_kind: SignatureHelpTriggerKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Character that caused signature help to be triggered.
    ///
    /// This is undefined when `triggerKind !== SignatureHelpTriggerKind.TriggerCharacter`
    pub trigger_character: Option<String>,

    /// `true` if signature help was already showing when it was triggered.
    ///
    /// Retriggers occurs when the signature help is already active and can be caused by actions such as
    /// typing a trigger character, a cursor move, or document content changes.
    pub is_retrigger: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The currently active `SignatureHelp`.
    ///
    /// The `activeSignatureHelp` has its `SignatureHelp.activeSignature` field updated based on
    /// the user navigating through available signatures.
    pub active_signature_help: Option<SignatureHelp>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the signature of something callable. A signature
/// can have a label, like a function-name, a doc-comment, and
/// a set of parameters.
pub struct SignatureInformation {
    /// The label of this signature. Will be shown in
    /// the UI.
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The human-readable doc-comment of this signature. Will be shown
    /// in the UI but can be omitted.
    pub documentation: Option<Union2<String, MarkupContent>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The parameters of this signature.
    pub parameters: Option<Vec<ParameterInformation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The index of the active parameter.
    ///
    /// If `null`, no parameter of the signature is active (for example a named
    /// argument that does not match any declared parameters). This is only valid
    /// if the client specifies the client capability
    /// `textDocument.signatureHelp.noActiveParameterSupport === true`
    ///
    /// If provided (or `null`), this is used in place of
    /// `SignatureHelp.activeParameter`.
    ///
    /// @since 3.16.0
    pub active_parameter: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Server Capabilities for a {@link SignatureHelpRequest}.
pub struct SignatureHelpOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of characters that trigger signature help automatically.
    pub trigger_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of characters that re-trigger signature help.
    ///
    /// These trigger characters are only active when signature help is already showing. All trigger characters
    /// are also counted as re-trigger characters.
    ///
    /// @since 3.15.0
    pub retrigger_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Server Capabilities for a {@link DefinitionRequest}.
pub struct DefinitionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Value-object that contains additional information when
/// requesting references.
pub struct ReferenceContext {
    /// Include the declaration of the current symbol.
    pub include_declaration: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Reference options.
pub struct ReferenceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link DocumentHighlightRequest}.
pub struct DocumentHighlightOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
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
/// Provider options for a {@link DocumentSymbolRequest}.
pub struct DocumentSymbolOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string that is shown when multiple outlines trees
    /// are shown for the same document.
    ///
    /// @since 3.16.0
    pub label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Contains additional diagnostic information about the context in which
/// a {@link CodeActionProvider.provideCodeActions code action} is run.
pub struct CodeActionContext {
    /// An array of diagnostics known on the client side overlapping the range provided to the
    /// `textDocument/codeAction` request. They are provided so that the server knows which
    /// errors are currently presented to the user for the given range. There is no guarantee
    /// that these accurately reflect the error state of the resource. The primary parameter
    /// to compute code actions is the provided range.
    pub diagnostics: Vec<Diagnostic>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Requested kind of actions to return.
    ///
    /// Actions not of this kind are filtered out by the client before being shown. So servers
    /// can omit computing them.
    pub only: Option<Vec<CodeActionKind>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The reason why code actions were requested.
    ///
    /// @since 3.17.0
    pub trigger_kind: Option<CodeActionTriggerKind>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Captures why the code action is currently disabled.
///
/// @since 3.18.0
pub struct CodeActionDisabled {
    /// Human readable description of why the code action is currently disabled.
    ///
    /// This is displayed in the code actions UI.
    pub reason: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link CodeActionRequest}.
pub struct CodeActionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CodeActionKinds that this server may return.
    ///
    /// The list of kinds may be generic, such as `CodeActionKind.Refactor`, or the server
    /// may list out every specific kind they provide.
    pub code_action_kinds: Option<Vec<CodeActionKind>>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Static documentation for a class of code actions.
    ///
    /// Documentation from the provider should be shown in the code actions menu if either:
    ///
    /// - Code actions of `kind` are requested by the editor. In this case, the editor will show the documentation that
    ///   most closely matches the requested code action kind. For example, if a provider has documentation for
    ///   both `Refactor` and `RefactorExtract`, when the user requests code actions for `RefactorExtract`,
    ///   the editor will use the documentation for `RefactorExtract` instead of the documentation for `Refactor`.
    ///
    /// - Any code actions of `kind` are returned by the provider.
    ///
    /// At most one documentation entry should be shown per provider.
    ///
    /// @since 3.18.0
    /// @proposed
    pub documentation: Option<Vec<CodeActionKindDocumentation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a code action.
    ///
    /// @since 3.16.0
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Location with only uri and does not include range.
///
/// @since 3.18.0
pub struct LocationUriOnly {
    pub uri: Uri,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Server capabilities for a {@link WorkspaceSymbolRequest}.
pub struct WorkspaceSymbolOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a workspace symbol.
    ///
    /// @since 3.17.0
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Code Lens provider options of a {@link CodeLensRequest}.
pub struct CodeLensOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Code lens has a resolve provider as well.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link DocumentLinkRequest}.
pub struct DocumentLinkOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Document links have a resolve provider as well.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
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
/// Provider options for a {@link DocumentFormattingRequest}.
pub struct DocumentFormattingOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link DocumentRangeFormattingRequest}.
pub struct DocumentRangeFormattingOptions {
    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the server supports formatting multiple ranges at once.
    ///
    /// @since 3.18.0
    /// @proposed
    pub ranges_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link DocumentOnTypeFormattingRequest}.
pub struct DocumentOnTypeFormattingOptions {
    /// A character on which formatting should be triggered, like `{`.
    pub first_trigger_character: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// More trigger characters.
    pub more_trigger_character: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link RenameRequest}.
pub struct RenameOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Renames should be checked and tested before being executed.
    ///
    /// @since version 3.12.0
    pub prepare_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The server capabilities of a {@link ExecuteCommandRequest}.
pub struct ExecuteCommandOptions {
    /// The commands to be executed on the server
    pub commands: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
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
/// @since 3.16.0
pub struct SemanticTokensLegend {
    /// The token types a server uses.
    pub token_types: Vec<String>,

    /// The token modifiers a server uses.
    pub token_modifiers: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Semantic tokens options to support deltas for full documents
///
/// @since 3.18.0
pub struct SemanticTokensFullDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server supports deltas for full documents.
    pub delta: Option<bool>,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A pattern to describe in which file operation requests or notifications
/// the server is interested in receiving.
///
/// @since 3.16.0
pub struct FileOperationPattern {
    /// The glob pattern to match. Glob patterns can have the following syntax:
    /// - `*` to match one or more characters in a path segment
    /// - `?` to match on one character in a path segment
    /// - `**` to match any number of path segments, including none
    /// - `{}` to group sub patterns into an OR expression. (e.g. `**​/*.{ts,js}` matches all TypeScript and JavaScript files)
    /// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
    /// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
    pub glob: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether to match files or folders with this pattern.
    ///
    /// Matches both if undefined.
    pub matches: Option<FileOperationPatternKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional options used during matching.
    pub options: Option<FileOperationPatternOptions>,
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct NotebookDocumentFilterWithNotebook {
    /// The notebook to be synced If a string
    /// value is provided it matches against the
    /// notebook type. '*' matches every notebook.
    pub notebook: Union2<String, NotebookDocumentFilter>,

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
    pub notebook: Option<Union2<String, NotebookDocumentFilter>>,

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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Open and close notifications are sent to the server. If omitted open close notification should not
    /// be sent.
    pub open_close: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Change notifications are sent to the server. See TextDocumentSyncKind.None, TextDocumentSyncKind.Full
    /// and TextDocumentSyncKind.Incremental. If omitted it defaults to TextDocumentSyncKind.None.
    pub change: Option<TextDocumentSyncKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// If present will save notifications are sent to the server. If omitted the notification should not be
    /// sent.
    pub will_save: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// If present will save wait until requests are sent to the server. If omitted the request should not be
    /// sent.
    pub will_save_wait_until: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// If present save notifications are sent to the server. If omitted the notification should not be
    /// sent.
    pub save: Option<Union2<bool, SaveOptions>>,
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
    pub text_document_content: Option<Union2<TextDocumentContentOptions, TextDocumentContentRegistrationOptions>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct TextDocumentContentChangePartial {
    /// The range of the document that changed.
    pub range: Range,

    #[deprecated = "use range instead."]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The optional length of the range that got replaced.
    ///
    /// @deprecated use range instead.
    pub range_length: Option<u32>,

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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Edit range variant that includes ranges for insert and replace operations.
///
/// @since 3.18.0
pub struct EditRangeWithInsertReplace {
    pub insert: Range,

    pub replace: Range,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ServerCompletionItemOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server has support for completion item label
    /// details (see also `CompletionItemLabelDetails`) when
    /// receiving a completion item in a resolve call.
    ///
    /// @since 3.17.0
    pub label_details_support: Option<bool>,
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
    pub label: Union2<String, (u32, u32)>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The human-readable doc-comment of this parameter. Will be shown
    /// in the UI but can be omitted.
    pub documentation: Option<Union2<String, MarkupContent>>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Documentation for a class of code actions.
///
/// @since 3.18.0
/// @proposed
pub struct CodeActionKindDocumentation {
    /// The kind of the code action being documented.
    ///
    /// If the kind is generic, such as `CodeActionKind.Refactor`, the documentation will be shown whenever any
    /// refactorings are returned. If the kind if more specific, such as `CodeActionKind.RefactorExtract`, the
    /// documentation will only be shown when extract refactoring code actions are returned.
    pub kind: CodeActionKind,

    /// Command that is ued to display the documentation to the user.
    ///
    /// The title of this documentation code action is taken from {@linkcode Command.title}
    pub command: Command,
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
    pub notebook: Union2<String, NotebookDocumentFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A language id like `python`.
    ///
    /// Will be matched against the language id of the
    /// notebook cell document. '*' matches every language.
    pub language: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Matching options for the file operation pattern.
///
/// @since 3.16.0
pub struct FileOperationPatternOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The pattern should be matched ignoring casing.
    pub ignore_case: Option<bool>,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceFoldersServerCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server has support for workspace folders
    pub supported: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the server wants to receive workspace folder
    /// change notifications.
    ///
    /// If a string is provided the string is treated as an ID
    /// under which the notification is registered on the client
    /// side. The ID can be used to unregister for these events
    /// using the `client/unregisterCapability` request.
    pub change_notifications: Option<Union2<String, bool>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Options for notifications/requests for user operations on files.
///
/// @since 3.16.0
pub struct FileOperationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server is interested in receiving didCreateFiles notifications.
    pub did_create: Option<FileOperationRegistrationOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server is interested in receiving willCreateFiles requests.
    pub will_create: Option<FileOperationRegistrationOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server is interested in receiving didRenameFiles notifications.
    pub did_rename: Option<FileOperationRegistrationOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server is interested in receiving willRenameFiles requests.
    pub will_rename: Option<FileOperationRegistrationOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server is interested in receiving didDeleteFiles file notifications.
    pub did_delete: Option<FileOperationRegistrationOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server is interested in receiving willDeleteFiles file requests.
    pub will_delete: Option<FileOperationRegistrationOptions>,
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
    pub base_uri: Union2<WorkspaceFolder, Uri>,

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
pub struct DidChangeConfigurationClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Did change configuration notification supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeWatchedFilesClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Did change watched files notification supports dynamic registration. Please note
    /// that the current protocol doesn't support static configuration for file changes
    /// from the server side.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client has support for {@link  RelativePattern relative pattern}
    /// or not.
    ///
    /// @since 3.17.0
    pub relative_pattern_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities for a {@link WorkspaceSymbolRequest}.
pub struct WorkspaceSymbolClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Symbol request supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specific capabilities for the `SymbolKind` in the `workspace/symbol` request.
    pub symbol_kind: Option<ClientSymbolKindOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports tags on `SymbolInformation`.
    /// Clients supporting tags have to handle unknown tags gracefully.
    ///
    /// @since 3.16.0
    pub tag_support: Option<ClientSymbolTagOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client support partial workspace symbols. The client will send the
    /// request `workspaceSymbol/resolve` to the server to resolve additional
    /// properties.
    ///
    /// @since 3.17.0
    pub resolve_support: Option<ClientSymbolResolveOptions>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The client capabilities of a {@link ExecuteCommandRequest}.
pub struct ExecuteCommandClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Execute command supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from
    /// the server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// semantic tokens currently shown. It should be used with absolute care
    /// and is useful for situation where a server for example detects a project
    /// wide change that requires such a calculation.
    pub refresh_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct CodeLensWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from the
    /// server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// code lenses currently shown. It should be used with absolute care and is
    /// useful for situation where a server for example detect a project wide
    /// change that requires such a calculation.
    pub refresh_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Capabilities relating to events from file operations by the user in the client.
///
/// These events do not come from the file system, they come from user operations
/// like renaming a file in the UI.
///
/// @since 3.16.0
pub struct FileOperationClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports dynamic registration for file requests/notifications.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client has support for sending didCreateFiles notifications.
    pub did_create: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client has support for sending willCreateFiles requests.
    pub will_create: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client has support for sending didRenameFiles notifications.
    pub did_rename: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client has support for sending willRenameFiles requests.
    pub will_rename: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client has support for sending didDeleteFiles notifications.
    pub did_delete: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client has support for sending willDeleteFiles requests.
    pub will_delete: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client workspace capabilities specific to inline values.
///
/// @since 3.17.0
pub struct InlineValueWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from the
    /// server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// inline values currently shown. It should be used with absolute care and is
    /// useful for situation where a server for example detects a project wide
    /// change that requires such a calculation.
    pub refresh_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client workspace capabilities specific to inlay hints.
///
/// @since 3.17.0
pub struct InlayHintWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from
    /// the server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// inlay hints currently shown. It should be used with absolute care and
    /// is useful for situation where a server for example detects a project wide
    /// change that requires such a calculation.
    pub refresh_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Workspace client capabilities specific to diagnostic pull requests.
///
/// @since 3.17.0
pub struct DiagnosticWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from
    /// the server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// pulled diagnostics currently shown. It should be used with absolute care and
    /// is useful for situation where a server for example detects a project wide
    /// change that requires such a calculation.
    pub refresh_support: Option<bool>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client workspace capabilities specific to folding ranges
///
/// @since 3.18.0
/// @proposed
pub struct FoldingRangeWorkspaceClientCapabilities {
    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from the
    /// server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// folding ranges currently shown. It should be used with absolute care and is
    /// useful for situation where a server for example detects a project wide
    /// change that requires such a calculation.
    ///
    /// @since 3.18.0
    /// @proposed
    pub refresh_support: Option<bool>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities for a text document content provider.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text document content provider supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentSyncClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether text document synchronization supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports sending will save notifications.
    pub will_save: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports sending a will save request and
    /// waits for a response providing text edits which will
    /// be applied to the document before it is saved.
    pub will_save_wait_until: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports did save notifications.
    pub did_save: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentFilterClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports Relative Patterns.
    ///
    /// @since 3.18.0
    pub relative_pattern_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Completion client capabilities
pub struct CompletionClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether completion supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the following `CompletionItem` specific
    /// capabilities.
    pub completion_item: Option<ClientCompletionItemOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_item_kind: Option<ClientCompletionItemOptionsKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Defines how the client handles whitespace and indentation
    /// when accepting a completion item that uses multi line
    /// text in either `insertText` or `textEdit`.
    ///
    /// @since 3.17.0
    pub insert_text_mode: Option<InsertTextMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports to send additional context information for a
    /// `textDocument/completion` request.
    pub context_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the following `CompletionList` specific
    /// capabilities.
    ///
    /// @since 3.17.0
    pub completion_list: Option<CompletionListCapabilities>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoverClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether hover supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the following content formats for the content
    /// property. The order describes the preferred format of the client.
    pub content_format: Option<Vec<MarkupKind>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client Capabilities for a {@link SignatureHelpRequest}.
pub struct SignatureHelpClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether signature help supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the following `SignatureInformation`
    /// specific properties.
    pub signature_information: Option<ClientSignatureInformationOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports to send additional context information for a
    /// `textDocument/signatureHelp` request. A client that opts into
    /// contextSupport will also support the `retriggerCharacters` on
    /// `SignatureHelpOptions`.
    ///
    /// @since 3.15.0
    pub context_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.14.0
pub struct DeclarationClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether declaration supports dynamic registration. If this is set to `true`
    /// the client supports the new `DeclarationRegistrationOptions` return value
    /// for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports additional metadata in the form of declaration links.
    pub link_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client Capabilities for a {@link DefinitionRequest}.
pub struct DefinitionClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether definition supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports additional metadata in the form of definition links.
    ///
    /// @since 3.14.0
    pub link_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Since 3.6.0
pub struct TypeDefinitionClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `TypeDefinitionRegistrationOptions` return value
    /// for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports additional metadata in the form of definition links.
    ///
    /// Since 3.14.0
    pub link_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.6.0
pub struct ImplementationClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `ImplementationRegistrationOptions` return value
    /// for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports additional metadata in the form of definition links.
    ///
    /// @since 3.14.0
    pub link_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client Capabilities for a {@link ReferencesRequest}.
pub struct ReferenceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether references supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client Capabilities for a {@link DocumentHighlightRequest}.
pub struct DocumentHighlightClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether document highlight supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client Capabilities for a {@link DocumentSymbolRequest}.
pub struct DocumentSymbolClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether document symbol supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specific capabilities for the `SymbolKind` in the
    /// `textDocument/documentSymbol` request.
    pub symbol_kind: Option<ClientSymbolKindOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports hierarchical document symbols.
    pub hierarchical_document_symbol_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports tags on `SymbolInformation`. Tags are supported on
    /// `DocumentSymbol` if `hierarchicalDocumentSymbolSupport` is set to true.
    /// Clients supporting tags have to handle unknown tags gracefully.
    ///
    /// @since 3.16.0
    pub tag_support: Option<ClientSymbolTagOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports an additional label presented in the UI when
    /// registering a document symbol provider.
    ///
    /// @since 3.16.0
    pub label_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The Client Capabilities of a {@link CodeActionRequest}.
pub struct CodeActionClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client support code action literals of type `CodeAction` as a valid
    /// response of the `textDocument/codeAction` request. If the property is not
    /// set the request can only return `Command` literals.
    ///
    /// @since 3.8.0
    pub code_action_literal_support: Option<ClientCodeActionLiteralOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports the `isPreferred` property.
    ///
    /// @since 3.15.0
    pub is_preferred_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports the `disabled` property.
    ///
    /// @since 3.16.0
    pub disabled_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports the `data` property which is
    /// preserved between a `textDocument/codeAction` and a
    /// `codeAction/resolve` request.
    ///
    /// @since 3.16.0
    pub data_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports resolving additional code action
    /// properties via a separate `codeAction/resolve` request.
    ///
    /// @since 3.16.0
    pub resolve_support: Option<ClientCodeActionResolveOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client honors the change annotations in
    /// text edits and resource operations returned via the
    /// `CodeAction#edit` property by for example presenting
    /// the workspace edit in the user interface and asking
    /// for confirmation.
    ///
    /// @since 3.16.0
    pub honors_change_annotations: Option<bool>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports documentation for a class of
    /// code actions.
    ///
    /// @since 3.18.0
    /// @proposed
    pub documentation_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the tag property on a code action. Clients
    /// supporting tags have to handle unknown tags gracefully.
    ///
    /// @since 3.18.0 - proposed
    pub tag_support: Option<CodeActionTagOptions>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The client capabilities  of a {@link CodeLensRequest}.
pub struct CodeLensClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code lens supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports resolving additional code lens
    /// properties via a separate `codeLens/resolve` request.
    ///
    /// @since 3.18.0
    pub resolve_support: Option<ClientCodeLensResolveOptions>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The client capabilities of a {@link DocumentLinkRequest}.
pub struct DocumentLinkClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether document link supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports the `tooltip` property on `DocumentLink`.
    ///
    /// @since 3.15.0
    pub tooltip_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentColorClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `DocumentColorRegistrationOptions` return value
    /// for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities of a {@link DocumentFormattingRequest}.
pub struct DocumentFormattingClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities of a {@link DocumentRangeFormattingRequest}.
pub struct DocumentRangeFormattingClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether range formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports formatting multiple ranges at once.
    ///
    /// @since 3.18.0
    /// @proposed
    pub ranges_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities of a {@link DocumentOnTypeFormattingRequest}.
pub struct DocumentOnTypeFormattingClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether on type formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether rename supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports testing for validity of rename operations
    /// before execution.
    ///
    /// @since 3.12.0
    pub prepare_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the default behavior result.
    ///
    /// The value indicates the default behavior used by the
    /// client.
    ///
    /// @since 3.16.0
    pub prepare_support_default_behavior: Option<PrepareSupportDefaultBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client honors the change annotations in
    /// text edits and resource operations returned via the
    /// rename request's workspace edit by for example presenting
    /// the workspace edit in the user interface and asking
    /// for confirmation.
    ///
    /// @since 3.16.0
    pub honors_change_annotations: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FoldingRangeClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration for folding range
    /// providers. If this is set to `true` the client supports the new
    /// `FoldingRangeRegistrationOptions` return value for the corresponding
    /// server capability as well.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The maximum number of folding ranges that the client prefers to receive
    /// per document. The value serves as a hint, servers are free to follow the
    /// limit.
    pub range_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// If set, the client signals that it only supports folding complete lines.
    /// If set, client will ignore specified `startCharacter` and `endCharacter`
    /// properties in a FoldingRange.
    pub line_folding_only: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specific options for the folding range kind.
    ///
    /// @since 3.17.0
    pub folding_range_kind: Option<ClientFoldingRangeKindOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specific options for the folding range.
    ///
    /// @since 3.17.0
    pub folding_range: Option<ClientFoldingRangeOptions>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SelectionRangeClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration for selection range providers. If this is set to `true`
    /// the client supports the new `SelectionRangeRegistrationOptions` return value for the corresponding server
    /// capability as well.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The publish diagnostic client capabilities.
pub struct PublishDiagnosticsClientCapabilities {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client interprets the version property of the
    /// `textDocument/publishDiagnostics` notification's parameter.
    ///
    /// @since 3.15.0
    pub version_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct CallHierarchyClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,

    /// Which requests the client supports and might send to the server
    /// depending on the server's capability. Please note that clients might not
    /// show semantic tokens or degrade some of the user experience if a range
    /// or full request is advertised by the client but not provided by the
    /// server. If for example the client capability `requests.full` and
    /// `request.range` are both set to true but the server only provides a
    /// range provider the client might not render a minimap correctly or might
    /// even decide to not show any semantic tokens at all.
    pub requests: ClientSemanticTokensRequestOptions,

    /// The token types that the client supports.
    pub token_types: Vec<String>,

    /// The token modifiers that the client supports.
    pub token_modifiers: Vec<String>,

    /// The token formats the clients supports.
    pub formats: Vec<TokenFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports tokens that can overlap each other.
    pub overlapping_token_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports tokens that can span multiple lines.
    pub multiline_token_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client allows the server to actively cancel a
    /// semantic token request, e.g. supports returning
    /// LSPErrorCodes.ServerCancelled. If a server does the client
    /// needs to retrigger the request.
    ///
    /// @since 3.17.0
    pub server_cancel_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client uses semantic tokens to augment existing
    /// syntax tokens. If set to `true` client side created syntax
    /// tokens and semantic tokens are both used for colorization. If
    /// set to `false` the client only uses the returned semantic tokens
    /// for colorization.
    ///
    /// If the value is `undefined` then the client behavior is not
    /// specified.
    ///
    /// @since 3.17.0
    pub augments_syntax_tokens: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities for the linked editing range request.
///
/// @since 3.16.0
pub struct LinkedEditingRangeClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities specific to the moniker request.
///
/// @since 3.16.0
pub struct MonikerClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether moniker supports dynamic registration. If this is set to `true`
    /// the client supports the new `MonikerRegistrationOptions` return value
    /// for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.17.0
pub struct TypeHierarchyClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities specific to inline values.
///
/// @since 3.17.0
pub struct InlineValueClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration for inline value providers.
    pub dynamic_registration: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inlay hint client capabilities.
///
/// @since 3.17.0
pub struct InlayHintClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether inlay hints support dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates which properties a client can resolve lazily on an inlay
    /// hint.
    pub resolve_support: Option<ClientInlayHintResolveOptions>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities specific to diagnostic pull requests.
///
/// @since 3.17.0
pub struct DiagnosticClientCapabilities {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the clients supports related documents for document diagnostic pulls.
    pub related_document_support: Option<bool>,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities specific to inline completions.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration for inline completion providers.
    pub dynamic_registration: Option<bool>,
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

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Show message request client capabilities
pub struct ShowMessageRequestClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the `MessageActionItem` type.
    pub message_action_item: Option<ClientShowMessageActionItemOptions>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities for the showDocument request.
///
/// @since 3.16.0
pub struct ShowDocumentClientCapabilities {
    /// The client has support for the showDocument
    /// request.
    pub support: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct StaleRequestSupportOptions {
    /// The client will actively cancel the request.
    pub cancel: bool,

    /// The list of requests for which the client
    /// will retry the request if it receives a
    /// response with error code `ContentModified`
    pub retry_on_content_modified: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities specific to regular expressions.
///
/// @since 3.16.0
pub struct RegularExpressionsClientCapabilities {
    /// The engine's name.
    pub engine: RegularExpressionEngineKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The engine's version.
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities specific to the used markdown parser.
///
/// @since 3.16.0
pub struct MarkdownClientCapabilities {
    /// The name of the parser.
    pub parser: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The version of the parser.
    pub version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A list of HTML tags that the client allows / supports in
    /// Markdown.
    ///
    /// @since 3.17.0
    pub allowed_tags: Option<Vec<String>>,
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
/// @since 3.18.0
pub struct ClientSymbolResolveOptions {
    /// The properties that a client can resolve lazily. Usually
    /// `location.range`
    pub properties: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCompletionItemOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports snippets as insert text.
    ///
    /// A snippet can define tab stops and placeholders with `$1`, `$2`
    /// and `${3:foo}`. `$0` defines the final tab stop, it defaults to
    /// the end of the snippet. Placeholders with equal identifiers are linked,
    /// that is typing in one will update others too.
    pub snippet_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports commit characters on a completion item.
    pub commit_characters_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the following content formats for the documentation
    /// property. The order describes the preferred format of the client.
    pub documentation_format: Option<Vec<MarkupKind>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the deprecated property on a completion item.
    pub deprecated_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the preselect property on a completion item.
    pub preselect_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the tag property on a completion item. Clients supporting
    /// tags have to handle unknown tags gracefully. Clients especially need to
    /// preserve unknown tags when sending a completion item back to the server in
    /// a resolve call.
    ///
    /// @since 3.15.0
    pub tag_support: Option<CompletionItemTagOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client support insert replace edit to control different behavior if a
    /// completion item is inserted in the text or should replace text.
    ///
    /// @since 3.16.0
    pub insert_replace_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates which properties a client can resolve lazily on a completion
    /// item. Before version 3.16.0 only the predefined properties `documentation`
    /// and `details` could be resolved lazily.
    ///
    /// @since 3.16.0
    pub resolve_support: Option<ClientCompletionItemResolveOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the `insertTextMode` property on
    /// a completion item to override the whitespace handling mode
    /// as defined by the client (see `insertTextMode`).
    ///
    /// @since 3.16.0
    pub insert_text_mode_support: Option<ClientCompletionItemInsertTextModeOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client has support for completion item label
    /// details (see also `CompletionItemLabelDetails`).
    ///
    /// @since 3.17.0
    pub label_details_support: Option<bool>,
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
/// The client supports the following `CompletionList` specific
/// capabilities.
///
/// @since 3.17.0
pub struct CompletionListCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the following itemDefaults on
    /// a completion list.
    ///
    /// The value lists the supported property names of the
    /// `CompletionList.itemDefaults` object. If omitted
    /// no properties are supported.
    ///
    /// @since 3.17.0
    pub item_defaults: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies whether the client supports `CompletionList.applyKind` to
    /// indicate how supported values from `completionList.itemDefaults`
    /// and `completion` will be combined.
    ///
    /// If a client supports `applyKind` it must support it for all fields
    /// that it supports that are listed in `CompletionList.applyKind`. This
    /// means when clients add support for new/future fields in completion
    /// items the MUST also support merge for them if those fields are
    /// defined in `CompletionList.applyKind`.
    ///
    /// @since 3.18.0
    pub apply_kind_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientSignatureInformationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the following content formats for the documentation
    /// property. The order describes the preferred format of the client.
    pub documentation_format: Option<Vec<MarkupKind>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client capabilities specific to parameter information.
    pub parameter_information: Option<ClientSignatureParameterInformationOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the `activeParameter` property on `SignatureInformation`
    /// literal.
    ///
    /// @since 3.16.0
    pub active_parameter_support: Option<bool>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the `activeParameter` property on
    /// `SignatureHelp`/`SignatureInformation` being set to `null` to
    /// indicate that no parameter should be active.
    ///
    /// @since 3.18.0
    /// @proposed
    pub no_active_parameter_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCodeActionLiteralOptions {
    /// The code action kind is support with the following value
    /// set.
    pub code_action_kind: ClientCodeActionKindOptions,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCodeActionResolveOptions {
    /// The properties that a client can resolve lazily.
    pub properties: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0 - proposed
pub struct CodeActionTagOptions {
    /// The tags supported by the client.
    pub value_set: Vec<CodeActionTag>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCodeLensResolveOptions {
    /// The properties that a client can resolve lazily.
    pub properties: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientFoldingRangeKindOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The folding range kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    pub value_set: Option<Vec<FoldingRangeKind>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientFoldingRangeOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// If set, the client signals that it supports setting collapsedText on
    /// folding ranges to display custom labels instead of the default text.
    ///
    /// @since 3.17.0
    pub collapsed_text: Option<bool>,
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
pub struct ClientSemanticTokensRequestOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client will send the `textDocument/semanticTokens/range` request if
    /// the server provides a corresponding handler.
    pub range: Option<Union2<bool, serde_json::Value>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client will send the `textDocument/semanticTokens/full` request if
    /// the server provides a corresponding handler.
    pub full: Option<Union2<bool, ClientSemanticTokensRequestFullDelta>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientInlayHintResolveOptions {
    /// The properties that a client can resolve lazily.
    pub properties: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientShowMessageActionItemOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports additional attributes which
    /// are preserved and send back to the server in the
    /// request's response.
    pub additional_properties_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct CompletionItemTagOptions {
    /// The tags supported by the client.
    pub value_set: Vec<CompletionItemTag>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCompletionItemResolveOptions {
    /// The properties that a client can resolve lazily.
    pub properties: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCompletionItemInsertTextModeOptions {
    pub value_set: Vec<InsertTextMode>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientSignatureParameterInformationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports processing label offsets instead of a
    /// simple label string.
    ///
    /// @since 3.14.0
    pub label_offset_support: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCodeActionKindOptions {
    /// The code action kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    pub value_set: Vec<CodeActionKind>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientDiagnosticsTagOptions {
    /// The tags supported by the client.
    pub value_set: Vec<DiagnosticTag>,
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
