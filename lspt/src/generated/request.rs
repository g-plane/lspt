// DO NOT EDIT THIS GENERATED FILE.

use crate::*;

pub trait Request {
    const METHOD: &'static str;
    type Params: serde::de::DeserializeOwned + Serialize + Send + Sync + 'static;
}

pub enum ImplementationRequest {}
impl Request for ImplementationRequest {
    const METHOD: &'static str = "textDocument/implementation";
    type Params = ImplementationParams;
}

pub enum TypeDefinitionRequest {}
impl Request for TypeDefinitionRequest {
    const METHOD: &'static str = "textDocument/typeDefinition";
    type Params = TypeDefinitionParams;
}

pub enum WorkspaceFoldersRequest {}
impl Request for WorkspaceFoldersRequest {
    const METHOD: &'static str = "workspace/workspaceFolders";
    type Params = ();
}

pub enum ConfigurationRequest {}
impl Request for ConfigurationRequest {
    const METHOD: &'static str = "workspace/configuration";
    type Params = ConfigurationParams;
}

pub enum DocumentColorRequest {}
impl Request for DocumentColorRequest {
    const METHOD: &'static str = "textDocument/documentColor";
    type Params = DocumentColorParams;
}

pub enum ColorPresentationRequest {}
impl Request for ColorPresentationRequest {
    const METHOD: &'static str = "textDocument/colorPresentation";
    type Params = ColorPresentationParams;
}

pub enum FoldingRangeRequest {}
impl Request for FoldingRangeRequest {
    const METHOD: &'static str = "textDocument/foldingRange";
    type Params = FoldingRangeParams;
}

#[cfg(feature = "proposed")]
pub enum FoldingRangeRefreshRequest {}
#[cfg(feature = "proposed")]
impl Request for FoldingRangeRefreshRequest {
    const METHOD: &'static str = "workspace/foldingRange/refresh";
    type Params = ();
}

pub enum DeclarationRequest {}
impl Request for DeclarationRequest {
    const METHOD: &'static str = "textDocument/declaration";
    type Params = DeclarationParams;
}

pub enum SelectionRangeRequest {}
impl Request for SelectionRangeRequest {
    const METHOD: &'static str = "textDocument/selectionRange";
    type Params = SelectionRangeParams;
}

pub enum WorkDoneProgressCreateRequest {}
impl Request for WorkDoneProgressCreateRequest {
    const METHOD: &'static str = "window/workDoneProgress/create";
    type Params = WorkDoneProgressCreateParams;
}

pub enum CallHierarchyPrepareRequest {}
impl Request for CallHierarchyPrepareRequest {
    const METHOD: &'static str = "textDocument/prepareCallHierarchy";
    type Params = CallHierarchyPrepareParams;
}

pub enum CallHierarchyIncomingCallsRequest {}
impl Request for CallHierarchyIncomingCallsRequest {
    const METHOD: &'static str = "callHierarchy/incomingCalls";
    type Params = CallHierarchyIncomingCallsParams;
}

pub enum CallHierarchyOutgoingCallsRequest {}
impl Request for CallHierarchyOutgoingCallsRequest {
    const METHOD: &'static str = "callHierarchy/outgoingCalls";
    type Params = CallHierarchyOutgoingCallsParams;
}

pub enum SemanticTokensRequest {}
impl Request for SemanticTokensRequest {
    const METHOD: &'static str = "textDocument/semanticTokens/full";
    type Params = SemanticTokensParams;
}

pub enum SemanticTokensDeltaRequest {}
impl Request for SemanticTokensDeltaRequest {
    const METHOD: &'static str = "textDocument/semanticTokens/full/delta";
    type Params = SemanticTokensDeltaParams;
}

pub enum SemanticTokensRangeRequest {}
impl Request for SemanticTokensRangeRequest {
    const METHOD: &'static str = "textDocument/semanticTokens/range";
    type Params = SemanticTokensRangeParams;
}

pub enum SemanticTokensRefreshRequest {}
impl Request for SemanticTokensRefreshRequest {
    const METHOD: &'static str = "workspace/semanticTokens/refresh";
    type Params = ();
}

pub enum ShowDocumentRequest {}
impl Request for ShowDocumentRequest {
    const METHOD: &'static str = "window/showDocument";
    type Params = ShowDocumentParams;
}

pub enum LinkedEditingRangeRequest {}
impl Request for LinkedEditingRangeRequest {
    const METHOD: &'static str = "textDocument/linkedEditingRange";
    type Params = LinkedEditingRangeParams;
}

pub enum WillCreateFilesRequest {}
impl Request for WillCreateFilesRequest {
    const METHOD: &'static str = "workspace/willCreateFiles";
    type Params = CreateFilesParams;
}

pub enum WillRenameFilesRequest {}
impl Request for WillRenameFilesRequest {
    const METHOD: &'static str = "workspace/willRenameFiles";
    type Params = RenameFilesParams;
}

pub enum WillDeleteFilesRequest {}
impl Request for WillDeleteFilesRequest {
    const METHOD: &'static str = "workspace/willDeleteFiles";
    type Params = DeleteFilesParams;
}

pub enum MonikerRequest {}
impl Request for MonikerRequest {
    const METHOD: &'static str = "textDocument/moniker";
    type Params = MonikerParams;
}

pub enum TypeHierarchyPrepareRequest {}
impl Request for TypeHierarchyPrepareRequest {
    const METHOD: &'static str = "textDocument/prepareTypeHierarchy";
    type Params = TypeHierarchyPrepareParams;
}

pub enum TypeHierarchySupertypesRequest {}
impl Request for TypeHierarchySupertypesRequest {
    const METHOD: &'static str = "typeHierarchy/supertypes";
    type Params = TypeHierarchySupertypesParams;
}

pub enum TypeHierarchySubtypesRequest {}
impl Request for TypeHierarchySubtypesRequest {
    const METHOD: &'static str = "typeHierarchy/subtypes";
    type Params = TypeHierarchySubtypesParams;
}

pub enum InlineValueRequest {}
impl Request for InlineValueRequest {
    const METHOD: &'static str = "textDocument/inlineValue";
    type Params = InlineValueParams;
}

pub enum InlineValueRefreshRequest {}
impl Request for InlineValueRefreshRequest {
    const METHOD: &'static str = "workspace/inlineValue/refresh";
    type Params = ();
}

pub enum InlayHintRequest {}
impl Request for InlayHintRequest {
    const METHOD: &'static str = "textDocument/inlayHint";
    type Params = InlayHintParams;
}

pub enum InlayHintResolveRequest {}
impl Request for InlayHintResolveRequest {
    const METHOD: &'static str = "inlayHint/resolve";
    type Params = InlayHint;
}

pub enum InlayHintRefreshRequest {}
impl Request for InlayHintRefreshRequest {
    const METHOD: &'static str = "workspace/inlayHint/refresh";
    type Params = ();
}

pub enum DocumentDiagnosticRequest {}
impl Request for DocumentDiagnosticRequest {
    const METHOD: &'static str = "textDocument/diagnostic";
    type Params = DocumentDiagnosticParams;
}

pub enum WorkspaceDiagnosticRequest {}
impl Request for WorkspaceDiagnosticRequest {
    const METHOD: &'static str = "workspace/diagnostic";
    type Params = WorkspaceDiagnosticParams;
}

pub enum DiagnosticRefreshRequest {}
impl Request for DiagnosticRefreshRequest {
    const METHOD: &'static str = "workspace/diagnostic/refresh";
    type Params = ();
}

#[cfg(feature = "proposed")]
pub enum InlineCompletionRequest {}
#[cfg(feature = "proposed")]
impl Request for InlineCompletionRequest {
    const METHOD: &'static str = "textDocument/inlineCompletion";
    type Params = InlineCompletionParams;
}

#[cfg(feature = "proposed")]
pub enum TextDocumentContentRequest {}
#[cfg(feature = "proposed")]
impl Request for TextDocumentContentRequest {
    const METHOD: &'static str = "workspace/textDocumentContent";
    type Params = TextDocumentContentParams;
}

#[cfg(feature = "proposed")]
pub enum TextDocumentContentRefreshRequest {}
#[cfg(feature = "proposed")]
impl Request for TextDocumentContentRefreshRequest {
    const METHOD: &'static str = "workspace/textDocumentContent/refresh";
    type Params = TextDocumentContentRefreshParams;
}

pub enum RegistrationRequest {}
impl Request for RegistrationRequest {
    const METHOD: &'static str = "client/registerCapability";
    type Params = RegistrationParams;
}

pub enum UnregistrationRequest {}
impl Request for UnregistrationRequest {
    const METHOD: &'static str = "client/unregisterCapability";
    type Params = UnregistrationParams;
}

pub enum InitializeRequest {}
impl Request for InitializeRequest {
    const METHOD: &'static str = "initialize";
    type Params = InitializeParams;
}

pub enum ShutdownRequest {}
impl Request for ShutdownRequest {
    const METHOD: &'static str = "shutdown";
    type Params = ();
}

pub enum ShowMessageRequest {}
impl Request for ShowMessageRequest {
    const METHOD: &'static str = "window/showMessageRequest";
    type Params = ShowMessageRequestParams;
}

pub enum WillSaveTextDocumentWaitUntilRequest {}
impl Request for WillSaveTextDocumentWaitUntilRequest {
    const METHOD: &'static str = "textDocument/willSaveWaitUntil";
    type Params = WillSaveTextDocumentParams;
}

pub enum CompletionRequest {}
impl Request for CompletionRequest {
    const METHOD: &'static str = "textDocument/completion";
    type Params = CompletionParams;
}

pub enum CompletionResolveRequest {}
impl Request for CompletionResolveRequest {
    const METHOD: &'static str = "completionItem/resolve";
    type Params = CompletionItem;
}

pub enum HoverRequest {}
impl Request for HoverRequest {
    const METHOD: &'static str = "textDocument/hover";
    type Params = HoverParams;
}

pub enum SignatureHelpRequest {}
impl Request for SignatureHelpRequest {
    const METHOD: &'static str = "textDocument/signatureHelp";
    type Params = SignatureHelpParams;
}

pub enum DefinitionRequest {}
impl Request for DefinitionRequest {
    const METHOD: &'static str = "textDocument/definition";
    type Params = DefinitionParams;
}

pub enum ReferencesRequest {}
impl Request for ReferencesRequest {
    const METHOD: &'static str = "textDocument/references";
    type Params = ReferenceParams;
}

pub enum DocumentHighlightRequest {}
impl Request for DocumentHighlightRequest {
    const METHOD: &'static str = "textDocument/documentHighlight";
    type Params = DocumentHighlightParams;
}

pub enum DocumentSymbolRequest {}
impl Request for DocumentSymbolRequest {
    const METHOD: &'static str = "textDocument/documentSymbol";
    type Params = DocumentSymbolParams;
}

pub enum CodeActionRequest {}
impl Request for CodeActionRequest {
    const METHOD: &'static str = "textDocument/codeAction";
    type Params = CodeActionParams;
}

pub enum CodeActionResolveRequest {}
impl Request for CodeActionResolveRequest {
    const METHOD: &'static str = "codeAction/resolve";
    type Params = CodeAction;
}

pub enum WorkspaceSymbolRequest {}
impl Request for WorkspaceSymbolRequest {
    const METHOD: &'static str = "workspace/symbol";
    type Params = WorkspaceSymbolParams;
}

pub enum WorkspaceSymbolResolveRequest {}
impl Request for WorkspaceSymbolResolveRequest {
    const METHOD: &'static str = "workspaceSymbol/resolve";
    type Params = WorkspaceSymbol;
}

pub enum CodeLensRequest {}
impl Request for CodeLensRequest {
    const METHOD: &'static str = "textDocument/codeLens";
    type Params = CodeLensParams;
}

pub enum CodeLensResolveRequest {}
impl Request for CodeLensResolveRequest {
    const METHOD: &'static str = "codeLens/resolve";
    type Params = CodeLens;
}

pub enum CodeLensRefreshRequest {}
impl Request for CodeLensRefreshRequest {
    const METHOD: &'static str = "workspace/codeLens/refresh";
    type Params = ();
}

pub enum DocumentLinkRequest {}
impl Request for DocumentLinkRequest {
    const METHOD: &'static str = "textDocument/documentLink";
    type Params = DocumentLinkParams;
}

pub enum DocumentLinkResolveRequest {}
impl Request for DocumentLinkResolveRequest {
    const METHOD: &'static str = "documentLink/resolve";
    type Params = DocumentLink;
}

pub enum DocumentFormattingRequest {}
impl Request for DocumentFormattingRequest {
    const METHOD: &'static str = "textDocument/formatting";
    type Params = DocumentFormattingParams;
}

pub enum DocumentRangeFormattingRequest {}
impl Request for DocumentRangeFormattingRequest {
    const METHOD: &'static str = "textDocument/rangeFormatting";
    type Params = DocumentRangeFormattingParams;
}

#[cfg(feature = "proposed")]
pub enum DocumentRangesFormattingRequest {}
#[cfg(feature = "proposed")]
impl Request for DocumentRangesFormattingRequest {
    const METHOD: &'static str = "textDocument/rangesFormatting";
    type Params = DocumentRangesFormattingParams;
}

pub enum DocumentOnTypeFormattingRequest {}
impl Request for DocumentOnTypeFormattingRequest {
    const METHOD: &'static str = "textDocument/onTypeFormatting";
    type Params = DocumentOnTypeFormattingParams;
}

pub enum RenameRequest {}
impl Request for RenameRequest {
    const METHOD: &'static str = "textDocument/rename";
    type Params = RenameParams;
}

pub enum PrepareRenameRequest {}
impl Request for PrepareRenameRequest {
    const METHOD: &'static str = "textDocument/prepareRename";
    type Params = PrepareRenameParams;
}

pub enum ExecuteCommandRequest {}
impl Request for ExecuteCommandRequest {
    const METHOD: &'static str = "workspace/executeCommand";
    type Params = ExecuteCommandParams;
}

pub enum ApplyWorkspaceEditRequest {}
impl Request for ApplyWorkspaceEditRequest {
    const METHOD: &'static str = "workspace/applyEdit";
    type Params = ApplyWorkspaceEditParams;
}
