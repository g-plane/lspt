// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link DocumentRangeFormattingRequest}.
pub struct DocumentRangeFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The range to format
    pub range: Range,

    /// The format options
    pub options: FormattingOptions,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link DocumentRangeFormattingRequest}.
pub struct DocumentRangeFormattingRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the server supports formatting multiple ranges at once.
    ///
    /// @since 3.18.0
    /// @proposed
    pub ranges_support: Option<bool>,
}


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


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link DocumentRangeFormattingRequest}.
pub struct DocumentRangeFormattingOptions {
    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the server supports formatting multiple ranges at once.
    ///
    /// @since 3.18.0
    /// @proposed
    pub ranges_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities of a {@link DocumentRangeFormattingRequest}.
pub struct DocumentRangeFormattingClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether range formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports formatting multiple ranges at once.
    ///
    /// @since 3.18.0
    /// @proposed
    pub ranges_support: Option<bool>,
}

pub type RegistrationOptions = DocumentRangeFormattingRegistrationOptions;

pub type Options = DocumentRangeFormattingOptions;

pub type ClientCapabilities = DocumentRangeFormattingClientCapabilities;