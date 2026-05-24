// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

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
    pub related_documents: HashMap<Uri, RelatedDocumentDiagnosticReport>,
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

pub type RegistrationOptions = DiagnosticRegistrationOptions;

pub type FullDocumentReport = FullDocumentDiagnosticReport;

pub type UnchangedDocumentReport = UnchangedDocumentDiagnosticReport;

pub type Options = DiagnosticOptions;

pub type ClientCapabilities = DiagnosticClientCapabilities;

pub mod document {
    pub type Params = super::DocumentDiagnosticParams;

    pub type ReportPartialResult = super::DocumentDiagnosticReportPartialResult;
}

pub mod workspace {
    pub type Params = super::WorkspaceDiagnosticParams;

    pub type Report = super::WorkspaceDiagnosticReport;

    pub type ReportPartialResult = super::WorkspaceDiagnosticReportPartialResult;

    pub type ClientCapabilities = super::DiagnosticWorkspaceClientCapabilities;
}