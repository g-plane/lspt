// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A parameter literal used in inlay hint requests.
///
/// @since 3.17.0
pub struct InlayHintParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The document range for which inlay hints should be computed.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inlay hint information.
///
/// @since 3.17.0
pub struct InlayHint {
    /// The position of this hint.
    ///
    /// If multiple hints have the same position, they will be shown in the order
    /// they appear in the response.
    pub position: Position,

    /// The label of this hint. A human readable string or an array of
    /// InlayHintLabelPart label parts.
    ///
    /// *Note* that neither the string nor the label part can be empty.
    pub label: InlayHintLabel,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The kind of this hint. Can be omitted in which case the client
    /// should fall back to a reasonable default.
    pub kind: Option<InlayHintKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional text edits that are performed when accepting this inlay hint.
    ///
    /// *Note* that edits are expected to change the document so that the inlay
    /// hint (or its nearest variant) is now part of the document and the inlay
    /// hint itself is now obsolete.
    pub text_edits: Option<Vec<TextEdit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The tooltip text when you hover over this item.
    pub tooltip: Option<InlayHintTooltip>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Render padding before the hint.
    ///
    /// Note: Padding should use the editor's background color, not the
    /// background color of the hint itself. That means padding can be used
    /// to visually align/separate an inlay hint.
    pub padding_left: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Render padding after the hint.
    ///
    /// Note: Padding should use the editor's background color, not the
    /// background color of the hint itself. That means padding can be used
    /// to visually align/separate an inlay hint.
    pub padding_right: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on an inlay hint between
    /// a `textDocument/inlayHint` and a `inlayHint/resolve` request.
    pub data: Option<serde_json::Value>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inlay hint options used during static or dynamic registration.
///
/// @since 3.17.0
pub struct InlayHintRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for an inlay hint item.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An inlay hint label part allows for interactive and composite labels
/// of inlay hints.
///
/// @since 3.17.0
pub struct InlayHintLabelPart {
    /// The value of this label part.
    pub value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The tooltip text when you hover over this label part. Depending on
    /// the client capability `inlayHint.resolveSupport` clients might resolve
    /// this property late using the resolve request.
    pub tooltip: Option<InlayHintLabelPartTooltip>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional source code location that represents this
    /// label part.
    ///
    /// The editor will use this location for the hover and for code navigation
    /// features: This part will become a clickable link that resolves to the
    /// definition of the symbol at the given location (not necessarily the
    /// location itself), it shows the hover that shows at the given location,
    /// and it shows a context menu with further code navigation commands.
    ///
    /// Depending on the client capability `inlayHint.resolveSupport` clients
    /// might resolve this property late using the resolve request.
    pub location: Option<Location>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional command for this label part.
    ///
    /// Depending on the client capability `inlayHint.resolveSupport` clients
    /// might resolve this property late using the resolve request.
    pub command: Option<Command>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inlay hint options used during static registration.
///
/// @since 3.17.0
pub struct InlayHintOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for an inlay hint item.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client workspace capabilities specific to inlay hints.
///
/// @since 3.17.0
pub struct InlayHintWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from
    /// the server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// inlay hints currently shown. It should be used with absolute care and
    /// is useful for situation where a server for example detects a project wide
    /// change that requires such a calculation.
    pub refresh_support: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inlay hint client capabilities.
///
/// @since 3.17.0
pub struct InlayHintClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether inlay hints support dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates which properties a client can resolve lazily on an inlay
    /// hint.
    pub resolve_support: Option<ClientInlayHintResolveOptions>,
}

pub type Params = InlayHintParams;

pub type RegistrationOptions = InlayHintRegistrationOptions;

pub type Options = InlayHintOptions;

pub type WorkspaceClientCapabilities = InlayHintWorkspaceClientCapabilities;

pub type ClientCapabilities = InlayHintClientCapabilities;