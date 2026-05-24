// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A partial result for a workspace diagnostic report.
///
/// @since 3.17.0
pub struct WorkspaceDiagnosticReportPartialResult {
    pub items: Vec<WorkspaceDocumentDiagnosticReport>,
}

pub type PartialResult = WorkspaceDiagnosticReportPartialResult;