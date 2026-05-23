// DO NOT EDIT THIS GENERATED FILE.

#![allow(deprecated)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_codeblock_attributes)]
#![allow(unused_imports)]

use super::*;
use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};

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
    LocationList(Vec<Location>),
}

/// The declaration of a symbol representation as one or many {@link Location locations}.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Declaration {
    Location(Location),
    LocationList(Vec<Location>),
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
    MarkedStringWithLanguage(MarkedStringWithLanguage),
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
pub enum ImplementationRequestResult {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
    LocationList(Vec<Location>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeDefinitionRequestResult {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
    LocationList(Vec<Location>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeclarationRequestResult {
    Declaration(Declaration),
    DeclarationLinkList(Vec<DeclarationLink>),
    LocationList(Vec<Location>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensRequestResult {
    SemanticTokens(SemanticTokens),
    SemanticTokensPartialResult(SemanticTokensPartialResult),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensDeltaRequestResult {
    SemanticTokens(SemanticTokens),
    SemanticTokensDelta(SemanticTokensDelta),
    SemanticTokensPartialResult(SemanticTokensPartialResult),
    SemanticTokensDeltaPartialResult(SemanticTokensDeltaPartialResult),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensRangeRequestResult {
    SemanticTokens(SemanticTokens),
    SemanticTokensPartialResult(SemanticTokensPartialResult),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentDiagnosticRequestResult {
    DocumentDiagnosticReport(DocumentDiagnosticReport),
    DocumentDiagnosticReportPartialResult(DocumentDiagnosticReportPartialResult),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceDiagnosticRequestResult {
    WorkspaceDiagnosticReport(WorkspaceDiagnosticReport),
    WorkspaceDiagnosticReportPartialResult(WorkspaceDiagnosticReportPartialResult),
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineCompletionRequestResult {
    /// `InlineCompletionList`.
    List(InlineCompletionList),
    /// `InlineCompletionItemList`.
    ItemList(Vec<InlineCompletionItem>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionRequestResult {
    /// `CompletionItemList`.
    ItemList(Vec<CompletionItem>),
    /// `CompletionList`.
    List(CompletionList),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinitionRequestResult {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
    LocationList(Vec<Location>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentSymbolRequestResult {
    SymbolInformationList(Vec<SymbolInformation>),
    DocumentSymbolList(Vec<DocumentSymbol>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeActionRequestResultItem {
    Command(Command),
    CodeAction(CodeAction),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceSymbolRequestResult {
    SymbolInformationList(Vec<SymbolInformation>),
    WorkspaceSymbolList(Vec<WorkspaceSymbol>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensRegistrationOptionsRange {
    Boolean(bool),
    Object(serde_json::Value),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensRegistrationOptionsFull {
    Boolean(bool),
    SemanticTokensFullDelta(SemanticTokensFullDelta),
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
    InlayHintLabelPartList(Vec<InlayHintLabelPart>),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlayHintTooltip {
    String(String),
    MarkupContent(MarkupContent),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentDiagnosticReportPartialResultRelatedDocumentsValue {
    FullDocumentDiagnosticReport(FullDocumentDiagnosticReport),
    UnchangedDocumentDiagnosticReport(UnchangedDocumentDiagnosticReport),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookDocumentSyncRegistrationOptionsNotebookSelectorItem {
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
pub enum DidChangeConfigurationRegistrationOptionsSection {
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
    LocationUriOnly(LocationUriOnly),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CancelParamsId {
    Integer(i32),
    String(String),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensOptionsRange {
    Boolean(bool),
    Object(serde_json::Value),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensOptionsFull {
    Boolean(bool),
    SemanticTokensFullDelta(SemanticTokensFullDelta),
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
pub enum RelatedFullDocumentDiagnosticReportRelatedDocumentsValue {
    FullDocumentDiagnosticReport(FullDocumentDiagnosticReport),
    UnchangedDocumentDiagnosticReport(UnchangedDocumentDiagnosticReport),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RelatedUnchangedDocumentDiagnosticReportRelatedDocumentsValue {
    FullDocumentDiagnosticReport(FullDocumentDiagnosticReport),
    UnchangedDocumentDiagnosticReport(UnchangedDocumentDiagnosticReport),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookDocumentSyncOptionsNotebookSelectorItem {
    /// `NotebookDocumentFilterWithNotebook`.
    Notebook(NotebookDocumentFilterWithNotebook),
    /// `NotebookDocumentFilterWithCells`.
    Cells(NotebookDocumentFilterWithCells),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesTextDocumentSync {
    /// `TextDocumentSyncOptions`.
    Options(TextDocumentSyncOptions),
    /// `TextDocumentSyncKind`.
    Kind(TextDocumentSyncKind),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesNotebookDocumentSync {
    /// `NotebookDocumentSyncOptions`.
    Options(NotebookDocumentSyncOptions),
    /// `NotebookDocumentSyncRegistrationOptions`.
    RegistrationOptions(NotebookDocumentSyncRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesHoverProvider {
    Boolean(bool),
    HoverOptions(HoverOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesDeclarationProvider {
    Boolean(bool),
    /// `DeclarationOptions`.
    Options(DeclarationOptions),
    /// `DeclarationRegistrationOptions`.
    RegistrationOptions(DeclarationRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesDefinitionProvider {
    Boolean(bool),
    DefinitionOptions(DefinitionOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesTypeDefinitionProvider {
    Boolean(bool),
    /// `TypeDefinitionOptions`.
    Options(TypeDefinitionOptions),
    /// `TypeDefinitionRegistrationOptions`.
    RegistrationOptions(TypeDefinitionRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesImplementationProvider {
    Boolean(bool),
    /// `ImplementationOptions`.
    Options(ImplementationOptions),
    /// `ImplementationRegistrationOptions`.
    RegistrationOptions(ImplementationRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesReferencesProvider {
    Boolean(bool),
    ReferenceOptions(ReferenceOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesDocumentHighlightProvider {
    Boolean(bool),
    DocumentHighlightOptions(DocumentHighlightOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesDocumentSymbolProvider {
    Boolean(bool),
    DocumentSymbolOptions(DocumentSymbolOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesCodeActionProvider {
    Boolean(bool),
    CodeActionOptions(CodeActionOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesColorProvider {
    Boolean(bool),
    /// `DocumentColorOptions`.
    Options(DocumentColorOptions),
    /// `DocumentColorRegistrationOptions`.
    RegistrationOptions(DocumentColorRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesWorkspaceSymbolProvider {
    Boolean(bool),
    WorkspaceSymbolOptions(WorkspaceSymbolOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesDocumentFormattingProvider {
    Boolean(bool),
    DocumentFormattingOptions(DocumentFormattingOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesDocumentRangeFormattingProvider {
    Boolean(bool),
    DocumentRangeFormattingOptions(DocumentRangeFormattingOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesRenameProvider {
    Boolean(bool),
    RenameOptions(RenameOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesFoldingRangeProvider {
    Boolean(bool),
    /// `FoldingRangeOptions`.
    Options(FoldingRangeOptions),
    /// `FoldingRangeRegistrationOptions`.
    RegistrationOptions(FoldingRangeRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesSelectionRangeProvider {
    Boolean(bool),
    /// `SelectionRangeOptions`.
    Options(SelectionRangeOptions),
    /// `SelectionRangeRegistrationOptions`.
    RegistrationOptions(SelectionRangeRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesCallHierarchyProvider {
    Boolean(bool),
    /// `CallHierarchyOptions`.
    Options(CallHierarchyOptions),
    /// `CallHierarchyRegistrationOptions`.
    RegistrationOptions(CallHierarchyRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesLinkedEditingRangeProvider {
    Boolean(bool),
    /// `LinkedEditingRangeOptions`.
    Options(LinkedEditingRangeOptions),
    /// `LinkedEditingRangeRegistrationOptions`.
    RegistrationOptions(LinkedEditingRangeRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesSemanticTokensProvider {
    /// `SemanticTokensOptions`.
    Options(SemanticTokensOptions),
    /// `SemanticTokensRegistrationOptions`.
    RegistrationOptions(SemanticTokensRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesMonikerProvider {
    Boolean(bool),
    /// `MonikerOptions`.
    Options(MonikerOptions),
    /// `MonikerRegistrationOptions`.
    RegistrationOptions(MonikerRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesTypeHierarchyProvider {
    Boolean(bool),
    /// `TypeHierarchyOptions`.
    Options(TypeHierarchyOptions),
    /// `TypeHierarchyRegistrationOptions`.
    RegistrationOptions(TypeHierarchyRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesInlineValueProvider {
    Boolean(bool),
    /// `InlineValueOptions`.
    Options(InlineValueOptions),
    /// `InlineValueRegistrationOptions`.
    RegistrationOptions(InlineValueRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesInlayHintProvider {
    Boolean(bool),
    /// `InlayHintOptions`.
    Options(InlayHintOptions),
    /// `InlayHintRegistrationOptions`.
    RegistrationOptions(InlayHintRegistrationOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesDiagnosticProvider {
    /// `DiagnosticOptions`.
    Options(DiagnosticOptions),
    /// `DiagnosticRegistrationOptions`.
    RegistrationOptions(DiagnosticRegistrationOptions),
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerCapabilitiesInlineCompletionProvider {
    Boolean(bool),
    InlineCompletionOptions(InlineCompletionOptions),
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
pub enum NotebookDocumentFilterWithNotebookNotebook {
    String(String),
    NotebookDocumentFilter(NotebookDocumentFilter),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookDocumentFilterWithCellsNotebook {
    String(String),
    NotebookDocumentFilter(NotebookDocumentFilter),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentSyncOptionsSave {
    Boolean(bool),
    SaveOptions(SaveOptions),
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceOptionsTextDocumentContent {
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
pub enum NotebookCellTextDocumentFilterNotebook {
    String(String),
    NotebookDocumentFilter(NotebookDocumentFilter),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceFoldersServerCapabilitiesChangeNotifications {
    String(String),
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RelativePatternBaseUri {
    WorkspaceFolder(WorkspaceFolder),
    Uri(Uri),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientSemanticTokensRequestOptionsRange {
    Boolean(bool),
    Object(serde_json::Value),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientSemanticTokensRequestOptionsFull {
    Boolean(bool),
    ClientSemanticTokensRequestFullDelta(ClientSemanticTokensRequestFullDelta),
}
