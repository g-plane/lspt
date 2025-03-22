// DO NOT EDIT THIS GENERATED FILE.

use crate::*;

pub trait Notification {
    const METHOD: &'static str;
    type Params: serde::de::DeserializeOwned + Serialize + Send + Sync + 'static;
}

pub enum DidChangeWorkspaceFoldersNotification {}
impl Notification for DidChangeWorkspaceFoldersNotification {
    const METHOD: &'static str = "workspace/didChangeWorkspaceFolders";
    type Params = DidChangeWorkspaceFoldersParams;
}

pub enum WorkDoneProgressCancelNotification {}
impl Notification for WorkDoneProgressCancelNotification {
    const METHOD: &'static str = "window/workDoneProgress/cancel";
    type Params = WorkDoneProgressCancelParams;
}

pub enum DidCreateFilesNotification {}
impl Notification for DidCreateFilesNotification {
    const METHOD: &'static str = "workspace/didCreateFiles";
    type Params = CreateFilesParams;
}

pub enum DidRenameFilesNotification {}
impl Notification for DidRenameFilesNotification {
    const METHOD: &'static str = "workspace/didRenameFiles";
    type Params = RenameFilesParams;
}

pub enum DidDeleteFilesNotification {}
impl Notification for DidDeleteFilesNotification {
    const METHOD: &'static str = "workspace/didDeleteFiles";
    type Params = DeleteFilesParams;
}

pub enum DidOpenNotebookDocumentNotification {}
impl Notification for DidOpenNotebookDocumentNotification {
    const METHOD: &'static str = "notebookDocument/didOpen";
    type Params = DidOpenNotebookDocumentParams;
}

pub enum DidChangeNotebookDocumentNotification {}
impl Notification for DidChangeNotebookDocumentNotification {
    const METHOD: &'static str = "notebookDocument/didChange";
    type Params = DidChangeNotebookDocumentParams;
}

pub enum DidSaveNotebookDocumentNotification {}
impl Notification for DidSaveNotebookDocumentNotification {
    const METHOD: &'static str = "notebookDocument/didSave";
    type Params = DidSaveNotebookDocumentParams;
}

pub enum DidCloseNotebookDocumentNotification {}
impl Notification for DidCloseNotebookDocumentNotification {
    const METHOD: &'static str = "notebookDocument/didClose";
    type Params = DidCloseNotebookDocumentParams;
}

pub enum InitializedNotification {}
impl Notification for InitializedNotification {
    const METHOD: &'static str = "initialized";
    type Params = InitializedParams;
}

pub enum ExitNotification {}
impl Notification for ExitNotification {
    const METHOD: &'static str = "exit";
    type Params = ();
}

pub enum DidChangeConfigurationNotification {}
impl Notification for DidChangeConfigurationNotification {
    const METHOD: &'static str = "workspace/didChangeConfiguration";
    type Params = DidChangeConfigurationParams;
}

pub enum ShowMessageNotification {}
impl Notification for ShowMessageNotification {
    const METHOD: &'static str = "window/showMessage";
    type Params = ShowMessageParams;
}

pub enum LogMessageNotification {}
impl Notification for LogMessageNotification {
    const METHOD: &'static str = "window/logMessage";
    type Params = LogMessageParams;
}

pub enum TelemetryEventNotification {}
impl Notification for TelemetryEventNotification {
    const METHOD: &'static str = "telemetry/event";
    type Params = serde_json::Value;
}

pub enum DidOpenTextDocumentNotification {}
impl Notification for DidOpenTextDocumentNotification {
    const METHOD: &'static str = "textDocument/didOpen";
    type Params = DidOpenTextDocumentParams;
}

pub enum DidChangeTextDocumentNotification {}
impl Notification for DidChangeTextDocumentNotification {
    const METHOD: &'static str = "textDocument/didChange";
    type Params = DidChangeTextDocumentParams;
}

pub enum DidCloseTextDocumentNotification {}
impl Notification for DidCloseTextDocumentNotification {
    const METHOD: &'static str = "textDocument/didClose";
    type Params = DidCloseTextDocumentParams;
}

pub enum DidSaveTextDocumentNotification {}
impl Notification for DidSaveTextDocumentNotification {
    const METHOD: &'static str = "textDocument/didSave";
    type Params = DidSaveTextDocumentParams;
}

pub enum WillSaveTextDocumentNotification {}
impl Notification for WillSaveTextDocumentNotification {
    const METHOD: &'static str = "textDocument/willSave";
    type Params = WillSaveTextDocumentParams;
}

pub enum DidChangeWatchedFilesNotification {}
impl Notification for DidChangeWatchedFilesNotification {
    const METHOD: &'static str = "workspace/didChangeWatchedFiles";
    type Params = DidChangeWatchedFilesParams;
}

pub enum PublishDiagnosticsNotification {}
impl Notification for PublishDiagnosticsNotification {
    const METHOD: &'static str = "textDocument/publishDiagnostics";
    type Params = PublishDiagnosticsParams;
}

pub enum SetTraceNotification {}
impl Notification for SetTraceNotification {
    const METHOD: &'static str = "$/setTrace";
    type Params = SetTraceParams;
}

pub enum LogTraceNotification {}
impl Notification for LogTraceNotification {
    const METHOD: &'static str = "$/logTrace";
    type Params = LogTraceParams;
}

pub enum CancelNotification {}
impl Notification for CancelNotification {
    const METHOD: &'static str = "$/cancelRequest";
    type Params = CancelParams;
}

pub enum ProgressNotification {}
impl Notification for ProgressNotification {
    const METHOD: &'static str = "$/progress";
    type Params = ProgressParams;
}
