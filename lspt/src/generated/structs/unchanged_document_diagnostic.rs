// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

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
