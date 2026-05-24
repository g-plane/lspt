// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensParams {
    /// The text document.
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
/// @since 3.16.0
pub struct SemanticTokens {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional result id. If provided and clients support delta updating
    /// the client will include the result id in the next semantic token request.
    /// A server can then instead of computing all semantic tokens again simply
    /// send a delta.
    pub result_id: Option<String>,

    /// The actual tokens.
    pub data: Vec<u32>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensPartialResult {
    pub data: Vec<u32>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    /// The legend used by the server
    pub legend: SemanticTokensLegend,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Server supports providing semantic tokens for a specific range
    /// of a document.
    pub range: Option<SemanticTokensRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Server supports providing semantic tokens for a full document.
    pub full: Option<SemanticTokensFull>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensOptions {
    /// The legend used by the server
    pub legend: SemanticTokensLegend,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Server supports providing semantic tokens for a specific range
    /// of a document.
    pub range: Option<SemanticTokensRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Server supports providing semantic tokens for a full document.
    pub full: Option<SemanticTokensFull>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensEdit {
    /// The start offset of the edit.
    pub start: u32,

    /// The count of elements to remove.
    pub delete_count: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The elements to insert.
    pub data: Option<Vec<u32>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensLegend {
    /// The token types a server uses.
    pub token_types: Vec<String>,

    /// The token modifiers a server uses.
    pub token_modifiers: Vec<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Semantic tokens options to support deltas for full documents
///
/// @since 3.18.0
pub struct SemanticTokensFullDelta {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server supports deltas for full documents.
    pub delta: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from
    /// the server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// semantic tokens currently shown. It should be used with absolute care
    /// and is useful for situation where a server for example detects a project
    /// wide change that requires such a calculation.
    pub refresh_support: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.16.0
pub struct SemanticTokensClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,

    /// Which requests the client supports and might send to the server
    /// depending on the server's capability. Please note that clients might not
    /// show semantic tokens or degrade some of the user experience if a range
    /// or full request is advertised by the client but not provided by the
    /// server. If for example the client capability `requests.full` and
    /// `request.range` are both set to true but the server only provides a
    /// range provider the client might not render a minimap correctly or might
    /// even decide to not show any semantic tokens at all.
    pub requests: ClientSemanticTokensRequestOptions,

    /// The token types that the client supports.
    pub token_types: Vec<String>,

    /// The token modifiers that the client supports.
    pub token_modifiers: Vec<String>,

    /// The token formats the clients supports.
    pub formats: Vec<TokenFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports tokens that can overlap each other.
    pub overlapping_token_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports tokens that can span multiple lines.
    pub multiline_token_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client allows the server to actively cancel a
    /// semantic token request, e.g. supports returning
    /// LSPErrorCodes.ServerCancelled. If a server does the client
    /// needs to retrigger the request.
    ///
    /// @since 3.17.0
    pub server_cancel_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client uses semantic tokens to augment existing
    /// syntax tokens. If set to `true` client side created syntax
    /// tokens and semantic tokens are both used for colorization. If
    /// set to `false` the client only uses the returned semantic tokens
    /// for colorization.
    ///
    /// If the value is `undefined` then the client behavior is not
    /// specified.
    ///
    /// @since 3.17.0
    pub augments_syntax_tokens: Option<bool>,
}

pub type Params = SemanticTokensParams;

pub type PartialResult = SemanticTokensPartialResult;

pub type RegistrationOptions = SemanticTokensRegistrationOptions;

pub type Options = SemanticTokensOptions;

pub type WorkspaceClientCapabilities = SemanticTokensWorkspaceClientCapabilities;

pub type ClientCapabilities = SemanticTokensClientCapabilities;