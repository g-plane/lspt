// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link WorkspaceSymbolRequest}.
pub struct WorkspaceSymbolParams {
    /// A query string to filter symbols by. Clients may send an empty
    /// string here to request all symbols.
    ///
    /// The `query`-parameter should be interpreted in a *relaxed way* as editors
    /// will apply their own highlighting and scoring on the results. A good rule
    /// of thumb is to match case-insensitive and to simply check that the
    /// characters of *query* appear in their order in a candidate symbol.
    /// Servers shouldn't use prefix, substring, or similar strict matching.
    pub query: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A special workspace symbol that supports locations without a range.
///
/// See also SymbolInformation.
///
/// @since 3.17.0
pub struct WorkspaceSymbol {
    /// The name of this symbol.
    pub name: String,

    /// The kind of this symbol.
    pub kind: SymbolKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this symbol.
    ///
    /// @since 3.16.0
    pub tags: Option<Vec<SymbolTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The name of the symbol containing this symbol. This information is for
    /// user interface purposes (e.g. to render a qualifier in the user interface
    /// if necessary). It can't be used to re-infer a hierarchy for the document
    /// symbols.
    pub container_name: Option<String>,

    /// The location of the symbol. Whether a server is allowed to
    /// return a location without a range depends on the client
    /// capability `workspace.symbol.resolveSupport`.
    ///
    /// See SymbolInformation#location for more details.
    pub location: WorkspaceSymbolLocation,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on a workspace symbol between a
    /// workspace symbol request and a workspace symbol resolve request.
    pub data: Option<serde_json::Value>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link WorkspaceSymbolRequest}.
pub struct WorkspaceSymbolRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a workspace symbol.
    ///
    /// @since 3.17.0
    pub resolve_provider: Option<bool>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Location with only uri and does not include range.
///
/// @since 3.18.0
pub struct LocationUriOnly {
    pub uri: Uri,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Server capabilities for a {@link WorkspaceSymbolRequest}.
pub struct WorkspaceSymbolOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a workspace symbol.
    ///
    /// @since 3.17.0
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities for a {@link WorkspaceSymbolRequest}.
pub struct WorkspaceSymbolClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Symbol request supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specific capabilities for the `SymbolKind` in the `workspace/symbol` request.
    pub symbol_kind: Option<ClientSymbolKindOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports tags on `SymbolInformation`.
    /// Clients supporting tags have to handle unknown tags gracefully.
    ///
    /// @since 3.16.0
    pub tag_support: Option<ClientSymbolTagOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client support partial workspace symbols. The client will send the
    /// request `workspaceSymbol/resolve` to the server to resolve additional
    /// properties.
    ///
    /// @since 3.17.0
    pub resolve_support: Option<ClientSymbolResolveOptions>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientSymbolResolveOptions {
    /// The properties that a client can resolve lazily. Usually
    /// `location.range`
    pub properties: Vec<String>,
}

pub type Params = WorkspaceSymbolParams;

pub type RegistrationOptions = WorkspaceSymbolRegistrationOptions;

pub type ClientCapabilities = WorkspaceSymbolClientCapabilities;