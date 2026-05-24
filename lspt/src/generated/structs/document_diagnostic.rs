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
