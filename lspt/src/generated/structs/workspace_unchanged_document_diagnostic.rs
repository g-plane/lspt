// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

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
