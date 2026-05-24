// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A partial result for a document diagnostic report.
///
/// @since 3.17.0
pub struct DocumentDiagnosticReportPartialResult {
    pub related_documents: HashMap<Uri, RelatedDocumentDiagnosticReport>,
}

pub type PartialResult = DocumentDiagnosticReportPartialResult;