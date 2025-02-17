// DO NOT EDIT THIS GENERATED FILE.

use super::{HashMap, Union2, Union3, Union4, Uri};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "method", content = "params")]
pub enum Notification {
    #[serde(rename = "workspace/didChangeWorkspaceFolders")]
    /// The `workspace/didChangeWorkspaceFolders` notification is sent from the client to the server when the workspace
    /// folder configuration changes.
    DidChangeWorkspaceFolders(DidChangeWorkspaceFoldersParams),

    #[serde(rename = "window/workDoneProgress/cancel")]
    /// The `window/workDoneProgress/cancel` notification is sent from  the client to the server to cancel a progress
    /// initiated on the server side.
    WorkDoneProgressCancel(WorkDoneProgressCancelParams),

    #[serde(rename = "workspace/didCreateFiles")]
    /// The did create files notification is sent from the client to the server when
    /// files were created from within the client.
    ///
    /// @since 3.16.0
    DidCreateFiles(CreateFilesParams),

    #[serde(rename = "workspace/didRenameFiles")]
    /// The did rename files notification is sent from the client to the server when
    /// files were renamed from within the client.
    ///
    /// @since 3.16.0
    DidRenameFiles(RenameFilesParams),

    #[serde(rename = "workspace/didDeleteFiles")]
    /// The will delete files request is sent from the client to the server before files are actually
    /// deleted as long as the deletion is triggered from within the client.
    ///
    /// @since 3.16.0
    DidDeleteFiles(DeleteFilesParams),

    #[serde(rename = "notebookDocument/didOpen")]
    /// A notification sent when a notebook opens.
    ///
    /// @since 3.17.0
    DidOpenNotebookDocument(DidOpenNotebookDocumentParams),

    #[serde(rename = "notebookDocument/didChange")]
    DidChangeNotebookDocument(DidChangeNotebookDocumentParams),

    #[serde(rename = "notebookDocument/didSave")]
    /// A notification sent when a notebook document is saved.
    ///
    /// @since 3.17.0
    DidSaveNotebookDocument(DidSaveNotebookDocumentParams),

    #[serde(rename = "notebookDocument/didClose")]
    /// A notification sent when a notebook closes.
    ///
    /// @since 3.17.0
    DidCloseNotebookDocument(DidCloseNotebookDocumentParams),

    #[serde(rename = "initialized")]
    /// The initialized notification is sent from the client to the
    /// server after the client is fully initialized and the server
    /// is allowed to send requests from the server to the client.
    Initialized(InitializedParams),

    #[serde(rename = "exit")]
    /// The exit event is sent from the client to the server to
    /// ask the server to exit its process.
    Exit,

    #[serde(rename = "workspace/didChangeConfiguration")]
    /// The configuration change notification is sent from the client to the server
    /// when the client's configuration has changed. The notification contains
    /// the changed configuration as defined by the language client.
    DidChangeConfiguration(DidChangeConfigurationParams),

    #[serde(rename = "window/showMessage")]
    /// The show message notification is sent from a server to a client to ask
    /// the client to display a particular message in the user interface.
    ShowMessage(ShowMessageParams),

    #[serde(rename = "window/logMessage")]
    /// The log message notification is sent from the server to the client to ask
    /// the client to log a particular message.
    LogMessage(LogMessageParams),

    #[serde(rename = "telemetry/event")]
    /// The telemetry event notification is sent from the server to the client to ask
    /// the client to log telemetry data.
    TelemetryEvent(serde_json::Value),

    #[serde(rename = "textDocument/didOpen")]
    /// The document open notification is sent from the client to the server to signal
    /// newly opened text documents. The document's truth is now managed by the client
    /// and the server must not try to read the document's truth using the document's
    /// uri. Open in this sense means it is managed by the client. It doesn't necessarily
    /// mean that its content is presented in an editor. An open notification must not
    /// be sent more than once without a corresponding close notification send before.
    /// This means open and close notification must be balanced and the max open count
    /// is one.
    DidOpenTextDocument(DidOpenTextDocumentParams),

    #[serde(rename = "textDocument/didChange")]
    /// The document change notification is sent from the client to the server to signal
    /// changes to a text document.
    DidChangeTextDocument(DidChangeTextDocumentParams),

    #[serde(rename = "textDocument/didClose")]
    /// The document close notification is sent from the client to the server when
    /// the document got closed in the client. The document's truth now exists where
    /// the document's uri points to (e.g. if the document's uri is a file uri the
    /// truth now exists on disk). As with the open notification the close notification
    /// is about managing the document's content. Receiving a close notification
    /// doesn't mean that the document was open in an editor before. A close
    /// notification requires a previous open notification to be sent.
    DidCloseTextDocument(DidCloseTextDocumentParams),

    #[serde(rename = "textDocument/didSave")]
    /// The document save notification is sent from the client to the server when
    /// the document got saved in the client.
    DidSaveTextDocument(DidSaveTextDocumentParams),

    #[serde(rename = "textDocument/willSave")]
    /// A document will save notification is sent from the client to the server before
    /// the document is actually saved.
    WillSaveTextDocument(WillSaveTextDocumentParams),

    #[serde(rename = "workspace/didChangeWatchedFiles")]
    /// The watched files notification is sent from the client to the server when
    /// the client detects changes to file watched by the language client.
    DidChangeWatchedFiles(DidChangeWatchedFilesParams),

    #[serde(rename = "textDocument/publishDiagnostics")]
    /// Diagnostics notification are sent from the server to the client to signal
    /// results of validation runs.
    PublishDiagnostics(PublishDiagnosticsParams),

    #[serde(rename = "$/setTrace")]
    SetTrace(SetTraceParams),

    #[serde(rename = "$/logTrace")]
    LogTrace(LogTraceParams),

    #[serde(rename = "$/cancelRequest")]
    Cancel(CancelParams),

    #[serde(rename = "$/progress")]
    Progress(ProgressParams),
}

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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A set of predefined token types. This set is not fixed
/// an clients can specify additional token types via the
/// corresponding client capabilities.
///
/// @since 3.16.0
pub enum SemanticTokenTypes {
    #[serde(rename = "namespace")]
    Namespace,

    #[serde(rename = "type")]
    /// Represents a generic type. Acts as a fallback for types which can't be mapped to
    /// a specific type like class or enum.
    Type,

    #[serde(rename = "class")]
    Class,

    #[serde(rename = "enum")]
    Enum,

    #[serde(rename = "interface")]
    Interface,

    #[serde(rename = "struct")]
    Struct,

    #[serde(rename = "typeParameter")]
    TypeParameter,

    #[serde(rename = "parameter")]
    Parameter,

    #[serde(rename = "variable")]
    Variable,

    #[serde(rename = "property")]
    Property,

    #[serde(rename = "enumMember")]
    EnumMember,

    #[serde(rename = "event")]
    Event,

    #[serde(rename = "function")]
    Function,

    #[serde(rename = "method")]
    Method,

    #[serde(rename = "macro")]
    Macro,

    #[serde(rename = "keyword")]
    Keyword,

    #[serde(rename = "modifier")]
    Modifier,

    #[serde(rename = "comment")]
    Comment,

    #[serde(rename = "string")]
    String,

    #[serde(rename = "number")]
    Number,

    #[serde(rename = "regexp")]
    Regexp,

    #[serde(rename = "operator")]
    Operator,

    #[serde(rename = "decorator")]
    /// @since 3.17.0
    Decorator,

    #[serde(rename = "label")]
    /// @since 3.18.0
    Label,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A set of predefined token modifiers. This set is not fixed
/// an clients can specify additional token types via the
/// corresponding client capabilities.
///
/// @since 3.16.0
pub enum SemanticTokenModifiers {
    #[serde(rename = "declaration")]
    Declaration,

    #[serde(rename = "definition")]
    Definition,

    #[serde(rename = "readonly")]
    Readonly,

    #[serde(rename = "static")]
    Static,

    #[serde(rename = "deprecated")]
    Deprecated,

    #[serde(rename = "abstract")]
    Abstract,

    #[serde(rename = "async")]
    Async,

    #[serde(rename = "modification")]
    Modification,

    #[serde(rename = "documentation")]
    Documentation,

    #[serde(rename = "defaultLibrary")]
    DefaultLibrary,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// The document diagnostic report kinds.
///
/// @since 3.17.0
pub enum DocumentDiagnosticReportKind {
    #[serde(rename = "full")]
    /// A diagnostic report with a full
    /// set of problems.
    Full,

    #[serde(rename = "unchanged")]
    /// A report indicating that the last
    /// returned report is still accurate.
    Unchanged,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Predefined error codes.
pub enum ErrorCodes {
    ParseError = -32700,

    InvalidRequest = -32600,

    MethodNotFound = -32601,

    InvalidParams = -32602,

    InternalError = -32603,

    /// Error code indicating that a server received a notification or
    /// request before the server has received the `initialize` request.
    ServerNotInitialized = -32002,

    UnknownErrorCode = -32001,
}
impl Serialize for ErrorCodes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ErrorCodes::ParseError => serializer.serialize_i32(-32700),
            ErrorCodes::InvalidRequest => serializer.serialize_i32(-32600),
            ErrorCodes::MethodNotFound => serializer.serialize_i32(-32601),
            ErrorCodes::InvalidParams => serializer.serialize_i32(-32602),
            ErrorCodes::InternalError => serializer.serialize_i32(-32603),
            ErrorCodes::ServerNotInitialized => serializer.serialize_i32(-32002),
            ErrorCodes::UnknownErrorCode => serializer.serialize_i32(-32001),
        }
    }
}
impl<'de> Deserialize<'de> for ErrorCodes {
    fn deserialize<D>(deserializer: D) -> Result<ErrorCodes, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            -32700 => Ok(ErrorCodes::ParseError),
            -32600 => Ok(ErrorCodes::InvalidRequest),
            -32601 => Ok(ErrorCodes::MethodNotFound),
            -32602 => Ok(ErrorCodes::InvalidParams),
            -32603 => Ok(ErrorCodes::InternalError),
            -32002 => Ok(ErrorCodes::ServerNotInitialized),
            -32001 => Ok(ErrorCodes::UnknownErrorCode),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of -32700, -32600, -32601, -32602, -32603, -32002, -32001",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LSPErrorCodes {
    /// A request failed but it was syntactically correct, e.g the
    /// method name was known and the parameters were valid. The error
    /// message should contain human readable information about why
    /// the request failed.
    ///
    /// @since 3.17.0
    RequestFailed = -32803,

    /// The server cancelled the request. This error code should
    /// only be used for requests that explicitly support being
    /// server cancellable.
    ///
    /// @since 3.17.0
    ServerCancelled = -32802,

    /// The server detected that the content of a document got
    /// modified outside normal conditions. A server should
    /// NOT send this error code if it detects a content change
    /// in it unprocessed messages. The result even computed
    /// on an older state might still be useful for the client.
    ///
    /// If a client decides that a result is not of any use anymore
    /// the client should cancel the request.
    ContentModified = -32801,

    /// The client has canceled a request and a server has detected
    /// the cancel.
    RequestCancelled = -32800,
}
impl Serialize for LSPErrorCodes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LSPErrorCodes::RequestFailed => serializer.serialize_i32(-32803),
            LSPErrorCodes::ServerCancelled => serializer.serialize_i32(-32802),
            LSPErrorCodes::ContentModified => serializer.serialize_i32(-32801),
            LSPErrorCodes::RequestCancelled => serializer.serialize_i32(-32800),
        }
    }
}
impl<'de> Deserialize<'de> for LSPErrorCodes {
    fn deserialize<D>(deserializer: D) -> Result<LSPErrorCodes, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            -32803 => Ok(LSPErrorCodes::RequestFailed),
            -32802 => Ok(LSPErrorCodes::ServerCancelled),
            -32801 => Ok(LSPErrorCodes::ContentModified),
            -32800 => Ok(LSPErrorCodes::RequestCancelled),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of -32803, -32802, -32801, -32800",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A set of predefined range kinds.
pub enum FoldingRangeKind {
    #[serde(rename = "comment")]
    /// Folding range for a comment
    Comment,

    #[serde(rename = "imports")]
    /// Folding range for an import or include
    Imports,

    #[serde(rename = "region")]
    /// Folding range for a region (e.g. `#region`)
    Region,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A symbol kind.
pub enum SymbolKind {
    File = 1,

    Module = 2,

    Namespace = 3,

    Package = 4,

    Class = 5,

    Method = 6,

    Property = 7,

    Field = 8,

    Constructor = 9,

    Enum = 10,

    Interface = 11,

    Function = 12,

    Variable = 13,

    Constant = 14,

    String = 15,

    Number = 16,

    Boolean = 17,

    Array = 18,

    Object = 19,

    Key = 20,

    Null = 21,

    EnumMember = 22,

    Struct = 23,

    Event = 24,

    Operator = 25,

    TypeParameter = 26,
}
impl Serialize for SymbolKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SymbolKind::File => serializer.serialize_u32(1),
            SymbolKind::Module => serializer.serialize_u32(2),
            SymbolKind::Namespace => serializer.serialize_u32(3),
            SymbolKind::Package => serializer.serialize_u32(4),
            SymbolKind::Class => serializer.serialize_u32(5),
            SymbolKind::Method => serializer.serialize_u32(6),
            SymbolKind::Property => serializer.serialize_u32(7),
            SymbolKind::Field => serializer.serialize_u32(8),
            SymbolKind::Constructor => serializer.serialize_u32(9),
            SymbolKind::Enum => serializer.serialize_u32(10),
            SymbolKind::Interface => serializer.serialize_u32(11),
            SymbolKind::Function => serializer.serialize_u32(12),
            SymbolKind::Variable => serializer.serialize_u32(13),
            SymbolKind::Constant => serializer.serialize_u32(14),
            SymbolKind::String => serializer.serialize_u32(15),
            SymbolKind::Number => serializer.serialize_u32(16),
            SymbolKind::Boolean => serializer.serialize_u32(17),
            SymbolKind::Array => serializer.serialize_u32(18),
            SymbolKind::Object => serializer.serialize_u32(19),
            SymbolKind::Key => serializer.serialize_u32(20),
            SymbolKind::Null => serializer.serialize_u32(21),
            SymbolKind::EnumMember => serializer.serialize_u32(22),
            SymbolKind::Struct => serializer.serialize_u32(23),
            SymbolKind::Event => serializer.serialize_u32(24),
            SymbolKind::Operator => serializer.serialize_u32(25),
            SymbolKind::TypeParameter => serializer.serialize_u32(26),
        }
    }
}
impl<'de> Deserialize<'de> for SymbolKind {
    fn deserialize<D>(deserializer: D) -> Result<SymbolKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(SymbolKind::File),
            2 => Ok(SymbolKind::Module),
            3 => Ok(SymbolKind::Namespace),
            4 => Ok(SymbolKind::Package),
            5 => Ok(SymbolKind::Class),
            6 => Ok(SymbolKind::Method),
            7 => Ok(SymbolKind::Property),
            8 => Ok(SymbolKind::Field),
            9 => Ok(SymbolKind::Constructor),
            10 => Ok(SymbolKind::Enum),
            11 => Ok(SymbolKind::Interface),
            12 => Ok(SymbolKind::Function),
            13 => Ok(SymbolKind::Variable),
            14 => Ok(SymbolKind::Constant),
            15 => Ok(SymbolKind::String),
            16 => Ok(SymbolKind::Number),
            17 => Ok(SymbolKind::Boolean),
            18 => Ok(SymbolKind::Array),
            19 => Ok(SymbolKind::Object),
            20 => Ok(SymbolKind::Key),
            21 => Ok(SymbolKind::Null),
            22 => Ok(SymbolKind::EnumMember),
            23 => Ok(SymbolKind::Struct),
            24 => Ok(SymbolKind::Event),
            25 => Ok(SymbolKind::Operator),
            26 => Ok(SymbolKind::TypeParameter),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Symbol tags are extra annotations that tweak the rendering of a symbol.
///
/// @since 3.16
pub enum SymbolTag {
    /// Render a symbol as obsolete, usually using a strike-out.
    Deprecated = 1,
}
impl Serialize for SymbolTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SymbolTag::Deprecated => serializer.serialize_u32(1),
        }
    }
}
impl<'de> Deserialize<'de> for SymbolTag {
    fn deserialize<D>(deserializer: D) -> Result<SymbolTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(SymbolTag::Deprecated),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Moniker uniqueness level to define scope of the moniker.
///
/// @since 3.16.0
pub enum UniquenessLevel {
    #[serde(rename = "document")]
    /// The moniker is only unique inside a document
    Document,

    #[serde(rename = "project")]
    /// The moniker is unique inside a project for which a dump got created
    Project,

    #[serde(rename = "group")]
    /// The moniker is unique inside the group to which a project belongs
    Group,

    #[serde(rename = "scheme")]
    /// The moniker is unique inside the moniker scheme.
    Scheme,

    #[serde(rename = "global")]
    /// The moniker is globally unique
    Global,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// The moniker kind.
///
/// @since 3.16.0
pub enum MonikerKind {
    #[serde(rename = "import")]
    /// The moniker represent a symbol that is imported into a project
    Import,

    #[serde(rename = "export")]
    /// The moniker represents a symbol that is exported from a project
    Export,

    #[serde(rename = "local")]
    /// The moniker represents a symbol that is local to a project (e.g. a local
    /// variable of a function, a class not visible outside the project, ...)
    Local,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Inlay hint kinds.
///
/// @since 3.17.0
pub enum InlayHintKind {
    /// An inlay hint that for a type annotation.
    Type = 1,

    /// An inlay hint that is for a parameter.
    Parameter = 2,
}
impl Serialize for InlayHintKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InlayHintKind::Type => serializer.serialize_u32(1),
            InlayHintKind::Parameter => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for InlayHintKind {
    fn deserialize<D>(deserializer: D) -> Result<InlayHintKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(InlayHintKind::Type),
            2 => Ok(InlayHintKind::Parameter),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The message type
pub enum MessageType {
    /// An error message.
    Error = 1,

    /// A warning message.
    Warning = 2,

    /// An information message.
    Info = 3,

    /// A log message.
    Log = 4,

    #[cfg(feature = "proposed")]
    /// A debug message.
    ///
    /// @since 3.18.0
    /// @proposed
    Debug = 5,
}
impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MessageType::Error => serializer.serialize_u32(1),
            MessageType::Warning => serializer.serialize_u32(2),
            MessageType::Info => serializer.serialize_u32(3),
            MessageType::Log => serializer.serialize_u32(4),
            #[cfg(feature = "proposed")]
            MessageType::Debug => serializer.serialize_u32(5),
        }
    }
}
impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<MessageType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(MessageType::Error),
            2 => Ok(MessageType::Warning),
            3 => Ok(MessageType::Info),
            4 => Ok(MessageType::Log),
            #[cfg(feature = "proposed")]
            5 => Ok(MessageType::Debug),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3, 4, 5",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Defines how the host (editor) should sync
/// document changes to the language server.
pub enum TextDocumentSyncKind {
    /// Documents should not be synced at all.
    None = 0,

    /// Documents are synced by always sending the full content
    /// of the document.
    Full = 1,

    /// Documents are synced by sending the full content on open.
    /// After that only incremental updates to the document are
    /// send.
    Incremental = 2,
}
impl Serialize for TextDocumentSyncKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TextDocumentSyncKind::None => serializer.serialize_u32(0),
            TextDocumentSyncKind::Full => serializer.serialize_u32(1),
            TextDocumentSyncKind::Incremental => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for TextDocumentSyncKind {
    fn deserialize<D>(deserializer: D) -> Result<TextDocumentSyncKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(TextDocumentSyncKind::None),
            1 => Ok(TextDocumentSyncKind::Full),
            2 => Ok(TextDocumentSyncKind::Incremental),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 0, 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Represents reasons why a text document is saved.
pub enum TextDocumentSaveReason {
    /// Manually triggered, e.g. by the user pressing save, by starting debugging,
    /// or by an API call.
    Manual = 1,

    /// Automatic after a delay.
    AfterDelay = 2,

    /// When the editor lost focus.
    FocusOut = 3,
}
impl Serialize for TextDocumentSaveReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TextDocumentSaveReason::Manual => serializer.serialize_u32(1),
            TextDocumentSaveReason::AfterDelay => serializer.serialize_u32(2),
            TextDocumentSaveReason::FocusOut => serializer.serialize_u32(3),
        }
    }
}
impl<'de> Deserialize<'de> for TextDocumentSaveReason {
    fn deserialize<D>(deserializer: D) -> Result<TextDocumentSaveReason, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(TextDocumentSaveReason::Manual),
            2 => Ok(TextDocumentSaveReason::AfterDelay),
            3 => Ok(TextDocumentSaveReason::FocusOut),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The kind of a completion entry.
pub enum CompletionItemKind {
    Text = 1,

    Method = 2,

    Function = 3,

    Constructor = 4,

    Field = 5,

    Variable = 6,

    Class = 7,

    Interface = 8,

    Module = 9,

    Property = 10,

    Unit = 11,

    Value = 12,

    Enum = 13,

    Keyword = 14,

    Snippet = 15,

    Color = 16,

    File = 17,

    Reference = 18,

    Folder = 19,

    EnumMember = 20,

    Constant = 21,

    Struct = 22,

    Event = 23,

    Operator = 24,

    TypeParameter = 25,
}
impl Serialize for CompletionItemKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CompletionItemKind::Text => serializer.serialize_u32(1),
            CompletionItemKind::Method => serializer.serialize_u32(2),
            CompletionItemKind::Function => serializer.serialize_u32(3),
            CompletionItemKind::Constructor => serializer.serialize_u32(4),
            CompletionItemKind::Field => serializer.serialize_u32(5),
            CompletionItemKind::Variable => serializer.serialize_u32(6),
            CompletionItemKind::Class => serializer.serialize_u32(7),
            CompletionItemKind::Interface => serializer.serialize_u32(8),
            CompletionItemKind::Module => serializer.serialize_u32(9),
            CompletionItemKind::Property => serializer.serialize_u32(10),
            CompletionItemKind::Unit => serializer.serialize_u32(11),
            CompletionItemKind::Value => serializer.serialize_u32(12),
            CompletionItemKind::Enum => serializer.serialize_u32(13),
            CompletionItemKind::Keyword => serializer.serialize_u32(14),
            CompletionItemKind::Snippet => serializer.serialize_u32(15),
            CompletionItemKind::Color => serializer.serialize_u32(16),
            CompletionItemKind::File => serializer.serialize_u32(17),
            CompletionItemKind::Reference => serializer.serialize_u32(18),
            CompletionItemKind::Folder => serializer.serialize_u32(19),
            CompletionItemKind::EnumMember => serializer.serialize_u32(20),
            CompletionItemKind::Constant => serializer.serialize_u32(21),
            CompletionItemKind::Struct => serializer.serialize_u32(22),
            CompletionItemKind::Event => serializer.serialize_u32(23),
            CompletionItemKind::Operator => serializer.serialize_u32(24),
            CompletionItemKind::TypeParameter => serializer.serialize_u32(25),
        }
    }
}
impl<'de> Deserialize<'de> for CompletionItemKind {
    fn deserialize<D>(deserializer: D) -> Result<CompletionItemKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CompletionItemKind::Text),
            2 => Ok(CompletionItemKind::Method),
            3 => Ok(CompletionItemKind::Function),
            4 => Ok(CompletionItemKind::Constructor),
            5 => Ok(CompletionItemKind::Field),
            6 => Ok(CompletionItemKind::Variable),
            7 => Ok(CompletionItemKind::Class),
            8 => Ok(CompletionItemKind::Interface),
            9 => Ok(CompletionItemKind::Module),
            10 => Ok(CompletionItemKind::Property),
            11 => Ok(CompletionItemKind::Unit),
            12 => Ok(CompletionItemKind::Value),
            13 => Ok(CompletionItemKind::Enum),
            14 => Ok(CompletionItemKind::Keyword),
            15 => Ok(CompletionItemKind::Snippet),
            16 => Ok(CompletionItemKind::Color),
            17 => Ok(CompletionItemKind::File),
            18 => Ok(CompletionItemKind::Reference),
            19 => Ok(CompletionItemKind::Folder),
            20 => Ok(CompletionItemKind::EnumMember),
            21 => Ok(CompletionItemKind::Constant),
            22 => Ok(CompletionItemKind::Struct),
            23 => Ok(CompletionItemKind::Event),
            24 => Ok(CompletionItemKind::Operator),
            25 => Ok(CompletionItemKind::TypeParameter),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Completion item tags are extra annotations that tweak the rendering of a completion
/// item.
///
/// @since 3.15.0
pub enum CompletionItemTag {
    /// Render a completion as obsolete, usually using a strike-out.
    Deprecated = 1,
}
impl Serialize for CompletionItemTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CompletionItemTag::Deprecated => serializer.serialize_u32(1),
        }
    }
}
impl<'de> Deserialize<'de> for CompletionItemTag {
    fn deserialize<D>(deserializer: D) -> Result<CompletionItemTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CompletionItemTag::Deprecated),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Defines whether the insert text in a completion item should be interpreted as
/// plain text or a snippet.
pub enum InsertTextFormat {
    /// The primary text to be inserted is treated as a plain string.
    PlainText = 1,

    /// The primary text to be inserted is treated as a snippet.
    ///
    /// A snippet can define tab stops and placeholders with `$1`, `$2`
    /// and `${3:foo}`. `$0` defines the final tab stop, it defaults to
    /// the end of the snippet. Placeholders with equal identifiers are linked,
    /// that is typing in one will update others too.
    ///
    /// See also: https://microsoft.github.io/language-server-protocol/specifications/specification-current/#snippet_syntax
    Snippet = 2,
}
impl Serialize for InsertTextFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InsertTextFormat::PlainText => serializer.serialize_u32(1),
            InsertTextFormat::Snippet => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for InsertTextFormat {
    fn deserialize<D>(deserializer: D) -> Result<InsertTextFormat, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(InsertTextFormat::PlainText),
            2 => Ok(InsertTextFormat::Snippet),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// How whitespace and indentation is handled during completion
/// item insertion.
///
/// @since 3.16.0
pub enum InsertTextMode {
    /// The insertion or replace strings is taken as it is. If the
    /// value is multi line the lines below the cursor will be
    /// inserted using the indentation defined in the string value.
    /// The client will not apply any kind of adjustments to the
    /// string.
    AsIs = 1,

    /// The editor adjusts leading whitespace of new lines so that
    /// they match the indentation up to the cursor of the line for
    /// which the item is accepted.
    ///
    /// Consider a line like this: <2tabs><cursor><3tabs>foo. Accepting a
    /// multi line completion item is indented using 2 tabs and all
    /// following lines inserted will be indented using 2 tabs as well.
    AdjustIndentation = 2,
}
impl Serialize for InsertTextMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InsertTextMode::AsIs => serializer.serialize_u32(1),
            InsertTextMode::AdjustIndentation => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for InsertTextMode {
    fn deserialize<D>(deserializer: D) -> Result<InsertTextMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(InsertTextMode::AsIs),
            2 => Ok(InsertTextMode::AdjustIndentation),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A document highlight kind.
pub enum DocumentHighlightKind {
    /// A textual occurrence.
    Text = 1,

    /// Read-access of a symbol, like reading a variable.
    Read = 2,

    /// Write-access of a symbol, like writing to a variable.
    Write = 3,
}
impl Serialize for DocumentHighlightKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DocumentHighlightKind::Text => serializer.serialize_u32(1),
            DocumentHighlightKind::Read => serializer.serialize_u32(2),
            DocumentHighlightKind::Write => serializer.serialize_u32(3),
        }
    }
}
impl<'de> Deserialize<'de> for DocumentHighlightKind {
    fn deserialize<D>(deserializer: D) -> Result<DocumentHighlightKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(DocumentHighlightKind::Text),
            2 => Ok(DocumentHighlightKind::Read),
            3 => Ok(DocumentHighlightKind::Write),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A set of predefined code action kinds
pub enum CodeActionKind {
    #[serde(rename = "")]
    /// Empty kind.
    Empty,

    #[serde(rename = "quickfix")]
    /// Base kind for quickfix actions: 'quickfix'
    QuickFix,

    #[serde(rename = "refactor")]
    /// Base kind for refactoring actions: 'refactor'
    Refactor,

    #[serde(rename = "refactor.extract")]
    /// Base kind for refactoring extraction actions: 'refactor.extract'
    ///
    /// Example extract actions:
    ///
    /// - Extract method
    /// - Extract function
    /// - Extract variable
    /// - Extract interface from class
    /// - ...
    RefactorExtract,

    #[serde(rename = "refactor.inline")]
    /// Base kind for refactoring inline actions: 'refactor.inline'
    ///
    /// Example inline actions:
    ///
    /// - Inline function
    /// - Inline variable
    /// - Inline constant
    /// - ...
    RefactorInline,

    #[cfg(feature = "proposed")]
    #[serde(rename = "refactor.move")]
    /// Base kind for refactoring move actions: `refactor.move`
    ///
    /// Example move actions:
    ///
    /// - Move a function to a new file
    /// - Move a property between classes
    /// - Move method to base class
    /// - ...
    ///
    /// @since 3.18.0
    /// @proposed
    RefactorMove,

    #[serde(rename = "refactor.rewrite")]
    /// Base kind for refactoring rewrite actions: 'refactor.rewrite'
    ///
    /// Example rewrite actions:
    ///
    /// - Convert JavaScript function to class
    /// - Add or remove parameter
    /// - Encapsulate field
    /// - Make method static
    /// - Move method to base class
    /// - ...
    RefactorRewrite,

    #[serde(rename = "source")]
    /// Base kind for source actions: `source`
    ///
    /// Source code actions apply to the entire file.
    Source,

    #[serde(rename = "source.organizeImports")]
    /// Base kind for an organize imports source action: `source.organizeImports`
    SourceOrganizeImports,

    #[serde(rename = "source.fixAll")]
    /// Base kind for auto-fix source actions: `source.fixAll`.
    ///
    /// Fix all actions automatically fix errors that have a clear fix that do not require user input.
    /// They should not suppress errors or perform unsafe fixes such as generating new types or classes.
    ///
    /// @since 3.15.0
    SourceFixAll,

    #[serde(rename = "notebook")]
    /// Base kind for all code actions applying to the entire notebook's scope. CodeActionKinds using
    /// this should always begin with `notebook.`
    ///
    /// @since 3.18.0
    Notebook,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Code action tags are extra annotations that tweak the behavior of a code action.
///
/// @since 3.18.0 - proposed
pub enum CodeActionTag {
    /// Marks the code action as LLM-generated.
    LlmGenerated = 1,
}
impl Serialize for CodeActionTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CodeActionTag::LlmGenerated => serializer.serialize_u32(1),
        }
    }
}
impl<'de> Deserialize<'de> for CodeActionTag {
    fn deserialize<D>(deserializer: D) -> Result<CodeActionTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CodeActionTag::LlmGenerated),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceValue {
    #[serde(rename = "off")]
    /// Turn tracing off.
    Off,

    #[serde(rename = "messages")]
    /// Trace messages only.
    Messages,

    #[serde(rename = "verbose")]
    /// Verbose message tracing.
    Verbose,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Describes the content type that a client supports in various
/// result literals like `Hover`, `ParameterInfo` or `CompletionItem`.
///
/// Please note that `MarkupKinds` must not start with a `$`. This kinds
/// are reserved for internal usage.
pub enum MarkupKind {
    #[serde(rename = "plaintext")]
    /// Plain text is supported as a content format
    PlainText,

    #[serde(rename = "markdown")]
    /// Markdown is supported as a content format
    Markdown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Predefined Language kinds
/// @since 3.18.0
/// @proposed
pub enum LanguageKind {
    #[serde(rename = "abap")]
    Abap,

    #[serde(rename = "bat")]
    WindowsBat,

    #[serde(rename = "bibtex")]
    BibTeX,

    #[serde(rename = "clojure")]
    Clojure,

    #[serde(rename = "coffeescript")]
    Coffeescript,

    #[serde(rename = "c")]
    C,

    #[serde(rename = "cpp")]
    Cpp,

    #[serde(rename = "csharp")]
    CSharp,

    #[serde(rename = "css")]
    Css,

    #[cfg(feature = "proposed")]
    #[serde(rename = "d")]
    /// @since 3.18.0
    /// @proposed
    D,

    #[cfg(feature = "proposed")]
    #[serde(rename = "pascal")]
    /// @since 3.18.0
    /// @proposed
    Delphi,

    #[serde(rename = "diff")]
    Diff,

    #[serde(rename = "dart")]
    Dart,

    #[serde(rename = "dockerfile")]
    Dockerfile,

    #[serde(rename = "elixir")]
    Elixir,

    #[serde(rename = "erlang")]
    Erlang,

    #[serde(rename = "fsharp")]
    FSharp,

    #[serde(rename = "git-commit")]
    GitCommit,

    #[serde(rename = "rebase")]
    GitRebase,

    #[serde(rename = "go")]
    Go,

    #[serde(rename = "groovy")]
    Groovy,

    #[serde(rename = "handlebars")]
    Handlebars,

    #[serde(rename = "haskell")]
    Haskell,

    #[serde(rename = "html")]
    Html,

    #[serde(rename = "ini")]
    Ini,

    #[serde(rename = "java")]
    Java,

    #[serde(rename = "javascript")]
    JavaScript,

    #[serde(rename = "javascriptreact")]
    JavaScriptReact,

    #[serde(rename = "json")]
    Json,

    #[serde(rename = "latex")]
    LaTeX,

    #[serde(rename = "less")]
    Less,

    #[serde(rename = "lua")]
    Lua,

    #[serde(rename = "makefile")]
    Makefile,

    #[serde(rename = "markdown")]
    Markdown,

    #[serde(rename = "objective-c")]
    ObjectiveC,

    #[serde(rename = "objective-cpp")]
    ObjectiveCpp,

    #[cfg(feature = "proposed")]
    #[serde(rename = "pascal")]
    /// @since 3.18.0
    /// @proposed
    Pascal,

    #[serde(rename = "perl")]
    Perl,

    #[serde(rename = "perl6")]
    Perl6,

    #[serde(rename = "php")]
    Php,

    #[serde(rename = "powershell")]
    Powershell,

    #[serde(rename = "jade")]
    Pug,

    #[serde(rename = "python")]
    Python,

    #[serde(rename = "r")]
    R,

    #[serde(rename = "razor")]
    Razor,

    #[serde(rename = "ruby")]
    Ruby,

    #[serde(rename = "rust")]
    Rust,

    #[serde(rename = "scss")]
    Scss,

    #[serde(rename = "sass")]
    Sass,

    #[serde(rename = "scala")]
    Scala,

    #[serde(rename = "shaderlab")]
    ShaderLab,

    #[serde(rename = "shellscript")]
    ShellScript,

    #[serde(rename = "sql")]
    Sql,

    #[serde(rename = "swift")]
    Swift,

    #[serde(rename = "typescript")]
    TypeScript,

    #[serde(rename = "typescriptreact")]
    TypeScriptReact,

    #[serde(rename = "tex")]
    TeX,

    #[serde(rename = "vb")]
    VisualBasic,

    #[serde(rename = "xml")]
    Xml,

    #[serde(rename = "xsl")]
    Xsl,

    #[serde(rename = "yaml")]
    Yaml,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Describes how an {@link InlineCompletionItemProvider inline completion provider} was triggered.
///
/// @since 3.18.0
/// @proposed
pub enum InlineCompletionTriggerKind {
    /// Completion was triggered explicitly by a user gesture.
    Invoked = 1,

    /// Completion was triggered automatically while editing.
    Automatic = 2,
}
impl Serialize for InlineCompletionTriggerKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InlineCompletionTriggerKind::Invoked => serializer.serialize_u32(1),
            InlineCompletionTriggerKind::Automatic => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for InlineCompletionTriggerKind {
    fn deserialize<D>(deserializer: D) -> Result<InlineCompletionTriggerKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(InlineCompletionTriggerKind::Invoked),
            2 => Ok(InlineCompletionTriggerKind::Automatic),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A set of predefined position encoding kinds.
///
/// @since 3.17.0
pub enum PositionEncodingKind {
    #[serde(rename = "utf-8")]
    /// Character offsets count UTF-8 code units (e.g. bytes).
    Utf8,

    #[serde(rename = "utf-16")]
    /// Character offsets count UTF-16 code units.
    ///
    /// This is the default and must always be supported
    /// by servers
    Utf16,

    #[serde(rename = "utf-32")]
    /// Character offsets count UTF-32 code units.
    ///
    /// Implementation note: these are the same as Unicode codepoints,
    /// so this `PositionEncodingKind` may also be used for an
    /// encoding-agnostic representation of character offsets.
    Utf32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The file event type
pub enum FileChangeType {
    /// The file got created.
    Created = 1,

    /// The file got changed.
    Changed = 2,

    /// The file got deleted.
    Deleted = 3,
}
impl Serialize for FileChangeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FileChangeType::Created => serializer.serialize_u32(1),
            FileChangeType::Changed => serializer.serialize_u32(2),
            FileChangeType::Deleted => serializer.serialize_u32(3),
        }
    }
}
impl<'de> Deserialize<'de> for FileChangeType {
    fn deserialize<D>(deserializer: D) -> Result<FileChangeType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(FileChangeType::Created),
            2 => Ok(FileChangeType::Changed),
            3 => Ok(FileChangeType::Deleted),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WatchKind {
    /// Interested in create events.
    Create = 1,

    /// Interested in change events
    Change = 2,

    /// Interested in delete events
    Delete = 4,
}
impl Serialize for WatchKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            WatchKind::Create => serializer.serialize_u32(1),
            WatchKind::Change => serializer.serialize_u32(2),
            WatchKind::Delete => serializer.serialize_u32(4),
        }
    }
}
impl<'de> Deserialize<'de> for WatchKind {
    fn deserialize<D>(deserializer: D) -> Result<WatchKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(WatchKind::Create),
            2 => Ok(WatchKind::Change),
            4 => Ok(WatchKind::Delete),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 4",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The diagnostic's severity.
pub enum DiagnosticSeverity {
    /// Reports an error.
    Error = 1,

    /// Reports a warning.
    Warning = 2,

    /// Reports an information.
    Information = 3,

    /// Reports a hint.
    Hint = 4,
}
impl Serialize for DiagnosticSeverity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DiagnosticSeverity::Error => serializer.serialize_u32(1),
            DiagnosticSeverity::Warning => serializer.serialize_u32(2),
            DiagnosticSeverity::Information => serializer.serialize_u32(3),
            DiagnosticSeverity::Hint => serializer.serialize_u32(4),
        }
    }
}
impl<'de> Deserialize<'de> for DiagnosticSeverity {
    fn deserialize<D>(deserializer: D) -> Result<DiagnosticSeverity, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(DiagnosticSeverity::Error),
            2 => Ok(DiagnosticSeverity::Warning),
            3 => Ok(DiagnosticSeverity::Information),
            4 => Ok(DiagnosticSeverity::Hint),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3, 4",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The diagnostic tags.
///
/// @since 3.15.0
pub enum DiagnosticTag {
    /// Unused or unnecessary code.
    ///
    /// Clients are allowed to render diagnostics with this tag faded out instead of having
    /// an error squiggle.
    Unnecessary = 1,

    /// Deprecated or obsolete code.
    ///
    /// Clients are allowed to rendered diagnostics with this tag strike through.
    Deprecated = 2,
}
impl Serialize for DiagnosticTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DiagnosticTag::Unnecessary => serializer.serialize_u32(1),
            DiagnosticTag::Deprecated => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for DiagnosticTag {
    fn deserialize<D>(deserializer: D) -> Result<DiagnosticTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(DiagnosticTag::Unnecessary),
            2 => Ok(DiagnosticTag::Deprecated),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// How a completion was triggered
pub enum CompletionTriggerKind {
    /// Completion was triggered by typing an identifier (24x7 code
    /// complete), manual invocation (e.g Ctrl+Space) or via API.
    Invoked = 1,

    /// Completion was triggered by a trigger character specified by
    /// the `triggerCharacters` properties of the `CompletionRegistrationOptions`.
    TriggerCharacter = 2,

    /// Completion was re-triggered as current completion list is incomplete
    TriggerForIncompleteCompletions = 3,
}
impl Serialize for CompletionTriggerKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CompletionTriggerKind::Invoked => serializer.serialize_u32(1),
            CompletionTriggerKind::TriggerCharacter => serializer.serialize_u32(2),
            CompletionTriggerKind::TriggerForIncompleteCompletions => serializer.serialize_u32(3),
        }
    }
}
impl<'de> Deserialize<'de> for CompletionTriggerKind {
    fn deserialize<D>(deserializer: D) -> Result<CompletionTriggerKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CompletionTriggerKind::Invoked),
            2 => Ok(CompletionTriggerKind::TriggerCharacter),
            3 => Ok(CompletionTriggerKind::TriggerForIncompleteCompletions),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Defines how values from a set of defaults and an individual item will be
/// merged.
///
/// @since 3.18.0
pub enum ApplyKind {
    /// The value from the individual item (if provided and not `null`) will be
    /// used instead of the default.
    Replace = 1,

    /// The value from the item will be merged with the default.
    ///
    /// The specific rules for mergeing values are defined against each field
    /// that supports merging.
    Merge = 2,
}
impl Serialize for ApplyKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ApplyKind::Replace => serializer.serialize_u32(1),
            ApplyKind::Merge => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for ApplyKind {
    fn deserialize<D>(deserializer: D) -> Result<ApplyKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(ApplyKind::Replace),
            2 => Ok(ApplyKind::Merge),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// How a signature help was triggered.
///
/// @since 3.15.0
pub enum SignatureHelpTriggerKind {
    /// Signature help was invoked manually by the user or by a command.
    Invoked = 1,

    /// Signature help was triggered by a trigger character.
    TriggerCharacter = 2,

    /// Signature help was triggered by the cursor moving or by the document content changing.
    ContentChange = 3,
}
impl Serialize for SignatureHelpTriggerKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SignatureHelpTriggerKind::Invoked => serializer.serialize_u32(1),
            SignatureHelpTriggerKind::TriggerCharacter => serializer.serialize_u32(2),
            SignatureHelpTriggerKind::ContentChange => serializer.serialize_u32(3),
        }
    }
}
impl<'de> Deserialize<'de> for SignatureHelpTriggerKind {
    fn deserialize<D>(deserializer: D) -> Result<SignatureHelpTriggerKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(SignatureHelpTriggerKind::Invoked),
            2 => Ok(SignatureHelpTriggerKind::TriggerCharacter),
            3 => Ok(SignatureHelpTriggerKind::ContentChange),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The reason why code actions were requested.
///
/// @since 3.17.0
pub enum CodeActionTriggerKind {
    /// Code actions were explicitly requested by the user or by an extension.
    Invoked = 1,

    /// Code actions were requested automatically.
    ///
    /// This typically happens when current selection in a file changes, but can
    /// also be triggered when file content changes.
    Automatic = 2,
}
impl Serialize for CodeActionTriggerKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CodeActionTriggerKind::Invoked => serializer.serialize_u32(1),
            CodeActionTriggerKind::Automatic => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for CodeActionTriggerKind {
    fn deserialize<D>(deserializer: D) -> Result<CodeActionTriggerKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CodeActionTriggerKind::Invoked),
            2 => Ok(CodeActionTriggerKind::Automatic),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A pattern kind describing if a glob pattern matches a file a folder or
/// both.
///
/// @since 3.16.0
pub enum FileOperationPatternKind {
    #[serde(rename = "file")]
    /// The pattern matches a file only.
    File,

    #[serde(rename = "folder")]
    /// The pattern matches a folder only.
    Folder,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A notebook cell kind.
///
/// @since 3.17.0
pub enum NotebookCellKind {
    /// A markup-cell is formatted source that is used for display.
    Markup = 1,

    /// A code-cell is source code.
    Code = 2,
}
impl Serialize for NotebookCellKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NotebookCellKind::Markup => serializer.serialize_u32(1),
            NotebookCellKind::Code => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for NotebookCellKind {
    fn deserialize<D>(deserializer: D) -> Result<NotebookCellKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(NotebookCellKind::Markup),
            2 => Ok(NotebookCellKind::Code),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceOperationKind {
    #[serde(rename = "create")]
    /// Supports creating new files and folders.
    Create,

    #[serde(rename = "rename")]
    /// Supports renaming existing files and folders.
    Rename,

    #[serde(rename = "delete")]
    /// Supports deleting existing files and folders.
    Delete,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FailureHandlingKind {
    #[serde(rename = "abort")]
    /// Applying the workspace change is simply aborted if one of the changes provided
    /// fails. All operations executed before the failing operation stay executed.
    Abort,

    #[serde(rename = "transactional")]
    /// All operations are executed transactional. That means they either all
    /// succeed or no changes at all are applied to the workspace.
    Transactional,

    #[serde(rename = "textOnlyTransactional")]
    /// If the workspace edit contains only textual file changes they are executed transactional.
    /// If resource changes (create, rename or delete file) are part of the change the failure
    /// handling strategy is abort.
    TextOnlyTransactional,

    #[serde(rename = "undo")]
    /// The client tries to undo the operations already executed. But there is no
    /// guarantee that this is succeeding.
    Undo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrepareSupportDefaultBehavior {
    /// The client's default behavior is to select the identifier
    /// according the to language's syntax rule.
    Identifier = 1,
}
impl Serialize for PrepareSupportDefaultBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PrepareSupportDefaultBehavior::Identifier => serializer.serialize_u32(1),
        }
    }
}
impl<'de> Deserialize<'de> for PrepareSupportDefaultBehavior {
    fn deserialize<D>(deserializer: D) -> Result<PrepareSupportDefaultBehavior, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(PrepareSupportDefaultBehavior::Identifier),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenFormat {
    #[serde(rename = "relative")]
    Relative,
}

/// The definition of a symbol represented as one or many {@link Location locations}.
/// For most programming languages there is only one location at which a symbol is
/// defined.
///
/// Servers should prefer returning `DefinitionLink` over `Definition` if supported
/// by the client.
pub type Definition = Union2<Location, Vec<Location>>;

/// Information about where a symbol is defined.
///
/// Provides additional metadata over normal {@link Location location} definitions, including the range of
/// the defining symbol
pub type DefinitionLink = LocationLink;

/// The declaration of a symbol representation as one or many {@link Location locations}.
pub type Declaration = Union2<Location, Vec<Location>>;

/// Information about where a symbol is declared.
///
/// Provides additional metadata over normal {@link Location location} declarations, including the range of
/// the declaring symbol.
///
/// Servers should prefer returning `DeclarationLink` over `Declaration` if supported
/// by the client.
pub type DeclarationLink = LocationLink;

/// Inline value information can be provided by different means:
/// - directly as a text value (class InlineValueText).
/// - as a name to use for a variable lookup (class InlineValueVariableLookup)
/// - as an evaluatable expression (class InlineValueEvaluatableExpression)
/// The InlineValue types combines all inline value types into one type.
///
/// @since 3.17.0
pub type InlineValue = Union3<InlineValueText, InlineValueVariableLookup, InlineValueEvaluatableExpression>;

/// The result of a document diagnostic pull request. A report can
/// either be a full report containing all diagnostics for the
/// requested document or an unchanged report indicating that nothing
/// has changed in terms of diagnostics in comparison to the last
/// pull request.
///
/// @since 3.17.0
pub type DocumentDiagnosticReport = Union2<RelatedFullDocumentDiagnosticReport, RelatedUnchangedDocumentDiagnosticReport>;

pub type PrepareRenameResult = Union3<Range, PrepareRenamePlaceholder, PrepareRenameDefaultBehavior>;

/// A document selector is the combination of one or many document filters.
///
/// @sample `let sel:DocumentSelector = [{ language: 'typescript' }, { language: 'json', pattern: '**∕tsconfig.json' }]`;
///
/// The use of a string as a document filter is deprecated @since 3.16.0.
pub type DocumentSelector = Vec<DocumentFilter>;

pub type ProgressToken = Union2<i32, String>;

/// An identifier to refer to a change annotation stored with a workspace edit.
pub type ChangeAnnotationIdentifier = String;

/// A workspace diagnostic document report.
///
/// @since 3.17.0
pub type WorkspaceDocumentDiagnosticReport = Union2<WorkspaceFullDocumentDiagnosticReport, WorkspaceUnchangedDocumentDiagnosticReport>;

/// An event describing a change to a text document. If only a text is provided
/// it is considered to be the full content of the document.
pub type TextDocumentContentChangeEvent = Union2<TextDocumentContentChangePartial, TextDocumentContentChangeWholeDocument>;

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
pub type MarkedString = Union2<String, MarkedStringWithLanguage>;

/// A document filter describes a top level text document or
/// a notebook cell document.
///
/// @since 3.17.0 - support for NotebookCellTextDocumentFilter.
pub type DocumentFilter = Union2<TextDocumentFilter, NotebookCellTextDocumentFilter>;

/// The glob pattern. Either a string pattern or a relative pattern.
///
/// @since 3.17.0
pub type GlobPattern = Union2<Pattern, RelativePattern>;

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
pub type TextDocumentFilter = Union3<TextDocumentFilterLanguage, TextDocumentFilterScheme, TextDocumentFilterPattern>;

/// A notebook document filter denotes a notebook document by
/// different properties. The properties will be match
/// against the notebook's URI (same as with documents)
///
/// @since 3.17.0
pub type NotebookDocumentFilter = Union3<NotebookDocumentFilterNotebookType, NotebookDocumentFilterScheme, NotebookDocumentFilterPattern>;

/// The glob pattern to watch relative to the base path. Glob patterns can have the following syntax:
/// - `*` to match one or more characters in a path segment
/// - `?` to match on one character in a path segment
/// - `**` to match any number of path segments, including none
/// - `{}` to group conditions (e.g. `**​/*.{ts,js}` matches all TypeScript and JavaScript files)
/// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
/// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
///
/// @since 3.17.0
pub type Pattern = String;

pub type RegularExpressionEngineKind = String;
