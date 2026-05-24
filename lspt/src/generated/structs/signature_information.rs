// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents the signature of something callable. A signature
/// can have a label, like a function-name, a doc-comment, and
/// a set of parameters.
pub struct SignatureInformation {
    /// The label of this signature. Will be shown in
    /// the UI.
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The human-readable doc-comment of this signature. Will be shown
    /// in the UI but can be omitted.
    pub documentation: Option<SignatureInformationDocumentation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The parameters of this signature.
    pub parameters: Option<Vec<ParameterInformation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The index of the active parameter.
    ///
    /// If `null`, no parameter of the signature is active (for example a named
    /// argument that does not match any declared parameters). This is only valid
    /// if the client specifies the client capability
    /// `textDocument.signatureHelp.noActiveParameterSupport === true`
    ///
    /// If provided (or `null`), this is used in place of
    /// `SignatureHelp.activeParameter`.
    ///
    /// @since 3.16.0
    pub active_parameter: Option<u32>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientSignatureInformationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the following content formats for the documentation
    /// property. The order describes the preferred format of the client.
    pub documentation_format: Option<Vec<MarkupKind>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client capabilities specific to parameter information.
    pub parameter_information: Option<ClientSignatureParameterInformationOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the `activeParameter` property on `SignatureInformation`
    /// literal.
    ///
    /// @since 3.16.0
    pub active_parameter_support: Option<bool>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the `activeParameter` property on
    /// `SignatureHelp`/`SignatureInformation` being set to `null` to
    /// indicate that no parameter should be active.
    ///
    /// @since 3.18.0
    /// @proposed
    pub no_active_parameter_support: Option<bool>,
}

pub type Options = ClientSignatureInformationOptions;