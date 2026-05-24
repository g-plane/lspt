// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

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

pub type Params = WorkspaceDiagnosticParams;

pub type Report = WorkspaceDiagnosticReport;