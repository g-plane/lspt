// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link HoverRequest}.
pub struct HoverParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result of a hover request.
pub struct Hover {
    /// The hover's content
    pub contents: HoverContents,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional range inside the text document that is used to
    /// visualize the hover, e.g. by changing the background color.
    pub range: Option<Range>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link HoverRequest}.
pub struct HoverRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Hover options.
pub struct HoverOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoverClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether hover supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the following content formats for the content
    /// property. The order describes the preferred format of the client.
    pub content_format: Option<Vec<MarkupKind>>,
}
