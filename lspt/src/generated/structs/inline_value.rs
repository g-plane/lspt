// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A parameter literal used in inline value requests.
///
/// @since 3.17.0
pub struct InlineValueParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The document range for which inline values should be computed.
    pub range: Range,

    /// Additional information about the context in which inline values were
    /// requested.
    pub context: InlineValueContext,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inline value options used during static or dynamic registration.
///
/// @since 3.17.0
pub struct InlineValueRegistrationOptions {
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
/// @since 3.17.0
pub struct InlineValueContext {
    /// The stack frame (as a DAP Id) where the execution has stopped.
    pub frame_id: i32,

    /// The document range where execution has stopped.
    /// Typically the end position of the range denotes the line where the inline values are shown.
    pub stopped_location: Range,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provide inline value as text.
///
/// @since 3.17.0
pub struct InlineValueText {
    /// The document range for which the inline value applies.
    pub range: Range,

    /// The text of the inline value.
    pub text: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provide inline value through a variable lookup.
/// If only a range is specified, the variable name will be extracted from the underlying document.
/// An optional variable name can be used to override the extracted name.
///
/// @since 3.17.0
pub struct InlineValueVariableLookup {
    /// The document range for which the inline value applies.
    /// The range is used to extract the variable name from the underlying document.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// If specified the name of the variable to look up.
    pub variable_name: Option<String>,

    /// How to perform the lookup.
    pub case_sensitive_lookup: bool,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provide an inline value through an expression evaluation.
/// If only a range is specified, the expression will be extracted from the underlying document.
/// An optional expression can be used to override the extracted expression.
///
/// @since 3.17.0
pub struct InlineValueEvaluatableExpression {
    /// The document range for which the inline value applies.
    /// The range is used to extract the evaluatable expression from the underlying document.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// If specified the expression overrides the extracted expression.
    pub expression: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inline value options used during static registration.
///
/// @since 3.17.0
pub struct InlineValueOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client workspace capabilities specific to inline values.
///
/// @since 3.17.0
pub struct InlineValueWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from the
    /// server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// inline values currently shown. It should be used with absolute care and is
    /// useful for situation where a server for example detects a project wide
    /// change that requires such a calculation.
    pub refresh_support: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities specific to inline values.
///
/// @since 3.17.0
pub struct InlineValueClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration for inline value providers.
    pub dynamic_registration: Option<bool>,
}

pub type Params = InlineValueParams;

pub type RegistrationOptions = InlineValueRegistrationOptions;

pub type Options = InlineValueOptions;

pub type WorkspaceClientCapabilities = InlineValueWorkspaceClientCapabilities;

pub type ClientCapabilities = InlineValueClientCapabilities;