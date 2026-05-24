// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link CodeLensRequest}.
pub struct CodeLensParams {
    /// The document to request code lens for.
    pub text_document: TextDocumentIdentifier,

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
/// A code lens represents a {@link Command command} that should be shown along with
/// source text, like the number of references, a way to run tests, etc.
///
/// A code lens is _unresolved_ when no command is associated to it. For performance
/// reasons the creation of a code lens and resolving should be done in two stages.
pub struct CodeLens {
    /// The range in which this code lens is valid. Should only span a single line.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The command this code lens represents.
    pub command: Option<Command>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on a code lens item between
    /// a {@link CodeLensRequest} and a {@link CodeLensResolveRequest}
    pub data: Option<serde_json::Value>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link CodeLensRequest}.
pub struct CodeLensRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Code lens has a resolve provider as well.
    pub resolve_provider: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Code Lens provider options of a {@link CodeLensRequest}.
pub struct CodeLensOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Code lens has a resolve provider as well.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct CodeLensWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from the
    /// server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// code lenses currently shown. It should be used with absolute care and is
    /// useful for situation where a server for example detect a project wide
    /// change that requires such a calculation.
    pub refresh_support: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The client capabilities  of a {@link CodeLensRequest}.
pub struct CodeLensClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code lens supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports resolving additional code lens
    /// properties via a separate `codeLens/resolve` request.
    ///
    /// @since 3.18.0
    pub resolve_support: Option<ClientCodeLensResolveOptions>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCodeLensResolveOptions {
    /// The properties that a client can resolve lazily.
    pub properties: Vec<String>,
}

pub type Params = CodeLensParams;

pub type RegistrationOptions = CodeLensRegistrationOptions;

pub type Options = CodeLensOptions;

pub type WorkspaceClientCapabilities = CodeLensWorkspaceClientCapabilities;

pub type ClientCapabilities = CodeLensClientCapabilities;

pub type ClientResolveOptions = ClientCodeLensResolveOptions;