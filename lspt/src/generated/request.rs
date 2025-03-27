// DO NOT EDIT THIS GENERATED FILE.

use crate::{Union2, Union3, Union4};
use serde::Serialize;
use super::*;

pub trait Request {
    const METHOD: &'static str;
    type Params: serde::de::DeserializeOwned + Serialize + Send + Sync + 'static;
    type Result: serde::de::DeserializeOwned + Serialize + Send + Sync + 'static;
}

pub enum ImplementationRequest {}
impl Request for ImplementationRequest {
    const METHOD: &'static str = "textDocument/implementation";
    type Params = ImplementationParams;
    type Result = Option<Union3<Definition, Vec<DefinitionLink>, Vec<Location>>>;
}

pub enum TypeDefinitionRequest {}
impl Request for TypeDefinitionRequest {
    const METHOD: &'static str = "textDocument/typeDefinition";
    type Params = TypeDefinitionParams;
    type Result = Option<Union3<Definition, Vec<DefinitionLink>, Vec<Location>>>;
}

pub enum WorkspaceFoldersRequest {}
impl Request for WorkspaceFoldersRequest {
    const METHOD: &'static str = "workspace/workspaceFolders";
    type Params = ();
    type Result = Option<Vec<WorkspaceFolder>>;
}

pub enum ConfigurationRequest {}
impl Request for ConfigurationRequest {
    const METHOD: &'static str = "workspace/configuration";
    type Params = ConfigurationParams;
    type Result = Vec<serde_json::Value>;
}

pub enum DocumentColorRequest {}
impl Request for DocumentColorRequest {
    const METHOD: &'static str = "textDocument/documentColor";
    type Params = DocumentColorParams;
    type Result = Vec<ColorInformation>;
}

pub enum ColorPresentationRequest {}
impl Request for ColorPresentationRequest {
    const METHOD: &'static str = "textDocument/colorPresentation";
    type Params = ColorPresentationParams;
    type Result = Vec<ColorPresentation>;
}

pub enum FoldingRangeRequest {}
impl Request for FoldingRangeRequest {
    const METHOD: &'static str = "textDocument/foldingRange";
    type Params = FoldingRangeParams;
    type Result = Option<Vec<FoldingRange>>;
}

#[cfg(feature = "proposed")]
pub enum FoldingRangeRefreshRequest {}
#[cfg(feature = "proposed")]
impl Request for FoldingRangeRefreshRequest {
    const METHOD: &'static str = "workspace/foldingRange/refresh";
    type Params = ();
    type Result = ();
}

pub enum DeclarationRequest {}
impl Request for DeclarationRequest {
    const METHOD: &'static str = "textDocument/declaration";
    type Params = DeclarationParams;
    type Result = Option<Union3<Declaration, Vec<DeclarationLink>, Vec<Location>>>;
}

pub enum SelectionRangeRequest {}
impl Request for SelectionRangeRequest {
    const METHOD: &'static str = "textDocument/selectionRange";
    type Params = SelectionRangeParams;
    type Result = Option<Vec<SelectionRange>>;
}

pub enum WorkDoneProgressCreateRequest {}
impl Request for WorkDoneProgressCreateRequest {
    const METHOD: &'static str = "window/workDoneProgress/create";
    type Params = WorkDoneProgressCreateParams;
    type Result = ();
}

pub enum CallHierarchyPrepareRequest {}
impl Request for CallHierarchyPrepareRequest {
    const METHOD: &'static str = "textDocument/prepareCallHierarchy";
    type Params = CallHierarchyPrepareParams;
    type Result = Option<Vec<CallHierarchyItem>>;
}

pub enum CallHierarchyIncomingCallsRequest {}
impl Request for CallHierarchyIncomingCallsRequest {
    const METHOD: &'static str = "callHierarchy/incomingCalls";
    type Params = CallHierarchyIncomingCallsParams;
    type Result = Option<Vec<CallHierarchyIncomingCall>>;
}

pub enum CallHierarchyOutgoingCallsRequest {}
impl Request for CallHierarchyOutgoingCallsRequest {
    const METHOD: &'static str = "callHierarchy/outgoingCalls";
    type Params = CallHierarchyOutgoingCallsParams;
    type Result = Option<Vec<CallHierarchyOutgoingCall>>;
}

pub enum SemanticTokensRequest {}
impl Request for SemanticTokensRequest {
    const METHOD: &'static str = "textDocument/semanticTokens/full";
    type Params = SemanticTokensParams;
    type Result = Option<Union2<SemanticTokens, SemanticTokensPartialResult>>;
}

pub enum SemanticTokensDeltaRequest {}
impl Request for SemanticTokensDeltaRequest {
    const METHOD: &'static str = "textDocument/semanticTokens/full/delta";
    type Params = SemanticTokensDeltaParams;
    type Result = Option<Union4<SemanticTokens, SemanticTokensDelta, SemanticTokensPartialResult, SemanticTokensDeltaPartialResult>>;
}

pub enum SemanticTokensRangeRequest {}
impl Request for SemanticTokensRangeRequest {
    const METHOD: &'static str = "textDocument/semanticTokens/range";
    type Params = SemanticTokensRangeParams;
    type Result = Option<Union2<SemanticTokens, SemanticTokensPartialResult>>;
}

pub enum SemanticTokensRefreshRequest {}
impl Request for SemanticTokensRefreshRequest {
    const METHOD: &'static str = "workspace/semanticTokens/refresh";
    type Params = ();
    type Result = ();
}

pub enum ShowDocumentRequest {}
impl Request for ShowDocumentRequest {
    const METHOD: &'static str = "window/showDocument";
    type Params = ShowDocumentParams;
    type Result = ShowDocumentResult;
}

pub enum LinkedEditingRangeRequest {}
impl Request for LinkedEditingRangeRequest {
    const METHOD: &'static str = "textDocument/linkedEditingRange";
    type Params = LinkedEditingRangeParams;
    type Result = Option<LinkedEditingRanges>;
}

pub enum WillCreateFilesRequest {}
impl Request for WillCreateFilesRequest {
    const METHOD: &'static str = "workspace/willCreateFiles";
    type Params = CreateFilesParams;
    type Result = Option<WorkspaceEdit>;
}

pub enum WillRenameFilesRequest {}
impl Request for WillRenameFilesRequest {
    const METHOD: &'static str = "workspace/willRenameFiles";
    type Params = RenameFilesParams;
    type Result = Option<WorkspaceEdit>;
}

pub enum WillDeleteFilesRequest {}
impl Request for WillDeleteFilesRequest {
    const METHOD: &'static str = "workspace/willDeleteFiles";
    type Params = DeleteFilesParams;
    type Result = Option<WorkspaceEdit>;
}

pub enum MonikerRequest {}
impl Request for MonikerRequest {
    const METHOD: &'static str = "textDocument/moniker";
    type Params = MonikerParams;
    type Result = Option<Vec<Moniker>>;
}

pub enum TypeHierarchyPrepareRequest {}
impl Request for TypeHierarchyPrepareRequest {
    const METHOD: &'static str = "textDocument/prepareTypeHierarchy";
    type Params = TypeHierarchyPrepareParams;
    type Result = Option<Vec<TypeHierarchyItem>>;
}

pub enum TypeHierarchySupertypesRequest {}
impl Request for TypeHierarchySupertypesRequest {
    const METHOD: &'static str = "typeHierarchy/supertypes";
    type Params = TypeHierarchySupertypesParams;
    type Result = Option<Vec<TypeHierarchyItem>>;
}

pub enum TypeHierarchySubtypesRequest {}
impl Request for TypeHierarchySubtypesRequest {
    const METHOD: &'static str = "typeHierarchy/subtypes";
    type Params = TypeHierarchySubtypesParams;
    type Result = Option<Vec<TypeHierarchyItem>>;
}

pub enum InlineValueRequest {}
impl Request for InlineValueRequest {
    const METHOD: &'static str = "textDocument/inlineValue";
    type Params = InlineValueParams;
    type Result = Option<Vec<InlineValue>>;
}

pub enum InlineValueRefreshRequest {}
impl Request for InlineValueRefreshRequest {
    const METHOD: &'static str = "workspace/inlineValue/refresh";
    type Params = ();
    type Result = ();
}

pub enum InlayHintRequest {}
impl Request for InlayHintRequest {
    const METHOD: &'static str = "textDocument/inlayHint";
    type Params = InlayHintParams;
    type Result = Option<Vec<InlayHint>>;
}

pub enum InlayHintResolveRequest {}
impl Request for InlayHintResolveRequest {
    const METHOD: &'static str = "inlayHint/resolve";
    type Params = InlayHint;
    type Result = InlayHint;
}

pub enum InlayHintRefreshRequest {}
impl Request for InlayHintRefreshRequest {
    const METHOD: &'static str = "workspace/inlayHint/refresh";
    type Params = ();
    type Result = ();
}

pub enum DocumentDiagnosticRequest {}
impl Request for DocumentDiagnosticRequest {
    const METHOD: &'static str = "textDocument/diagnostic";
    type Params = DocumentDiagnosticParams;
    type Result = Union2<DocumentDiagnosticReport, DocumentDiagnosticReportPartialResult>;
}

pub enum WorkspaceDiagnosticRequest {}
impl Request for WorkspaceDiagnosticRequest {
    const METHOD: &'static str = "workspace/diagnostic";
    type Params = WorkspaceDiagnosticParams;
    type Result = Union2<WorkspaceDiagnosticReport, WorkspaceDiagnosticReportPartialResult>;
}

pub enum DiagnosticRefreshRequest {}
impl Request for DiagnosticRefreshRequest {
    const METHOD: &'static str = "workspace/diagnostic/refresh";
    type Params = ();
    type Result = ();
}

#[cfg(feature = "proposed")]
pub enum InlineCompletionRequest {}
#[cfg(feature = "proposed")]
impl Request for InlineCompletionRequest {
    const METHOD: &'static str = "textDocument/inlineCompletion";
    type Params = InlineCompletionParams;
    type Result = Option<Union2<InlineCompletionList, Vec<InlineCompletionItem>>>;
}

#[cfg(feature = "proposed")]
pub enum TextDocumentContentRequest {}
#[cfg(feature = "proposed")]
impl Request for TextDocumentContentRequest {
    const METHOD: &'static str = "workspace/textDocumentContent";
    type Params = TextDocumentContentParams;
    type Result = TextDocumentContentResult;
}

#[cfg(feature = "proposed")]
pub enum TextDocumentContentRefreshRequest {}
#[cfg(feature = "proposed")]
impl Request for TextDocumentContentRefreshRequest {
    const METHOD: &'static str = "workspace/textDocumentContent/refresh";
    type Params = TextDocumentContentRefreshParams;
    type Result = ();
}

pub enum RegistrationRequest {}
impl Request for RegistrationRequest {
    const METHOD: &'static str = "client/registerCapability";
    type Params = RegistrationParams;
    type Result = ();
}

pub enum UnregistrationRequest {}
impl Request for UnregistrationRequest {
    const METHOD: &'static str = "client/unregisterCapability";
    type Params = UnregistrationParams;
    type Result = ();
}

pub enum InitializeRequest {}
impl Request for InitializeRequest {
    const METHOD: &'static str = "initialize";
    type Params = InitializeParams;
    type Result = InitializeResult;
}

pub enum ShutdownRequest {}
impl Request for ShutdownRequest {
    const METHOD: &'static str = "shutdown";
    type Params = ();
    type Result = ();
}

pub enum ShowMessageRequest {}
impl Request for ShowMessageRequest {
    const METHOD: &'static str = "window/showMessageRequest";
    type Params = ShowMessageRequestParams;
    type Result = Option<MessageActionItem>;
}

pub enum WillSaveTextDocumentWaitUntilRequest {}
impl Request for WillSaveTextDocumentWaitUntilRequest {
    const METHOD: &'static str = "textDocument/willSaveWaitUntil";
    type Params = WillSaveTextDocumentParams;
    type Result = Option<Vec<TextEdit>>;
}

pub enum CompletionRequest {}
impl Request for CompletionRequest {
    const METHOD: &'static str = "textDocument/completion";
    type Params = CompletionParams;
    type Result = Option<Union2<Vec<CompletionItem>, CompletionList>>;
}

pub enum CompletionResolveRequest {}
impl Request for CompletionResolveRequest {
    const METHOD: &'static str = "completionItem/resolve";
    type Params = CompletionItem;
    type Result = CompletionItem;
}

pub enum HoverRequest {}
impl Request for HoverRequest {
    const METHOD: &'static str = "textDocument/hover";
    type Params = HoverParams;
    type Result = Option<Hover>;
}

pub enum SignatureHelpRequest {}
impl Request for SignatureHelpRequest {
    const METHOD: &'static str = "textDocument/signatureHelp";
    type Params = SignatureHelpParams;
    type Result = Option<SignatureHelp>;
}

pub enum DefinitionRequest {}
impl Request for DefinitionRequest {
    const METHOD: &'static str = "textDocument/definition";
    type Params = DefinitionParams;
    type Result = Option<Union3<Definition, Vec<DefinitionLink>, Vec<Location>>>;
}

pub enum ReferencesRequest {}
impl Request for ReferencesRequest {
    const METHOD: &'static str = "textDocument/references";
    type Params = ReferenceParams;
    type Result = Option<Vec<Location>>;
}

pub enum DocumentHighlightRequest {}
impl Request for DocumentHighlightRequest {
    const METHOD: &'static str = "textDocument/documentHighlight";
    type Params = DocumentHighlightParams;
    type Result = Option<Vec<DocumentHighlight>>;
}

pub enum DocumentSymbolRequest {}
impl Request for DocumentSymbolRequest {
    const METHOD: &'static str = "textDocument/documentSymbol";
    type Params = DocumentSymbolParams;
    type Result = Option<Union2<Vec<SymbolInformation>, Vec<DocumentSymbol>>>;
}

pub enum CodeActionRequest {}
impl Request for CodeActionRequest {
    const METHOD: &'static str = "textDocument/codeAction";
    type Params = CodeActionParams;
    type Result = Option<Vec<Union2<Command, CodeAction>>>;
}

pub enum CodeActionResolveRequest {}
impl Request for CodeActionResolveRequest {
    const METHOD: &'static str = "codeAction/resolve";
    type Params = CodeAction;
    type Result = CodeAction;
}

pub enum WorkspaceSymbolRequest {}
impl Request for WorkspaceSymbolRequest {
    const METHOD: &'static str = "workspace/symbol";
    type Params = WorkspaceSymbolParams;
    type Result = Option<Union2<Vec<SymbolInformation>, Vec<WorkspaceSymbol>>>;
}

pub enum WorkspaceSymbolResolveRequest {}
impl Request for WorkspaceSymbolResolveRequest {
    const METHOD: &'static str = "workspaceSymbol/resolve";
    type Params = WorkspaceSymbol;
    type Result = WorkspaceSymbol;
}

pub enum CodeLensRequest {}
impl Request for CodeLensRequest {
    const METHOD: &'static str = "textDocument/codeLens";
    type Params = CodeLensParams;
    type Result = Option<Vec<CodeLens>>;
}

pub enum CodeLensResolveRequest {}
impl Request for CodeLensResolveRequest {
    const METHOD: &'static str = "codeLens/resolve";
    type Params = CodeLens;
    type Result = CodeLens;
}

pub enum CodeLensRefreshRequest {}
impl Request for CodeLensRefreshRequest {
    const METHOD: &'static str = "workspace/codeLens/refresh";
    type Params = ();
    type Result = ();
}

pub enum DocumentLinkRequest {}
impl Request for DocumentLinkRequest {
    const METHOD: &'static str = "textDocument/documentLink";
    type Params = DocumentLinkParams;
    type Result = Option<Vec<DocumentLink>>;
}

pub enum DocumentLinkResolveRequest {}
impl Request for DocumentLinkResolveRequest {
    const METHOD: &'static str = "documentLink/resolve";
    type Params = DocumentLink;
    type Result = DocumentLink;
}

pub enum DocumentFormattingRequest {}
impl Request for DocumentFormattingRequest {
    const METHOD: &'static str = "textDocument/formatting";
    type Params = DocumentFormattingParams;
    type Result = Option<Vec<TextEdit>>;
}

pub enum DocumentRangeFormattingRequest {}
impl Request for DocumentRangeFormattingRequest {
    const METHOD: &'static str = "textDocument/rangeFormatting";
    type Params = DocumentRangeFormattingParams;
    type Result = Option<Vec<TextEdit>>;
}

#[cfg(feature = "proposed")]
pub enum DocumentRangesFormattingRequest {}
#[cfg(feature = "proposed")]
impl Request for DocumentRangesFormattingRequest {
    const METHOD: &'static str = "textDocument/rangesFormatting";
    type Params = DocumentRangesFormattingParams;
    type Result = Option<Vec<TextEdit>>;
}

pub enum DocumentOnTypeFormattingRequest {}
impl Request for DocumentOnTypeFormattingRequest {
    const METHOD: &'static str = "textDocument/onTypeFormatting";
    type Params = DocumentOnTypeFormattingParams;
    type Result = Option<Vec<TextEdit>>;
}

pub enum RenameRequest {}
impl Request for RenameRequest {
    const METHOD: &'static str = "textDocument/rename";
    type Params = RenameParams;
    type Result = Option<WorkspaceEdit>;
}

pub enum PrepareRenameRequest {}
impl Request for PrepareRenameRequest {
    const METHOD: &'static str = "textDocument/prepareRename";
    type Params = PrepareRenameParams;
    type Result = Option<PrepareRenameResult>;
}

pub enum ExecuteCommandRequest {}
impl Request for ExecuteCommandRequest {
    const METHOD: &'static str = "workspace/executeCommand";
    type Params = ExecuteCommandParams;
    type Result = Option<serde_json::Value>;
}

pub enum ApplyWorkspaceEditRequest {}
impl Request for ApplyWorkspaceEditRequest {
    const METHOD: &'static str = "workspace/applyEdit";
    type Params = ApplyWorkspaceEditParams;
    type Result = ApplyWorkspaceEditResult;
}
