// DO NOT EDIT THIS GENERATED FILE.

use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum Request {
    #[serde(rename = "textDocument/implementation")]
    /// A request to resolve the implementation locations of a symbol at a given text
    /// document position. The request's parameter is of type {@link TextDocumentPositionParams}
    /// the response is of type {@link Definition} or a Thenable that resolves to such.
    Implementation { id: u32, params: ImplementationParams },

    #[serde(rename = "textDocument/typeDefinition")]
    /// A request to resolve the type definition locations of a symbol at a given text
    /// document position. The request's parameter is of type {@link TextDocumentPositionParams}
    /// the response is of type {@link Definition} or a Thenable that resolves to such.
    TypeDefinition { id: u32, params: TypeDefinitionParams },

    #[serde(rename = "workspace/workspaceFolders")]
    /// The `workspace/workspaceFolders` is sent from the server to the client to fetch the open workspace folders.
    WorkspaceFolders { id: u32 },

    #[serde(rename = "workspace/configuration")]
    /// The 'workspace/configuration' request is sent from the server to the client to fetch a certain
    /// configuration setting.
    ///
    /// This pull model replaces the old push model were the client signaled configuration change via an
    /// event. If the server still needs to react to configuration changes (since the server caches the
    /// result of `workspace/configuration` requests) the server should register for an empty configuration
    /// change event and empty the cache if such an event is received.
    Configuration { id: u32, params: ConfigurationParams },

    #[serde(rename = "textDocument/documentColor")]
    /// A request to list all color symbols found in a given text document. The request's
    /// parameter is of type {@link DocumentColorParams} the
    /// response is of type {@link ColorInformation ColorInformation[]} or a Thenable
    /// that resolves to such.
    DocumentColor { id: u32, params: DocumentColorParams },

    #[serde(rename = "textDocument/colorPresentation")]
    /// A request to list all presentation for a color. The request's
    /// parameter is of type {@link ColorPresentationParams} the
    /// response is of type {@link ColorInformation ColorInformation[]} or a Thenable
    /// that resolves to such.
    ColorPresentation { id: u32, params: ColorPresentationParams },

    #[serde(rename = "textDocument/foldingRange")]
    /// A request to provide folding ranges in a document. The request's
    /// parameter is of type {@link FoldingRangeParams}, the
    /// response is of type {@link FoldingRangeList} or a Thenable
    /// that resolves to such.
    FoldingRange { id: u32, params: FoldingRangeParams },

    #[cfg(feature = "proposed")]
    #[serde(rename = "workspace/foldingRange/refresh")]
    /// @since 3.18.0
    /// @proposed
    FoldingRangeRefresh { id: u32 },

    #[serde(rename = "textDocument/declaration")]
    /// A request to resolve the type definition locations of a symbol at a given text
    /// document position. The request's parameter is of type {@link TextDocumentPositionParams}
    /// the response is of type {@link Declaration} or a typed array of {@link DeclarationLink}
    /// or a Thenable that resolves to such.
    Declaration { id: u32, params: DeclarationParams },

    #[serde(rename = "textDocument/selectionRange")]
    /// A request to provide selection ranges in a document. The request's
    /// parameter is of type {@link SelectionRangeParams}, the
    /// response is of type {@link SelectionRange SelectionRange[]} or a Thenable
    /// that resolves to such.
    SelectionRange { id: u32, params: SelectionRangeParams },

    #[serde(rename = "window/workDoneProgress/create")]
    /// The `window/workDoneProgress/create` request is sent from the server to the client to initiate progress
    /// reporting from the server.
    WorkDoneProgressCreate { id: u32, params: WorkDoneProgressCreateParams },

    #[serde(rename = "textDocument/prepareCallHierarchy")]
    /// A request to result a `CallHierarchyItem` in a document at a given position.
    /// Can be used as an input to an incoming or outgoing call hierarchy.
    ///
    /// @since 3.16.0
    CallHierarchyPrepare { id: u32, params: CallHierarchyPrepareParams },

    #[serde(rename = "callHierarchy/incomingCalls")]
    /// A request to resolve the incoming calls for a given `CallHierarchyItem`.
    ///
    /// @since 3.16.0
    CallHierarchyIncomingCalls { id: u32, params: CallHierarchyIncomingCallsParams },

    #[serde(rename = "callHierarchy/outgoingCalls")]
    /// A request to resolve the outgoing calls for a given `CallHierarchyItem`.
    ///
    /// @since 3.16.0
    CallHierarchyOutgoingCalls { id: u32, params: CallHierarchyOutgoingCallsParams },

    #[serde(rename = "textDocument/semanticTokens/full")]
    /// @since 3.16.0
    SemanticTokens { id: u32, params: SemanticTokensParams },

    #[serde(rename = "textDocument/semanticTokens/full/delta")]
    /// @since 3.16.0
    SemanticTokensDelta { id: u32, params: SemanticTokensDeltaParams },

    #[serde(rename = "textDocument/semanticTokens/range")]
    /// @since 3.16.0
    SemanticTokensRange { id: u32, params: SemanticTokensRangeParams },

    #[serde(rename = "workspace/semanticTokens/refresh")]
    /// @since 3.16.0
    SemanticTokensRefresh { id: u32 },

    #[serde(rename = "window/showDocument")]
    /// A request to show a document. This request might open an
    /// external program depending on the value of the URI to open.
    /// For example a request to open `https://code.visualstudio.com/`
    /// will very likely open the URI in a WEB browser.
    ///
    /// @since 3.16.0
    ShowDocument { id: u32, params: ShowDocumentParams },

    #[serde(rename = "textDocument/linkedEditingRange")]
    /// A request to provide ranges that can be edited together.
    ///
    /// @since 3.16.0
    LinkedEditingRange { id: u32, params: LinkedEditingRangeParams },

    #[serde(rename = "workspace/willCreateFiles")]
    /// The will create files request is sent from the client to the server before files are actually
    /// created as long as the creation is triggered from within the client.
    ///
    /// The request can return a `WorkspaceEdit` which will be applied to workspace before the
    /// files are created. Hence the `WorkspaceEdit` can not manipulate the content of the file
    /// to be created.
    ///
    /// @since 3.16.0
    WillCreateFiles { id: u32, params: CreateFilesParams },

    #[serde(rename = "workspace/willRenameFiles")]
    /// The will rename files request is sent from the client to the server before files are actually
    /// renamed as long as the rename is triggered from within the client.
    ///
    /// @since 3.16.0
    WillRenameFiles { id: u32, params: RenameFilesParams },

    #[serde(rename = "workspace/willDeleteFiles")]
    /// The did delete files notification is sent from the client to the server when
    /// files were deleted from within the client.
    ///
    /// @since 3.16.0
    WillDeleteFiles { id: u32, params: DeleteFilesParams },

    #[serde(rename = "textDocument/moniker")]
    /// A request to get the moniker of a symbol at a given text document position.
    /// The request parameter is of type {@link TextDocumentPositionParams}.
    /// The response is of type {@link Moniker Moniker[]} or `null`.
    Moniker { id: u32, params: MonikerParams },

    #[serde(rename = "textDocument/prepareTypeHierarchy")]
    /// A request to result a `TypeHierarchyItem` in a document at a given position.
    /// Can be used as an input to a subtypes or supertypes type hierarchy.
    ///
    /// @since 3.17.0
    TypeHierarchyPrepare { id: u32, params: TypeHierarchyPrepareParams },

    #[serde(rename = "typeHierarchy/supertypes")]
    /// A request to resolve the supertypes for a given `TypeHierarchyItem`.
    ///
    /// @since 3.17.0
    TypeHierarchySupertypes { id: u32, params: TypeHierarchySupertypesParams },

    #[serde(rename = "typeHierarchy/subtypes")]
    /// A request to resolve the subtypes for a given `TypeHierarchyItem`.
    ///
    /// @since 3.17.0
    TypeHierarchySubtypes { id: u32, params: TypeHierarchySubtypesParams },

    #[serde(rename = "textDocument/inlineValue")]
    /// A request to provide inline values in a document. The request's parameter is of
    /// type {@link InlineValueParams}, the response is of type
    /// {@link InlineValue InlineValue[]} or a Thenable that resolves to such.
    ///
    /// @since 3.17.0
    InlineValue { id: u32, params: InlineValueParams },

    #[serde(rename = "workspace/inlineValue/refresh")]
    /// @since 3.17.0
    InlineValueRefresh { id: u32 },

    #[serde(rename = "textDocument/inlayHint")]
    /// A request to provide inlay hints in a document. The request's parameter is of
    /// type {@link InlayHintsParams}, the response is of type
    /// {@link InlayHint InlayHint[]} or a Thenable that resolves to such.
    ///
    /// @since 3.17.0
    InlayHint { id: u32, params: InlayHintParams },

    #[serde(rename = "inlayHint/resolve")]
    /// A request to resolve additional properties for an inlay hint.
    /// The request's parameter is of type {@link InlayHint}, the response is
    /// of type {@link InlayHint} or a Thenable that resolves to such.
    ///
    /// @since 3.17.0
    InlayHintResolve { id: u32, params: InlayHint },

    #[serde(rename = "workspace/inlayHint/refresh")]
    /// @since 3.17.0
    InlayHintRefresh { id: u32 },

    #[serde(rename = "textDocument/diagnostic")]
    /// The document diagnostic request definition.
    ///
    /// @since 3.17.0
    DocumentDiagnostic { id: u32, params: DocumentDiagnosticParams },

    #[serde(rename = "workspace/diagnostic")]
    /// The workspace diagnostic request definition.
    ///
    /// @since 3.17.0
    WorkspaceDiagnostic { id: u32, params: WorkspaceDiagnosticParams },

    #[serde(rename = "workspace/diagnostic/refresh")]
    /// The diagnostic refresh request definition.
    ///
    /// @since 3.17.0
    DiagnosticRefresh { id: u32 },

    #[cfg(feature = "proposed")]
    #[serde(rename = "textDocument/inlineCompletion")]
    /// A request to provide inline completions in a document. The request's parameter is of
    /// type {@link InlineCompletionParams}, the response is of type
    /// {@link InlineCompletion InlineCompletion[]} or a Thenable that resolves to such.
    ///
    /// @since 3.18.0
    /// @proposed
    InlineCompletion { id: u32, params: InlineCompletionParams },

    #[cfg(feature = "proposed")]
    #[serde(rename = "workspace/textDocumentContent")]
    /// The `workspace/textDocumentContent` request is sent from the client to the
    /// server to request the content of a text document.
    ///
    /// @since 3.18.0
    /// @proposed
    TextDocumentContent { id: u32, params: TextDocumentContentParams },

    #[cfg(feature = "proposed")]
    #[serde(rename = "workspace/textDocumentContent/refresh")]
    /// The `workspace/textDocumentContent` request is sent from the server to the client to refresh
    /// the content of a specific text document.
    ///
    /// @since 3.18.0
    /// @proposed
    TextDocumentContentRefresh { id: u32, params: TextDocumentContentRefreshParams },

    #[serde(rename = "client/registerCapability")]
    /// The `client/registerCapability` request is sent from the server to the client to register a new capability
    /// handler on the client side.
    Registration { id: u32, params: RegistrationParams },

    #[serde(rename = "client/unregisterCapability")]
    /// The `client/unregisterCapability` request is sent from the server to the client to unregister a previously registered capability
    /// handler on the client side.
    Unregistration { id: u32, params: UnregistrationParams },

    #[serde(rename = "initialize")]
    /// The initialize request is sent from the client to the server.
    /// It is sent once as the request after starting up the server.
    /// The requests parameter is of type {@link InitializeParams}
    /// the response if of type {@link InitializeResult} of a Thenable that
    /// resolves to such.
    Initialize { id: u32, params: InitializeParams },

    #[serde(rename = "shutdown")]
    /// A shutdown request is sent from the client to the server.
    /// It is sent once when the client decides to shutdown the
    /// server. The only notification that is sent after a shutdown request
    /// is the exit event.
    Shutdown { id: u32 },

    #[serde(rename = "window/showMessageRequest")]
    /// The show message request is sent from the server to the client to show a message
    /// and a set of options actions to the user.
    ShowMessage { id: u32, params: ShowMessageRequestParams },

    #[serde(rename = "textDocument/willSaveWaitUntil")]
    /// A document will save request is sent from the client to the server before
    /// the document is actually saved. The request can return an array of TextEdits
    /// which will be applied to the text document before it is saved. Please note that
    /// clients might drop results if computing the text edits took too long or if a
    /// server constantly fails on this request. This is done to keep the save fast and
    /// reliable.
    WillSaveTextDocumentWaitUntil { id: u32, params: WillSaveTextDocumentParams },

    #[serde(rename = "textDocument/completion")]
    /// Request to request completion at a given text document position. The request's
    /// parameter is of type {@link TextDocumentPosition} the response
    /// is of type {@link CompletionItem CompletionItem[]} or {@link CompletionList}
    /// or a Thenable that resolves to such.
    ///
    /// The request can delay the computation of the {@link CompletionItem.detail `detail`}
    /// and {@link CompletionItem.documentation `documentation`} properties to the `completionItem/resolve`
    /// request. However, properties that are needed for the initial sorting and filtering, like `sortText`,
    /// `filterText`, `insertText`, and `textEdit`, must not be changed during resolve.
    Completion { id: u32, params: CompletionParams },

    #[serde(rename = "completionItem/resolve")]
    /// Request to resolve additional information for a given completion item.The request's
    /// parameter is of type {@link CompletionItem} the response
    /// is of type {@link CompletionItem} or a Thenable that resolves to such.
    CompletionResolve { id: u32, params: CompletionItem },

    #[serde(rename = "textDocument/hover")]
    /// Request to request hover information at a given text document position. The request's
    /// parameter is of type {@link TextDocumentPosition} the response is of
    /// type {@link Hover} or a Thenable that resolves to such.
    Hover { id: u32, params: HoverParams },

    #[serde(rename = "textDocument/signatureHelp")]
    SignatureHelp { id: u32, params: SignatureHelpParams },

    #[serde(rename = "textDocument/definition")]
    /// A request to resolve the definition location of a symbol at a given text
    /// document position. The request's parameter is of type {@link TextDocumentPosition}
    /// the response is of either type {@link Definition} or a typed array of
    /// {@link DefinitionLink} or a Thenable that resolves to such.
    Definition { id: u32, params: DefinitionParams },

    #[serde(rename = "textDocument/references")]
    /// A request to resolve project-wide references for the symbol denoted
    /// by the given text document position. The request's parameter is of
    /// type {@link ReferenceParams} the response is of type
    /// {@link Location Location[]} or a Thenable that resolves to such.
    References { id: u32, params: ReferenceParams },

    #[serde(rename = "textDocument/documentHighlight")]
    /// Request to resolve a {@link DocumentHighlight} for a given
    /// text document position. The request's parameter is of type {@link TextDocumentPosition}
    /// the request response is an array of type {@link DocumentHighlight}
    /// or a Thenable that resolves to such.
    DocumentHighlight { id: u32, params: DocumentHighlightParams },

    #[serde(rename = "textDocument/documentSymbol")]
    /// A request to list all symbols found in a given text document. The request's
    /// parameter is of type {@link TextDocumentIdentifier} the
    /// response is of type {@link SymbolInformation SymbolInformation[]} or a Thenable
    /// that resolves to such.
    DocumentSymbol { id: u32, params: DocumentSymbolParams },

    #[serde(rename = "textDocument/codeAction")]
    /// A request to provide commands for the given text document and range.
    CodeAction { id: u32, params: CodeActionParams },

    #[serde(rename = "codeAction/resolve")]
    /// Request to resolve additional information for a given code action.The request's
    /// parameter is of type {@link CodeAction} the response
    /// is of type {@link CodeAction} or a Thenable that resolves to such.
    CodeActionResolve { id: u32, params: CodeAction },

    #[serde(rename = "workspace/symbol")]
    /// A request to list project-wide symbols matching the query string given
    /// by the {@link WorkspaceSymbolParams}. The response is
    /// of type {@link SymbolInformation SymbolInformation[]} or a Thenable that
    /// resolves to such.
    ///
    /// @since 3.17.0 - support for WorkspaceSymbol in the returned data. Clients
    ///  need to advertise support for WorkspaceSymbols via the client capability
    ///  `workspace.symbol.resolveSupport`.
    WorkspaceSymbol { id: u32, params: WorkspaceSymbolParams },

    #[serde(rename = "workspaceSymbol/resolve")]
    /// A request to resolve the range inside the workspace
    /// symbol's location.
    ///
    /// @since 3.17.0
    WorkspaceSymbolResolve { id: u32, params: WorkspaceSymbol },

    #[serde(rename = "textDocument/codeLens")]
    /// A request to provide code lens for the given text document.
    CodeLens { id: u32, params: CodeLensParams },

    #[serde(rename = "codeLens/resolve")]
    /// A request to resolve a command for a given code lens.
    CodeLensResolve { id: u32, params: CodeLens },

    #[serde(rename = "workspace/codeLens/refresh")]
    /// A request to refresh all code actions
    ///
    /// @since 3.16.0
    CodeLensRefresh { id: u32 },

    #[serde(rename = "textDocument/documentLink")]
    /// A request to provide document links
    DocumentLink { id: u32, params: DocumentLinkParams },

    #[serde(rename = "documentLink/resolve")]
    /// Request to resolve additional information for a given document link. The request's
    /// parameter is of type {@link DocumentLink} the response
    /// is of type {@link DocumentLink} or a Thenable that resolves to such.
    DocumentLinkResolve { id: u32, params: DocumentLink },

    #[serde(rename = "textDocument/formatting")]
    /// A request to format a whole document.
    DocumentFormatting { id: u32, params: DocumentFormattingParams },

    #[serde(rename = "textDocument/rangeFormatting")]
    /// A request to format a range in a document.
    DocumentRangeFormatting { id: u32, params: DocumentRangeFormattingParams },

    #[cfg(feature = "proposed")]
    #[serde(rename = "textDocument/rangesFormatting")]
    /// A request to format ranges in a document.
    ///
    /// @since 3.18.0
    /// @proposed
    DocumentRangesFormatting { id: u32, params: DocumentRangesFormattingParams },

    #[serde(rename = "textDocument/onTypeFormatting")]
    /// A request to format a document on type.
    DocumentOnTypeFormatting { id: u32, params: DocumentOnTypeFormattingParams },

    #[serde(rename = "textDocument/rename")]
    /// A request to rename a symbol.
    Rename { id: u32, params: RenameParams },

    #[serde(rename = "textDocument/prepareRename")]
    /// A request to test and perform the setup necessary for a rename.
    ///
    /// @since 3.16 - support for default behavior
    PrepareRename { id: u32, params: PrepareRenameParams },

    #[serde(rename = "workspace/executeCommand")]
    /// A request send from the client to the server to execute a command. The request might return
    /// a workspace edit which the client will apply to the workspace.
    ExecuteCommand { id: u32, params: ExecuteCommandParams },

    #[serde(rename = "workspace/applyEdit")]
    /// A request sent from the server to the client to modified certain resources.
    ApplyWorkspaceEdit { id: u32, params: ApplyWorkspaceEditParams },
}
