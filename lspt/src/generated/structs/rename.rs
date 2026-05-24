// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link RenameRequest}.
pub struct RenameParams {
    /// The document to rename.
    pub text_document: TextDocumentIdentifier,

    /// The position at which this request was sent.
    pub position: Position,

    /// The new name of the symbol. If the given name is not valid the
    /// request must return a {@link ResponseError} with an
    /// appropriate message set.
    pub new_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link RenameRequest}.
pub struct RenameRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Renames should be checked and tested before being executed.
    ///
    /// @since version 3.12.0
    pub prepare_provider: Option<bool>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrepareRenameParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link RenameRequest}.
pub struct RenameOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Renames should be checked and tested before being executed.
    ///
    /// @since version 3.12.0
    pub prepare_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RenameClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether rename supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports testing for validity of rename operations
    /// before execution.
    ///
    /// @since 3.12.0
    pub prepare_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the default behavior result.
    ///
    /// The value indicates the default behavior used by the
    /// client.
    ///
    /// @since 3.16.0
    pub prepare_support_default_behavior: Option<PrepareSupportDefaultBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client honors the change annotations in
    /// text edits and resource operations returned via the
    /// rename request's workspace edit by for example presenting
    /// the workspace edit in the user interface and asking
    /// for confirmation.
    ///
    /// @since 3.16.0
    pub honors_change_annotations: Option<bool>,
}

pub type Params = RenameParams;

pub type RegistrationOptions = RenameRegistrationOptions;

pub type PrepareParams = PrepareRenameParams;

pub type Options = RenameOptions;

pub type ClientCapabilities = RenameClientCapabilities;