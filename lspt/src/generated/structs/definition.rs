// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link DefinitionRequest}.
pub struct DefinitionParams {
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
/// Registration options for a {@link DefinitionRequest}.
pub struct DefinitionRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Server Capabilities for a {@link DefinitionRequest}.
pub struct DefinitionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client Capabilities for a {@link DefinitionRequest}.
pub struct DefinitionClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether definition supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports additional metadata in the form of definition links.
    ///
    /// @since 3.14.0
    pub link_support: Option<bool>,
}

pub type Params = DefinitionParams;

pub type RegistrationOptions = DefinitionRegistrationOptions;

pub type Options = DefinitionOptions;

pub type ClientCapabilities = DefinitionClientCapabilities;