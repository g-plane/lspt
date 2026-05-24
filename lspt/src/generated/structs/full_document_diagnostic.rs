// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

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
