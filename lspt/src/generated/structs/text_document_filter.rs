// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A document filter where `language` is required field.
///
/// @since 3.18.0
pub struct TextDocumentFilterLanguage {
    /// A language id, like `typescript`.
    pub language: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
    pub scheme: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A glob pattern, like **​/*.{ts,js}. See TextDocumentFilter for examples.
    ///
    /// @since 3.18.0 - support for relative patterns. Whether clients support
    /// relative patterns depends on the client capability
    /// `textDocuments.filters.relativePatternSupport`.
    pub pattern: Option<GlobPattern>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A document filter where `scheme` is required field.
///
/// @since 3.18.0
pub struct TextDocumentFilterScheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A language id, like `typescript`.
    pub language: Option<String>,

    /// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
    pub scheme: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A glob pattern, like **​/*.{ts,js}. See TextDocumentFilter for examples.
    ///
    /// @since 3.18.0 - support for relative patterns. Whether clients support
    /// relative patterns depends on the client capability
    /// `textDocuments.filters.relativePatternSupport`.
    pub pattern: Option<GlobPattern>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A document filter where `pattern` is required field.
///
/// @since 3.18.0
pub struct TextDocumentFilterPattern {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A language id, like `typescript`.
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A Uri {@link Uri.scheme scheme}, like `file` or `untitled`.
    pub scheme: Option<String>,

    /// A glob pattern, like **​/*.{ts,js}. See TextDocumentFilter for examples.
    ///
    /// @since 3.18.0 - support for relative patterns. Whether clients support
    /// relative patterns depends on the client capability
    /// `textDocuments.filters.relativePatternSupport`.
    pub pattern: GlobPattern,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentFilterClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports Relative Patterns.
    ///
    /// @since 3.18.0
    pub relative_pattern_support: Option<bool>,
}

pub type ClientCapabilities = TextDocumentFilterClientCapabilities;