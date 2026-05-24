// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Params to show a resource in the UI.
///
/// @since 3.16.0
pub struct ShowDocumentParams {
    /// The uri to show.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates to show the resource in an external program.
    /// To show, for example, `https://code.visualstudio.com/`
    /// in the default WEB browser set `external` to `true`.
    pub external: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional property to indicate whether the editor
    /// showing the document should take focus or not.
    /// Clients might ignore this property if an external
    /// program is started.
    pub take_focus: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional selection range if the document is a text
    /// document. Clients might ignore the property if an
    /// external program is started or the file is not a text
    /// file.
    pub selection: Option<Range>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result of a showDocument request.
///
/// @since 3.16.0
pub struct ShowDocumentResult {
    /// A boolean indicating if the show was successful.
    pub success: bool,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities for the showDocument request.
///
/// @since 3.16.0
pub struct ShowDocumentClientCapabilities {
    /// The client has support for the showDocument
    /// request.
    pub support: bool,
}

pub type Params = ShowDocumentParams;

pub type Result = ShowDocumentResult;

pub type ClientCapabilities = ShowDocumentClientCapabilities;