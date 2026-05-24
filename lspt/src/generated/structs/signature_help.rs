// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Parameters for a {@link SignatureHelpRequest}.
pub struct SignatureHelpParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The signature help context. This is only available if the client specifies
    /// to send this using the client capability `textDocument.signatureHelp.contextSupport === true`
    ///
    /// @since 3.15.0
    pub context: Option<SignatureHelpContext>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Signature help represents the signature of something
/// callable. There can be multiple signature but only one
/// active and only one active parameter.
pub struct SignatureHelp {
    /// One or more signatures.
    pub signatures: Vec<SignatureInformation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The active signature. If omitted or the value lies outside the
    /// range of `signatures` the value defaults to zero or is ignored if
    /// the `SignatureHelp` has no signatures.
    ///
    /// Whenever possible implementors should make an active decision about
    /// the active signature and shouldn't rely on a default value.
    ///
    /// In future version of the protocol this property might become
    /// mandatory to better express this.
    pub active_signature: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The active parameter of the active signature.
    ///
    /// If `null`, no parameter of the signature is active (for example a named
    /// argument that does not match any declared parameters). This is only valid
    /// if the client specifies the client capability
    /// `textDocument.signatureHelp.noActiveParameterSupport === true`
    ///
    /// If omitted or the value lies outside the range of
    /// `signatures[activeSignature].parameters` defaults to 0 if the active
    /// signature has parameters.
    ///
    /// If the active signature has no parameters it is ignored.
    ///
    /// In future version of the protocol this property might become
    /// mandatory (but still nullable) to better express the active parameter if
    /// the active signature does have any.
    pub active_parameter: Option<u32>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link SignatureHelpRequest}.
pub struct SignatureHelpRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of characters that trigger signature help automatically.
    pub trigger_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of characters that re-trigger signature help.
    ///
    /// These trigger characters are only active when signature help is already showing. All trigger characters
    /// are also counted as re-trigger characters.
    ///
    /// @since 3.15.0
    pub retrigger_characters: Option<Vec<String>>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Additional information about the context in which a signature help request was triggered.
///
/// @since 3.15.0
pub struct SignatureHelpContext {
    /// Action that caused signature help to be triggered.
    pub trigger_kind: SignatureHelpTriggerKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Character that caused signature help to be triggered.
    ///
    /// This is undefined when `triggerKind !== SignatureHelpTriggerKind.TriggerCharacter`
    pub trigger_character: Option<String>,

    /// `true` if signature help was already showing when it was triggered.
    ///
    /// Retriggers occurs when the signature help is already active and can be caused by actions such as
    /// typing a trigger character, a cursor move, or document content changes.
    pub is_retrigger: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The currently active `SignatureHelp`.
    ///
    /// The `activeSignatureHelp` has its `SignatureHelp.activeSignature` field updated based on
    /// the user navigating through available signatures.
    pub active_signature_help: Option<SignatureHelp>,
}


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
/// Server Capabilities for a {@link SignatureHelpRequest}.
pub struct SignatureHelpOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of characters that trigger signature help automatically.
    pub trigger_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// List of characters that re-trigger signature help.
    ///
    /// These trigger characters are only active when signature help is already showing. All trigger characters
    /// are also counted as re-trigger characters.
    ///
    /// @since 3.15.0
    pub retrigger_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a parameter of a callable-signature. A parameter can
/// have a label and a doc-comment.
pub struct ParameterInformation {
    /// The label of this parameter information.
    ///
    /// Either a string or an inclusive start and exclusive end offsets within its containing
    /// signature label. (see SignatureInformation.label). The offsets are based on a UTF-16
    /// string representation as `Position` and `Range` does.
    ///
    /// To avoid ambiguities a server should use the [start, end] offset value instead of using
    /// a substring. Whether a client support this is controlled via `labelOffsetSupport` client
    /// capability.
    ///
    /// *Note*: a label of type string should be a substring of its containing signature label.
    /// Its intended use case is to highlight the parameter label part in the `SignatureInformation.label`.
    pub label: ParameterInformationLabel,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The human-readable doc-comment of this parameter. Will be shown
    /// in the UI but can be omitted.
    pub documentation: Option<ParameterInformationDocumentation>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client Capabilities for a {@link SignatureHelpRequest}.
pub struct SignatureHelpClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether signature help supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the following `SignatureInformation`
    /// specific properties.
    pub signature_information: Option<ClientSignatureInformationOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports to send additional context information for a
    /// `textDocument/signatureHelp` request. A client that opts into
    /// contextSupport will also support the `retriggerCharacters` on
    /// `SignatureHelpOptions`.
    ///
    /// @since 3.15.0
    pub context_support: Option<bool>,
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


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientSignatureParameterInformationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports processing label offsets instead of a
    /// simple label string.
    ///
    /// @since 3.14.0
    pub label_offset_support: Option<bool>,
}

pub type Params = SignatureHelpParams;

pub type RegistrationOptions = SignatureHelpRegistrationOptions;

pub type Context = SignatureHelpContext;

pub type ClientCapabilities = SignatureHelpClientCapabilities;