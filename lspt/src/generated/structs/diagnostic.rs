// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Cancellation data returned from a diagnostic request.
///
/// @since 3.17.0
pub struct DiagnosticServerCancellationData {
    pub retrigger_request: bool,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Diagnostic registration options.
///
/// @since 3.17.0
pub struct DiagnosticRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional identifier under which the diagnostics are
    /// managed by the client.
    pub identifier: Option<String>,

    /// Whether the language has inter file dependencies meaning that
    /// editing code in one file can result in a different diagnostic
    /// set in another file. Inter file dependencies are common for
    /// most programming languages and typically uncommon for linters.
    pub inter_file_dependencies: bool,

    /// The server provides support for workspace diagnostics as well.
    pub workspace_diagnostics: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Diagnostic options.
///
/// @since 3.17.0
pub struct DiagnosticOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional identifier under which the diagnostics are
    /// managed by the client.
    pub identifier: Option<String>,

    /// Whether the language has inter file dependencies meaning that
    /// editing code in one file can result in a different diagnostic
    /// set in another file. Inter file dependencies are common for
    /// most programming languages and typically uncommon for linters.
    pub inter_file_dependencies: bool,

    /// The server provides support for workspace diagnostics as well.
    pub workspace_diagnostics: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a diagnostic, such as a compiler error or warning. Diagnostic objects
/// are only valid in the scope of a resource.
pub struct Diagnostic {
    /// The range at which the message applies
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostic's severity. To avoid interpretation mismatches when a
    /// server is used with different clients it is highly recommended that servers
    /// always provide a severity value.
    pub severity: Option<DiagnosticSeverity>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostic's code, which usually appear in the user interface.
    pub code: Option<DiagnosticCode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional property to describe the error code.
    /// Requires the code field (above) to be present/not null.
    ///
    /// @since 3.16.0
    pub code_description: Option<CodeDescription>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string describing the source of this
    /// diagnostic, e.g. 'typescript' or 'super lint'. It usually
    /// appears in the user interface.
    pub source: Option<String>,

    /// The diagnostic's message. It usually appears in the user interface
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional metadata about the diagnostic.
    ///
    /// @since 3.15.0
    pub tags: Option<Vec<DiagnosticTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An array of related diagnostic information, e.g. when symbol-names within
    /// a scope collide all definitions can be marked via this property.
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved between a `textDocument/publishDiagnostics`
    /// notification and `textDocument/codeAction` request.
    ///
    /// @since 3.16.0
    pub data: Option<serde_json::Value>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a related message and source code location for a diagnostic. This should be
/// used to point to code locations that cause or related to a diagnostics, e.g when duplicating
/// a symbol in a scope.
pub struct DiagnosticRelatedInformation {
    /// The location of this related diagnostic information.
    pub location: Location,

    /// The message of this related diagnostic information.
    pub message: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Workspace client capabilities specific to diagnostic pull requests.
///
/// @since 3.17.0
pub struct DiagnosticWorkspaceClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client implementation supports a refresh request sent from
    /// the server to the client.
    ///
    /// Note that this event is global and will force the client to refresh all
    /// pulled diagnostics currently shown. It should be used with absolute care and
    /// is useful for situation where a server for example detects a project wide
    /// change that requires such a calculation.
    pub refresh_support: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities specific to diagnostic pull requests.
///
/// @since 3.17.0
pub struct DiagnosticClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the clients accepts diagnostics with related information.
    pub related_information: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the tag property to provide meta data about a diagnostic.
    /// Clients supporting tags have to handle unknown tags gracefully.
    ///
    /// @since 3.15.0
    pub tag_support: Option<ClientDiagnosticsTagOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports a codeDescription property
    ///
    /// @since 3.16.0
    pub code_description_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports the `data` property which is
    /// preserved between a `textDocument/publishDiagnostics` and
    /// `textDocument/codeAction` request.
    ///
    /// @since 3.16.0
    pub data_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the clients supports related documents for document diagnostic pulls.
    pub related_document_support: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// General diagnostics capabilities for pull and push model.
pub struct DiagnosticsCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the clients accepts diagnostics with related information.
    pub related_information: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the tag property to provide meta data about a diagnostic.
    /// Clients supporting tags have to handle unknown tags gracefully.
    ///
    /// @since 3.15.0
    pub tag_support: Option<ClientDiagnosticsTagOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports a codeDescription property
    ///
    /// @since 3.16.0
    pub code_description_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports the `data` property which is
    /// preserved between a `textDocument/publishDiagnostics` and
    /// `textDocument/codeAction` request.
    ///
    /// @since 3.16.0
    pub data_support: Option<bool>,
}

pub type RegistrationOptions = DiagnosticRegistrationOptions;

pub type Options = DiagnosticOptions;

pub type WorkspaceClientCapabilities = DiagnosticWorkspaceClientCapabilities;

pub type ClientCapabilities = DiagnosticClientCapabilities;