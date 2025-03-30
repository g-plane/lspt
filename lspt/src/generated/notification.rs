// DO NOT EDIT THIS GENERATED FILE.

use serde::Serialize;
use super::*;

pub trait Notification {
    const METHOD: &'static str;
    type Params: serde::de::DeserializeOwned + Serialize + Send + Sync + 'static;
}

pub enum DidChangeWorkspaceFolders {}
impl Notification for DidChangeWorkspaceFoldersNotification {
    const METHOD: &'static str = "workspace/didChangeWorkspaceFolders";
    type Params = DidChangeWorkspaceFoldersParams;
}

pub enum WorkDoneProgressCancel {}
impl Notification for WorkDoneProgressCancelNotification {
    const METHOD: &'static str = "window/workDoneProgress/cancel";
    type Params = WorkDoneProgressCancelParams;
}

pub enum DidCreateFiles {}
impl Notification for DidCreateFilesNotification {
    const METHOD: &'static str = "workspace/didCreateFiles";
    type Params = CreateFilesParams;
}

pub enum DidRenameFiles {}
impl Notification for DidRenameFilesNotification {
    const METHOD: &'static str = "workspace/didRenameFiles";
    type Params = RenameFilesParams;
}

pub enum DidDeleteFiles {}
impl Notification for DidDeleteFilesNotification {
    const METHOD: &'static str = "workspace/didDeleteFiles";
    type Params = DeleteFilesParams;
}

pub enum DidOpenNotebookDocument {}
impl Notification for DidOpenNotebookDocumentNotification {
    const METHOD: &'static str = "notebookDocument/didOpen";
    type Params = DidOpenNotebookDocumentParams;
}

pub enum DidChangeNotebookDocument {}
impl Notification for DidChangeNotebookDocumentNotification {
    const METHOD: &'static str = "notebookDocument/didChange";
    type Params = DidChangeNotebookDocumentParams;
}

pub enum DidSaveNotebookDocument {}
impl Notification for DidSaveNotebookDocumentNotification {
    const METHOD: &'static str = "notebookDocument/didSave";
    type Params = DidSaveNotebookDocumentParams;
}

pub enum DidCloseNotebookDocument {}
impl Notification for DidCloseNotebookDocumentNotification {
    const METHOD: &'static str = "notebookDocument/didClose";
    type Params = DidCloseNotebookDocumentParams;
}

pub enum Initialized {}
impl Notification for InitializedNotification {
    const METHOD: &'static str = "initialized";
    type Params = InitializedParams;
}

pub enum Exit {}
impl Notification for ExitNotification {
    const METHOD: &'static str = "exit";
    type Params = ();
}

pub enum DidChangeConfiguration {}
impl Notification for DidChangeConfigurationNotification {
    const METHOD: &'static str = "workspace/didChangeConfiguration";
    type Params = DidChangeConfigurationParams;
}

pub enum ShowMessage {}
impl Notification for ShowMessageNotification {
    const METHOD: &'static str = "window/showMessage";
    type Params = ShowMessageParams;
}

pub enum LogMessage {}
impl Notification for LogMessageNotification {
    const METHOD: &'static str = "window/logMessage";
    type Params = LogMessageParams;
}

pub enum TelemetryEvent {}
impl Notification for TelemetryEventNotification {
    const METHOD: &'static str = "telemetry/event";
    type Params = serde_json::Value;
}

pub enum DidOpenTextDocument {}
impl Notification for DidOpenTextDocumentNotification {
    const METHOD: &'static str = "textDocument/didOpen";
    type Params = DidOpenTextDocumentParams;
}

pub enum DidChangeTextDocument {}
impl Notification for DidChangeTextDocumentNotification {
    const METHOD: &'static str = "textDocument/didChange";
    type Params = DidChangeTextDocumentParams;
}

pub enum DidCloseTextDocument {}
impl Notification for DidCloseTextDocumentNotification {
    const METHOD: &'static str = "textDocument/didClose";
    type Params = DidCloseTextDocumentParams;
}

pub enum DidSaveTextDocument {}
impl Notification for DidSaveTextDocumentNotification {
    const METHOD: &'static str = "textDocument/didSave";
    type Params = DidSaveTextDocumentParams;
}

pub enum WillSaveTextDocument {}
impl Notification for WillSaveTextDocumentNotification {
    const METHOD: &'static str = "textDocument/willSave";
    type Params = WillSaveTextDocumentParams;
}

pub enum DidChangeWatchedFiles {}
impl Notification for DidChangeWatchedFilesNotification {
    const METHOD: &'static str = "workspace/didChangeWatchedFiles";
    type Params = DidChangeWatchedFilesParams;
}

pub enum PublishDiagnostics {}
impl Notification for PublishDiagnosticsNotification {
    const METHOD: &'static str = "textDocument/publishDiagnostics";
    type Params = PublishDiagnosticsParams;
}

pub enum SetTrace {}
impl Notification for SetTraceNotification {
    const METHOD: &'static str = "$/setTrace";
    type Params = SetTraceParams;
}

pub enum LogTrace {}
impl Notification for LogTraceNotification {
    const METHOD: &'static str = "$/logTrace";
    type Params = LogTraceParams;
}

pub enum Cancel {}
impl Notification for CancelNotification {
    const METHOD: &'static str = "$/cancelRequest";
    type Params = CancelParams;
}

pub enum Progress {}
impl Notification for ProgressNotification {
    const METHOD: &'static str = "$/progress";
    type Params = ProgressParams;
}
