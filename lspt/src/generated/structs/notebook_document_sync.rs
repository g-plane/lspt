// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options specific to a notebook.
///
/// @since 3.17.0
pub struct NotebookDocumentSyncRegistrationOptions {
    /// The notebooks to be synced
    pub notebook_selector: Vec<NotebookSelectorItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether save notification should be forwarded to
    /// the server. Will only be honored if mode === `notebook`.
    pub save: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Options specific to a notebook plus its cells
/// to be synced to the server.
///
/// If a selector provides a notebook document
/// filter but no cell selector all cells of a
/// matching notebook document will be synced.
///
/// If a selector provides no notebook document
/// filter but only a cell selector all notebook
/// document that contain at least one matching
/// cell will be synced.
///
/// @since 3.17.0
pub struct NotebookDocumentSyncOptions {
    /// The notebooks to be synced
    pub notebook_selector: Vec<NotebookSelectorItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether save notification should be forwarded to
    /// the server. Will only be honored if mode === `notebook`.
    pub save: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Notebook specific client capabilities.
///
/// @since 3.17.0
pub struct NotebookDocumentSyncClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is
    /// set to `true` the client supports the new
    /// `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports sending execution summary data per cell.
    pub execution_summary_support: Option<bool>,
}
