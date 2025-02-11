// DO NOT EDIT THIS GENERATED FILE.

use crate::*;
use serde::{Deserialize, Serialize};

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
