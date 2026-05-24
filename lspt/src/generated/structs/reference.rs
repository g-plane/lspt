// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link ReferencesRequest}.
pub struct ReferenceParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    pub context: ReferenceContext,

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
/// Registration options for a {@link ReferencesRequest}.
pub struct ReferenceRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Value-object that contains additional information when
/// requesting references.
pub struct ReferenceContext {
    /// Include the declaration of the current symbol.
    pub include_declaration: bool,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Reference options.
pub struct ReferenceOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client Capabilities for a {@link ReferencesRequest}.
pub struct ReferenceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether references supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

pub type Params = ReferenceParams;

pub type RegistrationOptions = ReferenceRegistrationOptions;

pub type Options = ReferenceOptions;

pub type ClientCapabilities = ReferenceClientCapabilities;