// DO NOT EDIT THIS GENERATED FILE.

#![allow(deprecated)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_codeblock_attributes)]
#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// The definition of a symbol represented as one or many {@link Location locations}.
/// For most programming languages there is only one location at which a symbol is
/// defined.
///
/// Servers should prefer returning `DefinitionLink` over `Definition` if supported
/// by the client.
pub enum Definition {
    Location(Location),
    /// `LocationList`.
    List(Vec<Location>),
}

impl From<Location> for Definition {
    fn from(value: Location) -> Self {
        Self::Location(value)
    }
}

impl From<Vec<Location>> for Definition {
    fn from(value: Vec<Location>) -> Self {
        Self::List(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// The declaration of a symbol representation as one or many {@link Location locations}.
pub enum Declaration {
    Location(Location),
    /// `LocationList`.
    List(Vec<Location>),
}

impl From<Location> for Declaration {
    fn from(value: Location) -> Self {
        Self::Location(value)
    }
}

impl From<Vec<Location>> for Declaration {
    fn from(value: Vec<Location>) -> Self {
        Self::List(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// Inline value information can be provided by different means:
/// - directly as a text value (class InlineValueText).
/// - as a name to use for a variable lookup (class InlineValueVariableLookup)
/// - as an evaluatable expression (class InlineValueEvaluatableExpression)
/// The InlineValue types combines all inline value types into one type.
///
/// @since 3.17.0
pub enum InlineValue {
    /// `InlineValueText`.
    Text(InlineValueText),
    /// `InlineValueVariableLookup`.
    VariableLookup(InlineValueVariableLookup),
    /// `InlineValueEvaluatableExpression`.
    EvaluatableExpression(InlineValueEvaluatableExpression),
}

impl From<InlineValueText> for InlineValue {
    fn from(value: InlineValueText) -> Self {
        Self::Text(value)
    }
}

impl From<InlineValueVariableLookup> for InlineValue {
    fn from(value: InlineValueVariableLookup) -> Self {
        Self::VariableLookup(value)
    }
}

impl From<InlineValueEvaluatableExpression> for InlineValue {
    fn from(value: InlineValueEvaluatableExpression) -> Self {
        Self::EvaluatableExpression(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// The result of a document diagnostic pull request. A report can
/// either be a full report containing all diagnostics for the
/// requested document or an unchanged report indicating that nothing
/// has changed in terms of diagnostics in comparison to the last
/// pull request.
///
/// @since 3.17.0
pub enum DocumentDiagnosticReport {
    /// `RelatedFullDocumentDiagnosticReport`.
    Full(RelatedFullDocumentDiagnosticReport),
    /// `RelatedUnchangedDocumentDiagnosticReport`.
    Unchanged(RelatedUnchangedDocumentDiagnosticReport),
}

impl From<RelatedFullDocumentDiagnosticReport> for DocumentDiagnosticReport {
    fn from(value: RelatedFullDocumentDiagnosticReport) -> Self {
        Self::Full(value)
    }
}

impl From<RelatedUnchangedDocumentDiagnosticReport> for DocumentDiagnosticReport {
    fn from(value: RelatedUnchangedDocumentDiagnosticReport) -> Self {
        Self::Unchanged(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrepareRenameResult {
    Range(Range),
    PrepareRenamePlaceholder(PrepareRenamePlaceholder),
    PrepareRenameDefaultBehavior(PrepareRenameDefaultBehavior),
}

impl From<Range> for PrepareRenameResult {
    fn from(value: Range) -> Self {
        Self::Range(value)
    }
}

impl From<PrepareRenamePlaceholder> for PrepareRenameResult {
    fn from(value: PrepareRenamePlaceholder) -> Self {
        Self::PrepareRenamePlaceholder(value)
    }
}

impl From<PrepareRenameDefaultBehavior> for PrepareRenameResult {
    fn from(value: PrepareRenameDefaultBehavior) -> Self {
        Self::PrepareRenameDefaultBehavior(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NumberOrString {
    Integer(i32),
    String(String),
}

impl From<i32> for NumberOrString {
    fn from(value: i32) -> Self {
        Self::Integer(value)
    }
}

impl From<String> for NumberOrString {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

pub type ProgressToken = NumberOrString;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// A workspace diagnostic document report.
///
/// @since 3.17.0
pub enum WorkspaceDocumentDiagnosticReport {
    /// `WorkspaceFullDocumentDiagnosticReport`.
    Full(WorkspaceFullDocumentDiagnosticReport),
    /// `WorkspaceUnchangedDocumentDiagnosticReport`.
    Unchanged(WorkspaceUnchangedDocumentDiagnosticReport),
}

impl From<WorkspaceFullDocumentDiagnosticReport> for WorkspaceDocumentDiagnosticReport {
    fn from(value: WorkspaceFullDocumentDiagnosticReport) -> Self {
        Self::Full(value)
    }
}

impl From<WorkspaceUnchangedDocumentDiagnosticReport> for WorkspaceDocumentDiagnosticReport {
    fn from(value: WorkspaceUnchangedDocumentDiagnosticReport) -> Self {
        Self::Unchanged(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// An event describing a change to a text document. If only a text is provided
/// it is considered to be the full content of the document.
pub enum TextDocumentContentChangeEvent {
    /// `TextDocumentContentChangePartial`.
    Partial(TextDocumentContentChangePartial),
    /// `TextDocumentContentChangeWholeDocument`.
    WholeDocument(TextDocumentContentChangeWholeDocument),
}

impl From<TextDocumentContentChangePartial> for TextDocumentContentChangeEvent {
    fn from(value: TextDocumentContentChangePartial) -> Self {
        Self::Partial(value)
    }
}

impl From<TextDocumentContentChangeWholeDocument> for TextDocumentContentChangeEvent {
    fn from(value: TextDocumentContentChangeWholeDocument) -> Self {
        Self::WholeDocument(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
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
pub enum MarkedString {
    String(String),
    /// `MarkedStringWithLanguage`.
    WithLanguage(MarkedStringWithLanguage),
}

impl From<String> for MarkedString {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<MarkedStringWithLanguage> for MarkedString {
    fn from(value: MarkedStringWithLanguage) -> Self {
        Self::WithLanguage(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// A document filter describes a top level text document or
/// a notebook cell document.
///
/// @since 3.17.0 - support for NotebookCellTextDocumentFilter.
pub enum DocumentFilter {
    TextDocumentFilter(TextDocumentFilter),
    NotebookCellTextDocumentFilter(NotebookCellTextDocumentFilter),
}

impl From<TextDocumentFilter> for DocumentFilter {
    fn from(value: TextDocumentFilter) -> Self {
        Self::TextDocumentFilter(value)
    }
}

impl From<NotebookCellTextDocumentFilter> for DocumentFilter {
    fn from(value: NotebookCellTextDocumentFilter) -> Self {
        Self::NotebookCellTextDocumentFilter(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// The glob pattern. Either a string pattern or a relative pattern.
///
/// @since 3.17.0
pub enum GlobPattern {
    Pattern(Pattern),
    RelativePattern(RelativePattern),
}

impl From<Pattern> for GlobPattern {
    fn from(value: Pattern) -> Self {
        Self::Pattern(value)
    }
}

impl From<RelativePattern> for GlobPattern {
    fn from(value: RelativePattern) -> Self {
        Self::RelativePattern(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
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
pub enum TextDocumentFilter {
    /// `TextDocumentFilterLanguage`.
    Language(TextDocumentFilterLanguage),
    /// `TextDocumentFilterScheme`.
    Scheme(TextDocumentFilterScheme),
    /// `TextDocumentFilterPattern`.
    Pattern(TextDocumentFilterPattern),
}

impl From<TextDocumentFilterLanguage> for TextDocumentFilter {
    fn from(value: TextDocumentFilterLanguage) -> Self {
        Self::Language(value)
    }
}

impl From<TextDocumentFilterScheme> for TextDocumentFilter {
    fn from(value: TextDocumentFilterScheme) -> Self {
        Self::Scheme(value)
    }
}

impl From<TextDocumentFilterPattern> for TextDocumentFilter {
    fn from(value: TextDocumentFilterPattern) -> Self {
        Self::Pattern(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
/// A notebook document filter denotes a notebook document by
/// different properties. The properties will be match
/// against the notebook's URI (same as with documents)
///
/// @since 3.17.0
pub enum NotebookDocumentFilter {
    /// `NotebookDocumentFilterNotebookType`.
    NotebookType(NotebookDocumentFilterNotebookType),
    /// `NotebookDocumentFilterScheme`.
    Scheme(NotebookDocumentFilterScheme),
    /// `NotebookDocumentFilterPattern`.
    Pattern(NotebookDocumentFilterPattern),
}

impl From<NotebookDocumentFilterNotebookType> for NotebookDocumentFilter {
    fn from(value: NotebookDocumentFilterNotebookType) -> Self {
        Self::NotebookType(value)
    }
}

impl From<NotebookDocumentFilterScheme> for NotebookDocumentFilter {
    fn from(value: NotebookDocumentFilterScheme) -> Self {
        Self::Scheme(value)
    }
}

impl From<NotebookDocumentFilterPattern> for NotebookDocumentFilter {
    fn from(value: NotebookDocumentFilterPattern) -> Self {
        Self::Pattern(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImplementationResponse {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
    LocationList(Vec<Location>),
}

impl From<Definition> for ImplementationResponse {
    fn from(value: Definition) -> Self {
        Self::Definition(value)
    }
}

impl From<Vec<DefinitionLink>> for ImplementationResponse {
    fn from(value: Vec<DefinitionLink>) -> Self {
        Self::DefinitionLinkList(value)
    }
}

impl From<Vec<Location>> for ImplementationResponse {
    fn from(value: Vec<Location>) -> Self {
        Self::LocationList(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TypeDefinitionResponse {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
    LocationList(Vec<Location>),
}

impl From<Definition> for TypeDefinitionResponse {
    fn from(value: Definition) -> Self {
        Self::Definition(value)
    }
}

impl From<Vec<DefinitionLink>> for TypeDefinitionResponse {
    fn from(value: Vec<DefinitionLink>) -> Self {
        Self::DefinitionLinkList(value)
    }
}

impl From<Vec<Location>> for TypeDefinitionResponse {
    fn from(value: Vec<Location>) -> Self {
        Self::LocationList(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeclarationResponse {
    Declaration(Declaration),
    DeclarationLinkList(Vec<DeclarationLink>),
    LocationList(Vec<Location>),
}

impl From<Declaration> for DeclarationResponse {
    fn from(value: Declaration) -> Self {
        Self::Declaration(value)
    }
}

impl From<Vec<DeclarationLink>> for DeclarationResponse {
    fn from(value: Vec<DeclarationLink>) -> Self {
        Self::DeclarationLinkList(value)
    }
}

impl From<Vec<Location>> for DeclarationResponse {
    fn from(value: Vec<Location>) -> Self {
        Self::LocationList(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensResponse {
    SemanticTokens(SemanticTokens),
    /// `SemanticTokensPartialResult`.
    PartialResult(SemanticTokensPartialResult),
}

impl From<SemanticTokens> for SemanticTokensResponse {
    fn from(value: SemanticTokens) -> Self {
        Self::SemanticTokens(value)
    }
}

impl From<SemanticTokensPartialResult> for SemanticTokensResponse {
    fn from(value: SemanticTokensPartialResult) -> Self {
        Self::PartialResult(value)
    }
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

impl From<SemanticTokens> for SemanticTokensDeltaResponse {
    fn from(value: SemanticTokens) -> Self {
        Self::SemanticTokens(value)
    }
}

impl From<SemanticTokensDelta> for SemanticTokensDeltaResponse {
    fn from(value: SemanticTokensDelta) -> Self {
        Self::Delta(value)
    }
}

impl From<SemanticTokensPartialResult> for SemanticTokensDeltaResponse {
    fn from(value: SemanticTokensPartialResult) -> Self {
        Self::PartialResult(value)
    }
}

impl From<SemanticTokensDeltaPartialResult> for SemanticTokensDeltaResponse {
    fn from(value: SemanticTokensDeltaPartialResult) -> Self {
        Self::DeltaPartialResult(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensRangeResponse {
    SemanticTokens(SemanticTokens),
    /// `SemanticTokensPartialResult`.
    PartialResult(SemanticTokensPartialResult),
}

impl From<SemanticTokens> for SemanticTokensRangeResponse {
    fn from(value: SemanticTokens) -> Self {
        Self::SemanticTokens(value)
    }
}

impl From<SemanticTokensPartialResult> for SemanticTokensRangeResponse {
    fn from(value: SemanticTokensPartialResult) -> Self {
        Self::PartialResult(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentDiagnosticResponse {
    DocumentDiagnosticReport(DocumentDiagnosticReport),
    /// `DocumentDiagnosticReportPartialResult`.
    PartialResult(DocumentDiagnosticReportPartialResult),
}

impl From<DocumentDiagnosticReport> for DocumentDiagnosticResponse {
    fn from(value: DocumentDiagnosticReport) -> Self {
        Self::DocumentDiagnosticReport(value)
    }
}

impl From<DocumentDiagnosticReportPartialResult> for DocumentDiagnosticResponse {
    fn from(value: DocumentDiagnosticReportPartialResult) -> Self {
        Self::PartialResult(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceDiagnosticResponse {
    WorkspaceDiagnosticReport(WorkspaceDiagnosticReport),
    /// `WorkspaceDiagnosticReportPartialResult`.
    PartialResult(WorkspaceDiagnosticReportPartialResult),
}

impl From<WorkspaceDiagnosticReport> for WorkspaceDiagnosticResponse {
    fn from(value: WorkspaceDiagnosticReport) -> Self {
        Self::WorkspaceDiagnosticReport(value)
    }
}

impl From<WorkspaceDiagnosticReportPartialResult> for WorkspaceDiagnosticResponse {
    fn from(value: WorkspaceDiagnosticReportPartialResult) -> Self {
        Self::PartialResult(value)
    }
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

#[cfg(feature = "proposed")]
impl From<InlineCompletionList> for InlineCompletionResponse {
    fn from(value: InlineCompletionList) -> Self {
        Self::List(value)
    }
}

#[cfg(feature = "proposed")]
impl From<Vec<InlineCompletionItem>> for InlineCompletionResponse {
    fn from(value: Vec<InlineCompletionItem>) -> Self {
        Self::ItemList(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionResponse {
    /// `CompletionItemList`.
    ItemList(Vec<CompletionItem>),
    /// `CompletionList`.
    List(CompletionList),
}

impl From<Vec<CompletionItem>> for CompletionResponse {
    fn from(value: Vec<CompletionItem>) -> Self {
        Self::ItemList(value)
    }
}

impl From<CompletionList> for CompletionResponse {
    fn from(value: CompletionList) -> Self {
        Self::List(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinitionResponse {
    Definition(Definition),
    DefinitionLinkList(Vec<DefinitionLink>),
    LocationList(Vec<Location>),
}

impl From<Definition> for DefinitionResponse {
    fn from(value: Definition) -> Self {
        Self::Definition(value)
    }
}

impl From<Vec<DefinitionLink>> for DefinitionResponse {
    fn from(value: Vec<DefinitionLink>) -> Self {
        Self::DefinitionLinkList(value)
    }
}

impl From<Vec<Location>> for DefinitionResponse {
    fn from(value: Vec<Location>) -> Self {
        Self::LocationList(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentSymbolResponse {
    SymbolInformationList(Vec<SymbolInformation>),
    DocumentSymbolList(Vec<DocumentSymbol>),
}

impl From<Vec<SymbolInformation>> for DocumentSymbolResponse {
    fn from(value: Vec<SymbolInformation>) -> Self {
        Self::SymbolInformationList(value)
    }
}

impl From<Vec<DocumentSymbol>> for DocumentSymbolResponse {
    fn from(value: Vec<DocumentSymbol>) -> Self {
        Self::DocumentSymbolList(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeActionResponseItem {
    Command(Command),
    CodeAction(CodeAction),
}

impl From<Command> for CodeActionResponseItem {
    fn from(value: Command) -> Self {
        Self::Command(value)
    }
}

impl From<CodeAction> for CodeActionResponseItem {
    fn from(value: CodeAction) -> Self {
        Self::CodeAction(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceSymbolResponse {
    SymbolInformationList(Vec<SymbolInformation>),
    WorkspaceSymbolList(Vec<WorkspaceSymbol>),
}

impl From<Vec<SymbolInformation>> for WorkspaceSymbolResponse {
    fn from(value: Vec<SymbolInformation>) -> Self {
        Self::SymbolInformationList(value)
    }
}

impl From<Vec<WorkspaceSymbol>> for WorkspaceSymbolResponse {
    fn from(value: Vec<WorkspaceSymbol>) -> Self {
        Self::WorkspaceSymbolList(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensRange {
    Bool(bool),
    Object(serde_json::Value),
}

impl From<bool> for SemanticTokensRange {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<serde_json::Value> for SemanticTokensRange {
    fn from(value: serde_json::Value) -> Self {
        Self::Object(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensFull {
    Bool(bool),
    /// `SemanticTokensFullDelta`.
    Delta(SemanticTokensFullDelta),
}

impl From<bool> for SemanticTokensFull {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<SemanticTokensFullDelta> for SemanticTokensFull {
    fn from(value: SemanticTokensFullDelta) -> Self {
        Self::Delta(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceEditDocumentChangesItem {
    TextDocumentEdit(TextDocumentEdit),
    CreateFile(CreateFile),
    RenameFile(RenameFile),
    DeleteFile(DeleteFile),
}

impl From<TextDocumentEdit> for WorkspaceEditDocumentChangesItem {
    fn from(value: TextDocumentEdit) -> Self {
        Self::TextDocumentEdit(value)
    }
}

impl From<CreateFile> for WorkspaceEditDocumentChangesItem {
    fn from(value: CreateFile) -> Self {
        Self::CreateFile(value)
    }
}

impl From<RenameFile> for WorkspaceEditDocumentChangesItem {
    fn from(value: RenameFile) -> Self {
        Self::RenameFile(value)
    }
}

impl From<DeleteFile> for WorkspaceEditDocumentChangesItem {
    fn from(value: DeleteFile) -> Self {
        Self::DeleteFile(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlayHintLabel {
    String(String),
    /// `InlayHintLabelPartList`.
    PartList(Vec<InlayHintLabelPart>),
}

impl From<String> for InlayHintLabel {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<InlayHintLabelPart>> for InlayHintLabel {
    fn from(value: Vec<InlayHintLabelPart>) -> Self {
        Self::PartList(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StringOrMarkupContent {
    String(String),
    MarkupContent(MarkupContent),
}

impl From<String> for StringOrMarkupContent {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<MarkupContent> for StringOrMarkupContent {
    fn from(value: MarkupContent) -> Self {
        Self::MarkupContent(value)
    }
}

pub type InlayHintTooltip = StringOrMarkupContent;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RelatedDocumentDiagnosticReport {
    /// `FullDocumentDiagnosticReport`.
    Full(FullDocumentDiagnosticReport),
    /// `UnchangedDocumentDiagnosticReport`.
    Unchanged(UnchangedDocumentDiagnosticReport),
}

impl From<FullDocumentDiagnosticReport> for RelatedDocumentDiagnosticReport {
    fn from(value: FullDocumentDiagnosticReport) -> Self {
        Self::Full(value)
    }
}

impl From<UnchangedDocumentDiagnosticReport> for RelatedDocumentDiagnosticReport {
    fn from(value: UnchangedDocumentDiagnosticReport) -> Self {
        Self::Unchanged(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookSelectorItem {
    /// `NotebookDocumentFilterWithNotebook`.
    Notebook(NotebookDocumentFilterWithNotebook),
    /// `NotebookDocumentFilterWithCells`.
    Cells(NotebookDocumentFilterWithCells),
}

impl From<NotebookDocumentFilterWithNotebook> for NotebookSelectorItem {
    fn from(value: NotebookDocumentFilterWithNotebook) -> Self {
        Self::Notebook(value)
    }
}

impl From<NotebookDocumentFilterWithCells> for NotebookSelectorItem {
    fn from(value: NotebookDocumentFilterWithCells) -> Self {
        Self::Cells(value)
    }
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineCompletionItemInsertText {
    String(String),
    StringValue(StringValue),
}

#[cfg(feature = "proposed")]
impl From<String> for InlineCompletionItemInsertText {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

#[cfg(feature = "proposed")]
impl From<StringValue> for InlineCompletionItemInsertText {
    fn from(value: StringValue) -> Self {
        Self::StringValue(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DidChangeConfigurationSection {
    String(String),
    StringList(Vec<String>),
}

impl From<String> for DidChangeConfigurationSection {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<Vec<String>> for DidChangeConfigurationSection {
    fn from(value: Vec<String>) -> Self {
        Self::StringList(value)
    }
}

pub type CompletionItemDocumentation = StringOrMarkupContent;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionItemTextEdit {
    TextEdit(TextEdit),
    InsertReplaceEdit(InsertReplaceEdit),
}

impl From<TextEdit> for CompletionItemTextEdit {
    fn from(value: TextEdit) -> Self {
        Self::TextEdit(value)
    }
}

impl From<InsertReplaceEdit> for CompletionItemTextEdit {
    fn from(value: InsertReplaceEdit) -> Self {
        Self::InsertReplaceEdit(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverContents {
    MarkupContent(MarkupContent),
    MarkedString(MarkedString),
    MarkedStringList(Vec<MarkedString>),
}

impl From<MarkupContent> for HoverContents {
    fn from(value: MarkupContent) -> Self {
        Self::MarkupContent(value)
    }
}

impl From<MarkedString> for HoverContents {
    fn from(value: MarkedString) -> Self {
        Self::MarkedString(value)
    }
}

impl From<Vec<MarkedString>> for HoverContents {
    fn from(value: Vec<MarkedString>) -> Self {
        Self::MarkedStringList(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceSymbolLocation {
    Location(Location),
    /// `LocationUriOnly`.
    UriOnly(LocationUriOnly),
}

impl From<Location> for WorkspaceSymbolLocation {
    fn from(value: Location) -> Self {
        Self::Location(value)
    }
}

impl From<LocationUriOnly> for WorkspaceSymbolLocation {
    fn from(value: LocationUriOnly) -> Self {
        Self::UriOnly(value)
    }
}

pub type CancelParamsId = NumberOrString;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentEditEditsItem {
    TextEdit(TextEdit),
    AnnotatedTextEdit(AnnotatedTextEdit),
}

impl From<TextEdit> for TextDocumentEditEditsItem {
    fn from(value: TextEdit) -> Self {
        Self::TextEdit(value)
    }
}

impl From<AnnotatedTextEdit> for TextDocumentEditEditsItem {
    fn from(value: AnnotatedTextEdit) -> Self {
        Self::AnnotatedTextEdit(value)
    }
}

pub type InlayHintLabelPartTooltip = StringOrMarkupContent;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentSync {
    /// `TextDocumentSyncOptions`.
    Options(TextDocumentSyncOptions),
    /// `TextDocumentSyncKind`.
    Kind(TextDocumentSyncKind),
}

impl From<TextDocumentSyncOptions> for TextDocumentSync {
    fn from(value: TextDocumentSyncOptions) -> Self {
        Self::Options(value)
    }
}

impl From<TextDocumentSyncKind> for TextDocumentSync {
    fn from(value: TextDocumentSyncKind) -> Self {
        Self::Kind(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookDocumentSync {
    /// `NotebookDocumentSyncOptions`.
    Options(NotebookDocumentSyncOptions),
    /// `NotebookDocumentSyncRegistrationOptions`.
    RegistrationOptions(NotebookDocumentSyncRegistrationOptions),
}

impl From<NotebookDocumentSyncOptions> for NotebookDocumentSync {
    fn from(value: NotebookDocumentSyncOptions) -> Self {
        Self::Options(value)
    }
}

impl From<NotebookDocumentSyncRegistrationOptions> for NotebookDocumentSync {
    fn from(value: NotebookDocumentSyncRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HoverProvider {
    Bool(bool),
    /// `HoverOptions`.
    Options(HoverOptions),
}

impl From<bool> for HoverProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<HoverOptions> for HoverProvider {
    fn from(value: HoverOptions) -> Self {
        Self::Options(value)
    }
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

impl From<bool> for DeclarationProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<DeclarationOptions> for DeclarationProvider {
    fn from(value: DeclarationOptions) -> Self {
        Self::Options(value)
    }
}

impl From<DeclarationRegistrationOptions> for DeclarationProvider {
    fn from(value: DeclarationRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefinitionProvider {
    Bool(bool),
    /// `DefinitionOptions`.
    Options(DefinitionOptions),
}

impl From<bool> for DefinitionProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<DefinitionOptions> for DefinitionProvider {
    fn from(value: DefinitionOptions) -> Self {
        Self::Options(value)
    }
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

impl From<bool> for TypeDefinitionProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<TypeDefinitionOptions> for TypeDefinitionProvider {
    fn from(value: TypeDefinitionOptions) -> Self {
        Self::Options(value)
    }
}

impl From<TypeDefinitionRegistrationOptions> for TypeDefinitionProvider {
    fn from(value: TypeDefinitionRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
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

impl From<bool> for ImplementationProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<ImplementationOptions> for ImplementationProvider {
    fn from(value: ImplementationOptions) -> Self {
        Self::Options(value)
    }
}

impl From<ImplementationRegistrationOptions> for ImplementationProvider {
    fn from(value: ImplementationRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReferencesProvider {
    Bool(bool),
    ReferenceOptions(ReferenceOptions),
}

impl From<bool> for ReferencesProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<ReferenceOptions> for ReferencesProvider {
    fn from(value: ReferenceOptions) -> Self {
        Self::ReferenceOptions(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentHighlightProvider {
    Bool(bool),
    /// `DocumentHighlightOptions`.
    Options(DocumentHighlightOptions),
}

impl From<bool> for DocumentHighlightProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<DocumentHighlightOptions> for DocumentHighlightProvider {
    fn from(value: DocumentHighlightOptions) -> Self {
        Self::Options(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentSymbolProvider {
    Bool(bool),
    /// `DocumentSymbolOptions`.
    Options(DocumentSymbolOptions),
}

impl From<bool> for DocumentSymbolProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<DocumentSymbolOptions> for DocumentSymbolProvider {
    fn from(value: DocumentSymbolOptions) -> Self {
        Self::Options(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CodeActionProvider {
    Bool(bool),
    /// `CodeActionOptions`.
    Options(CodeActionOptions),
}

impl From<bool> for CodeActionProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<CodeActionOptions> for CodeActionProvider {
    fn from(value: CodeActionOptions) -> Self {
        Self::Options(value)
    }
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

impl From<bool> for ColorProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<DocumentColorOptions> for ColorProvider {
    fn from(value: DocumentColorOptions) -> Self {
        Self::Options(value)
    }
}

impl From<DocumentColorRegistrationOptions> for ColorProvider {
    fn from(value: DocumentColorRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceSymbolProvider {
    Bool(bool),
    /// `WorkspaceSymbolOptions`.
    Options(WorkspaceSymbolOptions),
}

impl From<bool> for WorkspaceSymbolProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<WorkspaceSymbolOptions> for WorkspaceSymbolProvider {
    fn from(value: WorkspaceSymbolOptions) -> Self {
        Self::Options(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentFormattingProvider {
    Bool(bool),
    /// `DocumentFormattingOptions`.
    Options(DocumentFormattingOptions),
}

impl From<bool> for DocumentFormattingProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<DocumentFormattingOptions> for DocumentFormattingProvider {
    fn from(value: DocumentFormattingOptions) -> Self {
        Self::Options(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DocumentRangeFormattingProvider {
    Bool(bool),
    /// `DocumentRangeFormattingOptions`.
    Options(DocumentRangeFormattingOptions),
}

impl From<bool> for DocumentRangeFormattingProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<DocumentRangeFormattingOptions> for DocumentRangeFormattingProvider {
    fn from(value: DocumentRangeFormattingOptions) -> Self {
        Self::Options(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RenameProvider {
    Bool(bool),
    /// `RenameOptions`.
    Options(RenameOptions),
}

impl From<bool> for RenameProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<RenameOptions> for RenameProvider {
    fn from(value: RenameOptions) -> Self {
        Self::Options(value)
    }
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

impl From<bool> for FoldingRangeProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<FoldingRangeOptions> for FoldingRangeProvider {
    fn from(value: FoldingRangeOptions) -> Self {
        Self::Options(value)
    }
}

impl From<FoldingRangeRegistrationOptions> for FoldingRangeProvider {
    fn from(value: FoldingRangeRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
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

impl From<bool> for SelectionRangeProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<SelectionRangeOptions> for SelectionRangeProvider {
    fn from(value: SelectionRangeOptions) -> Self {
        Self::Options(value)
    }
}

impl From<SelectionRangeRegistrationOptions> for SelectionRangeProvider {
    fn from(value: SelectionRangeRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
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

impl From<bool> for CallHierarchyProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<CallHierarchyOptions> for CallHierarchyProvider {
    fn from(value: CallHierarchyOptions) -> Self {
        Self::Options(value)
    }
}

impl From<CallHierarchyRegistrationOptions> for CallHierarchyProvider {
    fn from(value: CallHierarchyRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
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

impl From<bool> for LinkedEditingRangeProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<LinkedEditingRangeOptions> for LinkedEditingRangeProvider {
    fn from(value: LinkedEditingRangeOptions) -> Self {
        Self::Options(value)
    }
}

impl From<LinkedEditingRangeRegistrationOptions> for LinkedEditingRangeProvider {
    fn from(value: LinkedEditingRangeRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SemanticTokensProvider {
    /// `SemanticTokensOptions`.
    Options(SemanticTokensOptions),
    /// `SemanticTokensRegistrationOptions`.
    RegistrationOptions(SemanticTokensRegistrationOptions),
}

impl From<SemanticTokensOptions> for SemanticTokensProvider {
    fn from(value: SemanticTokensOptions) -> Self {
        Self::Options(value)
    }
}

impl From<SemanticTokensRegistrationOptions> for SemanticTokensProvider {
    fn from(value: SemanticTokensRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
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

impl From<bool> for MonikerProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<MonikerOptions> for MonikerProvider {
    fn from(value: MonikerOptions) -> Self {
        Self::Options(value)
    }
}

impl From<MonikerRegistrationOptions> for MonikerProvider {
    fn from(value: MonikerRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
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

impl From<bool> for TypeHierarchyProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<TypeHierarchyOptions> for TypeHierarchyProvider {
    fn from(value: TypeHierarchyOptions) -> Self {
        Self::Options(value)
    }
}

impl From<TypeHierarchyRegistrationOptions> for TypeHierarchyProvider {
    fn from(value: TypeHierarchyRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
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

impl From<bool> for InlineValueProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<InlineValueOptions> for InlineValueProvider {
    fn from(value: InlineValueOptions) -> Self {
        Self::Options(value)
    }
}

impl From<InlineValueRegistrationOptions> for InlineValueProvider {
    fn from(value: InlineValueRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
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

impl From<bool> for InlayHintProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<InlayHintOptions> for InlayHintProvider {
    fn from(value: InlayHintOptions) -> Self {
        Self::Options(value)
    }
}

impl From<InlayHintRegistrationOptions> for InlayHintProvider {
    fn from(value: InlayHintRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiagnosticProvider {
    /// `DiagnosticOptions`.
    Options(DiagnosticOptions),
    /// `DiagnosticRegistrationOptions`.
    RegistrationOptions(DiagnosticRegistrationOptions),
}

impl From<DiagnosticOptions> for DiagnosticProvider {
    fn from(value: DiagnosticOptions) -> Self {
        Self::Options(value)
    }
}

impl From<DiagnosticRegistrationOptions> for DiagnosticProvider {
    fn from(value: DiagnosticRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InlineCompletionProvider {
    Bool(bool),
    /// `InlineCompletionOptions`.
    Options(InlineCompletionOptions),
}

#[cfg(feature = "proposed")]
impl From<bool> for InlineCompletionProvider {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

#[cfg(feature = "proposed")]
impl From<InlineCompletionOptions> for InlineCompletionProvider {
    fn from(value: InlineCompletionOptions) -> Self {
        Self::Options(value)
    }
}

pub type DiagnosticCode = NumberOrString;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompletionItemDefaultsEditRange {
    Range(Range),
    EditRangeWithInsertReplace(EditRangeWithInsertReplace),
}

impl From<Range> for CompletionItemDefaultsEditRange {
    fn from(value: Range) -> Self {
        Self::Range(value)
    }
}

impl From<EditRangeWithInsertReplace> for CompletionItemDefaultsEditRange {
    fn from(value: EditRangeWithInsertReplace) -> Self {
        Self::EditRangeWithInsertReplace(value)
    }
}

pub type SignatureInformationDocumentation = StringOrMarkupContent;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NotebookDocumentFilterNotebook {
    String(String),
    NotebookDocumentFilter(NotebookDocumentFilter),
}

impl From<String> for NotebookDocumentFilterNotebook {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<NotebookDocumentFilter> for NotebookDocumentFilterNotebook {
    fn from(value: NotebookDocumentFilter) -> Self {
        Self::NotebookDocumentFilter(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextDocumentSyncSave {
    Bool(bool),
    SaveOptions(SaveOptions),
}

impl From<bool> for TextDocumentSyncSave {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<SaveOptions> for TextDocumentSyncSave {
    fn from(value: SaveOptions) -> Self {
        Self::SaveOptions(value)
    }
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

#[cfg(feature = "proposed")]
impl From<TextDocumentContentOptions> for WorkspaceTextDocumentContent {
    fn from(value: TextDocumentContentOptions) -> Self {
        Self::Options(value)
    }
}

#[cfg(feature = "proposed")]
impl From<TextDocumentContentRegistrationOptions> for WorkspaceTextDocumentContent {
    fn from(value: TextDocumentContentRegistrationOptions) -> Self {
        Self::RegistrationOptions(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ParameterInformationLabel {
    String(String),
    Tuple((u32, u32)),
}

impl From<String> for ParameterInformationLabel {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<(u32, u32)> for ParameterInformationLabel {
    fn from(value: (u32, u32)) -> Self {
        Self::Tuple(value)
    }
}

pub type ParameterInformationDocumentation = StringOrMarkupContent;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorkspaceFoldersChangeNotifications {
    String(String),
    Bool(bool),
}

impl From<String> for WorkspaceFoldersChangeNotifications {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<bool> for WorkspaceFoldersChangeNotifications {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RelativePatternBaseUri {
    WorkspaceFolder(WorkspaceFolder),
    Uri(Uri),
}

impl From<WorkspaceFolder> for RelativePatternBaseUri {
    fn from(value: WorkspaceFolder) -> Self {
        Self::WorkspaceFolder(value)
    }
}

impl From<Uri> for RelativePatternBaseUri {
    fn from(value: Uri) -> Self {
        Self::Uri(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientSemanticTokensRequestRange {
    Bool(bool),
    Object(serde_json::Value),
}

impl From<bool> for ClientSemanticTokensRequestRange {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<serde_json::Value> for ClientSemanticTokensRequestRange {
    fn from(value: serde_json::Value) -> Self {
        Self::Object(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientSemanticTokensRequestFull {
    Bool(bool),
    /// `ClientSemanticTokensRequestFullDelta`.
    Delta(ClientSemanticTokensRequestFullDelta),
}

impl From<bool> for ClientSemanticTokensRequestFull {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<ClientSemanticTokensRequestFullDelta> for ClientSemanticTokensRequestFull {
    fn from(value: ClientSemanticTokensRequestFullDelta) -> Self {
        Self::Delta(value)
    }
}