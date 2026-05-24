// DO NOT EDIT THIS GENERATED FILE.

#![allow(deprecated)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_codeblock_attributes)]
#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;

/// The definition of a symbol represented as one or many {@link Location locations}.
/// For most programming languages there is only one location at which a symbol is
/// defined.
///
/// Servers should prefer returning `DefinitionLink` over `Definition` if supported
/// by the client.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Definition {
    Location(Location),
    /// `LocationList`.
    List(Vec<Location>),
}


/// The declaration of a symbol representation as one or many {@link Location locations}.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Declaration {
    Location(Location),
    /// `LocationList`.
    List(Vec<Location>),
}


/// Inline value information can be provided by different means:
/// - directly as a text value (class InlineValueText).
/// - as a name to use for a variable lookup (class InlineValueVariableLookup)
/// - as an evaluatable expression (class InlineValueEvaluatableExpression)
/// The InlineValue types combines all inline value types into one type.
///
/// @since 3.17.0
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineValue {
    /// `InlineValueText`.
    Text(InlineValueText),
    /// `InlineValueVariableLookup`.
    VariableLookup(InlineValueVariableLookup),
    /// `InlineValueEvaluatableExpression`.
    EvaluatableExpression(InlineValueEvaluatableExpression),
}


/// The result of a document diagnostic pull request. A report can
/// either be a full report containing all diagnostics for the
/// requested document or an unchanged report indicating that nothing
/// has changed in terms of diagnostics in comparison to the last
/// pull request.
///
/// @since 3.17.0
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentDiagnosticReport {
    /// `RelatedFullDocumentDiagnosticReport`.
    Full(RelatedFullDocumentDiagnosticReport),
    /// `RelatedUnchangedDocumentDiagnosticReport`.
    Unchanged(RelatedUnchangedDocumentDiagnosticReport),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrepareRenameResult {
    Range(Range),
    PrepareRenamePlaceholder(PrepareRenamePlaceholder),
    PrepareRenameDefaultBehavior(PrepareRenameDefaultBehavior),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ProgressToken {
    Integer(i32),
    String(String),
}


/// A workspace diagnostic document report.
///
/// @since 3.17.0
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceDocumentDiagnosticReport {
    /// `WorkspaceFullDocumentDiagnosticReport`.
    Full(WorkspaceFullDocumentDiagnosticReport),
    /// `WorkspaceUnchangedDocumentDiagnosticReport`.
    Unchanged(WorkspaceUnchangedDocumentDiagnosticReport),
}


/// An event describing a change to a text document. If only a text is provided
/// it is considered to be the full content of the document.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentContentChangeEvent {
    /// `TextDocumentContentChangePartial`.
    Partial(TextDocumentContentChangePartial),
    /// `TextDocumentContentChangeWholeDocument`.
    WholeDocument(TextDocumentContentChangeWholeDocument),
}


/// MarkedString can be used to render human readable text. It is either a markdown string
/// or a code-block that provides a language and a code snippet. The language identifier
/// is semantically equal to the optional language identifier in fenced code blocks in GitHub
/// issues. See https://help.github.com/articles/creating-and-highlighting-code-blocks/#syntax-highlighting
///
/// The pair of a language and a value is an equivalent to markdown:
/// ```${language}
/// ${value}
/// ```
///
/// Note that markdown strings will be sanitized - that means html will be escaped.
/// @deprecated use MarkupContent instead.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MarkedString {
    String(String),
    /// `MarkedStringWithLanguage`.
    WithLanguage(MarkedStringWithLanguage),
}


/// A document filter describes a top level text document or
/// a notebook cell document.
///
/// @since 3.17.0 - support for NotebookCellTextDocumentFilter.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentFilter {
    TextDocumentFilter(TextDocumentFilter),
    NotebookCellTextDocumentFilter(NotebookCellTextDocumentFilter),
}


/// The glob pattern. Either a string pattern or a relative pattern.
///
/// @since 3.17.0
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GlobPattern {
    Pattern(Pattern),
    RelativePattern(RelativePattern),
}


/// A document filter denotes a document by different properties like
/// the {@link TextDocument.languageId language}, the {@link Uri.scheme scheme} of
/// its resource, or a glob-pattern that is applied to the {@link TextDocument.fileName path}.
///
/// Glob patterns can have the following syntax:
/// - `*` to match one or more characters in a path segment
/// - `?` to match on one character in a path segment
/// - `**` to match any number of path segments, including none
/// - `{}` to group sub patterns into an OR expression. (e.g. `**​/*.{ts,js}` matches all TypeScript and JavaScript files)
/// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
/// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
///
/// @sample A language filter that applies to typescript files on disk: `{ language: 'typescript', scheme: 'file' }`
/// @sample A language filter that applies to all package.json paths: `{ language: 'json', pattern: '**package.json' }`
///
/// @since 3.17.0
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentFilter {
    /// `TextDocumentFilterLanguage`.
    Language(TextDocumentFilterLanguage),
    /// `TextDocumentFilterScheme`.
    Scheme(TextDocumentFilterScheme),
    /// `TextDocumentFilterPattern`.
    Pattern(TextDocumentFilterPattern),
}


/// A notebook document filter denotes a notebook document by
/// different properties. The properties will be match
/// against the notebook's URI (same as with documents)
///
/// @since 3.17.0
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookDocumentFilter {
    /// `NotebookDocumentFilterNotebookType`.
    NotebookType(NotebookDocumentFilterNotebookType),
    /// `NotebookDocumentFilterScheme`.
    Scheme(NotebookDocumentFilterScheme),
    /// `NotebookDocumentFilterPattern`.
    Pattern(NotebookDocumentFilterPattern),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImplementationResponse {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
    LocationList(Vec<Location>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeDefinitionResponse {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
    LocationList(Vec<Location>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeclarationResponse {
    Declaration(Declaration),
    DeclarationLinkList(Vec<DeclarationLink>),
    LocationList(Vec<Location>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensResponse {
    SemanticTokens(SemanticTokens),
    /// `SemanticTokensPartialResult`.
    PartialResult(SemanticTokensPartialResult),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensDeltaResponse {
    SemanticTokens(SemanticTokens),
    /// `SemanticTokensDelta`.
    Delta(SemanticTokensDelta),
    /// `SemanticTokensPartialResult`.
    PartialResult(SemanticTokensPartialResult),
    /// `SemanticTokensDeltaPartialResult`.
    DeltaPartialResult(SemanticTokensDeltaPartialResult),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensRangeResponse {
    SemanticTokens(SemanticTokens),
    /// `SemanticTokensPartialResult`.
    PartialResult(SemanticTokensPartialResult),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentDiagnosticResponse {
    DocumentDiagnosticReport(DocumentDiagnosticReport),
    /// `DocumentDiagnosticReportPartialResult`.
    PartialResult(DocumentDiagnosticReportPartialResult),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceDiagnosticResponse {
    WorkspaceDiagnosticReport(WorkspaceDiagnosticReport),
    /// `WorkspaceDiagnosticReportPartialResult`.
    PartialResult(WorkspaceDiagnosticReportPartialResult),
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineCompletionResponse {
    /// `InlineCompletionList`.
    List(InlineCompletionList),
    /// `InlineCompletionItemList`.
    ItemList(Vec<InlineCompletionItem>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionResponse {
    /// `CompletionItemList`.
    ItemList(Vec<CompletionItem>),
    /// `CompletionList`.
    List(CompletionList),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinitionResponse {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
    LocationList(Vec<Location>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentSymbolResponse {
    SymbolInformationList(Vec<SymbolInformation>),
    DocumentSymbolList(Vec<DocumentSymbol>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeActionResponseItem {
    Command(Command),
    CodeAction(CodeAction),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceSymbolResponse {
    SymbolInformationList(Vec<SymbolInformation>),
    WorkspaceSymbolList(Vec<WorkspaceSymbol>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensRange {
    Bool(bool),
    Object(serde_json::Value),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensFull {
    Bool(bool),
    /// `SemanticTokensFullDelta`.
    Delta(SemanticTokensFullDelta),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceEditDocumentChangesItem {
    TextDocumentEdit(TextDocumentEdit),
    CreateFile(CreateFile),
    RenameFile(RenameFile),
    DeleteFile(DeleteFile),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlayHintLabel {
    String(String),
    /// `InlayHintLabelPartList`.
    PartList(Vec<InlayHintLabelPart>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlayHintTooltip {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RelatedDocumentDiagnosticReport {
    /// `FullDocumentDiagnosticReport`.
    Full(FullDocumentDiagnosticReport),
    /// `UnchangedDocumentDiagnosticReport`.
    Unchanged(UnchangedDocumentDiagnosticReport),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookSelectorItem {
    /// `NotebookDocumentFilterWithNotebook`.
    Notebook(NotebookDocumentFilterWithNotebook),
    /// `NotebookDocumentFilterWithCells`.
    Cells(NotebookDocumentFilterWithCells),
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineCompletionItemInsertText {
    String(String),
    StringValue(StringValue),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DidChangeConfigurationSection {
    String(String),
    StringList(Vec<String>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionItemDocumentation {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionItemTextEdit {
    TextEdit(TextEdit),
    InsertReplaceEdit(InsertReplaceEdit),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContents {
    MarkupContent(MarkupContent),
    MarkedString(MarkedString),
    MarkedStringList(Vec<MarkedString>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceSymbolLocation {
    Location(Location),
    /// `LocationUriOnly`.
    UriOnly(LocationUriOnly),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelParamsId {
    Integer(i32),
    String(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentEditEditsItem {
    TextEdit(TextEdit),
    AnnotatedTextEdit(AnnotatedTextEdit),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlayHintLabelPartTooltip {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentSync {
    /// `TextDocumentSyncOptions`.
    Options(TextDocumentSyncOptions),
    /// `TextDocumentSyncKind`.
    Kind(TextDocumentSyncKind),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookDocumentSync {
    /// `NotebookDocumentSyncOptions`.
    Options(NotebookDocumentSyncOptions),
    /// `NotebookDocumentSyncRegistrationOptions`.
    RegistrationOptions(NotebookDocumentSyncRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverProvider {
    Bool(bool),
    /// `HoverOptions`.
    Options(HoverOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeclarationProvider {
    Bool(bool),
    /// `DeclarationOptions`.
    Options(DeclarationOptions),
    /// `DeclarationRegistrationOptions`.
    RegistrationOptions(DeclarationRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinitionProvider {
    Bool(bool),
    /// `DefinitionOptions`.
    Options(DefinitionOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeDefinitionProvider {
    Bool(bool),
    /// `TypeDefinitionOptions`.
    Options(TypeDefinitionOptions),
    /// `TypeDefinitionRegistrationOptions`.
    RegistrationOptions(TypeDefinitionRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImplementationProvider {
    Bool(bool),
    /// `ImplementationOptions`.
    Options(ImplementationOptions),
    /// `ImplementationRegistrationOptions`.
    RegistrationOptions(ImplementationRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReferencesProvider {
    Bool(bool),
    ReferenceOptions(ReferenceOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentHighlightProvider {
    Bool(bool),
    /// `DocumentHighlightOptions`.
    Options(DocumentHighlightOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentSymbolProvider {
    Bool(bool),
    /// `DocumentSymbolOptions`.
    Options(DocumentSymbolOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeActionProvider {
    Bool(bool),
    /// `CodeActionOptions`.
    Options(CodeActionOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ColorProvider {
    Bool(bool),
    /// `DocumentColorOptions`.
    Options(DocumentColorOptions),
    /// `DocumentColorRegistrationOptions`.
    RegistrationOptions(DocumentColorRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceSymbolProvider {
    Bool(bool),
    /// `WorkspaceSymbolOptions`.
    Options(WorkspaceSymbolOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentFormattingProvider {
    Bool(bool),
    /// `DocumentFormattingOptions`.
    Options(DocumentFormattingOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentRangeFormattingProvider {
    Bool(bool),
    /// `DocumentRangeFormattingOptions`.
    Options(DocumentRangeFormattingOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenameProvider {
    Bool(bool),
    /// `RenameOptions`.
    Options(RenameOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FoldingRangeProvider {
    Bool(bool),
    /// `FoldingRangeOptions`.
    Options(FoldingRangeOptions),
    /// `FoldingRangeRegistrationOptions`.
    RegistrationOptions(FoldingRangeRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SelectionRangeProvider {
    Bool(bool),
    /// `SelectionRangeOptions`.
    Options(SelectionRangeOptions),
    /// `SelectionRangeRegistrationOptions`.
    RegistrationOptions(SelectionRangeRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CallHierarchyProvider {
    Bool(bool),
    /// `CallHierarchyOptions`.
    Options(CallHierarchyOptions),
    /// `CallHierarchyRegistrationOptions`.
    RegistrationOptions(CallHierarchyRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LinkedEditingRangeProvider {
    Bool(bool),
    /// `LinkedEditingRangeOptions`.
    Options(LinkedEditingRangeOptions),
    /// `LinkedEditingRangeRegistrationOptions`.
    RegistrationOptions(LinkedEditingRangeRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensProvider {
    /// `SemanticTokensOptions`.
    Options(SemanticTokensOptions),
    /// `SemanticTokensRegistrationOptions`.
    RegistrationOptions(SemanticTokensRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MonikerProvider {
    Bool(bool),
    /// `MonikerOptions`.
    Options(MonikerOptions),
    /// `MonikerRegistrationOptions`.
    RegistrationOptions(MonikerRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeHierarchyProvider {
    Bool(bool),
    /// `TypeHierarchyOptions`.
    Options(TypeHierarchyOptions),
    /// `TypeHierarchyRegistrationOptions`.
    RegistrationOptions(TypeHierarchyRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineValueProvider {
    Bool(bool),
    /// `InlineValueOptions`.
    Options(InlineValueOptions),
    /// `InlineValueRegistrationOptions`.
    RegistrationOptions(InlineValueRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlayHintProvider {
    Bool(bool),
    /// `InlayHintOptions`.
    Options(InlayHintOptions),
    /// `InlayHintRegistrationOptions`.
    RegistrationOptions(InlayHintRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiagnosticProvider {
    /// `DiagnosticOptions`.
    Options(DiagnosticOptions),
    /// `DiagnosticRegistrationOptions`.
    RegistrationOptions(DiagnosticRegistrationOptions),
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineCompletionProvider {
    Bool(bool),
    /// `InlineCompletionOptions`.
    Options(InlineCompletionOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiagnosticCode {
    Integer(i32),
    String(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionItemDefaultsEditRange {
    Range(Range),
    EditRangeWithInsertReplace(EditRangeWithInsertReplace),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SignatureInformationDocumentation {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookDocumentFilterNotebook {
    String(String),
    NotebookDocumentFilter(NotebookDocumentFilter),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentSyncSave {
    Bool(bool),
    SaveOptions(SaveOptions),
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceTextDocumentContent {
    /// `TextDocumentContentOptions`.
    Options(TextDocumentContentOptions),
    /// `TextDocumentContentRegistrationOptions`.
    RegistrationOptions(TextDocumentContentRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterInformationLabel {
    String(String),
    Tuple((u32, u32)),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterInformationDocumentation {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceFoldersChangeNotifications {
    String(String),
    Bool(bool),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RelativePatternBaseUri {
    WorkspaceFolder(WorkspaceFolder),
    Uri(Uri),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientSemanticTokensRequestRange {
    Bool(bool),
    Object(serde_json::Value),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientSemanticTokensRequestFull {
    Bool(bool),
    /// `ClientSemanticTokensRequestFullDelta`.
    Delta(ClientSemanticTokensRequestFullDelta),
}