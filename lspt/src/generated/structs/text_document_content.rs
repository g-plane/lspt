// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentParams {
    /// The uri of the text document.
    pub uri: Uri,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Result of the `workspace/textDocumentContent` request.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentResult {
    /// The text content of the text document. Please note, that the content of
    /// any subsequent open notifications for the text document might differ
    /// from the returned content due to whitespace and line ending
    /// normalizations done on the client
    pub text: String,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Text document content provider registration options.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentRegistrationOptions {
    /// The schemes for which the server provides content.
    pub schemes: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Text document content provider options.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentOptions {
    /// The schemes for which the server provides content.
    pub schemes: Vec<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct TextDocumentContentChangePartial {
    /// The range of the document that changed.
    pub range: Range,

    /// The new text for the provided range.
    pub text: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct TextDocumentContentChangeWholeDocument {
    /// The new text of the whole document.
    pub text: String,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities for a text document content provider.
///
/// @since 3.18.0
/// @proposed
pub struct TextDocumentContentClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Text document content provider supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}
