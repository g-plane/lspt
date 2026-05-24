// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link DocumentHighlightRequest}.
pub struct DocumentHighlightParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A document highlight is a range inside a text document which deserves
/// special attention. Usually a document highlight is visualized by changing
/// the background color of its range.
pub struct DocumentHighlight {
    /// The range this highlight applies to.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The highlight kind, default is {@link DocumentHighlightKind.Text text}.
    pub kind: Option<DocumentHighlightKind>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link DocumentHighlightRequest}.
pub struct DocumentHighlightRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link DocumentHighlightRequest}.
pub struct DocumentHighlightOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client Capabilities for a {@link DocumentHighlightRequest}.
pub struct DocumentHighlightClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether document highlight supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

pub type Params = DocumentHighlightParams;

pub type RegistrationOptions = DocumentHighlightRegistrationOptions;

pub type Options = DocumentHighlightOptions;

pub type ClientCapabilities = DocumentHighlightClientCapabilities;