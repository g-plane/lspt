// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link DocumentRangesFormattingRequest}.
///
/// @since 3.18.0
/// @proposed
pub struct DocumentRangesFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The ranges to format
    pub ranges: Vec<Range>,

    /// The format options
    pub options: FormattingOptions,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}
