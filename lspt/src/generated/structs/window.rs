// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// It indicates whether the client supports server initiated
    /// progress using the `window/workDoneProgress/create` request.
    ///
    /// The capability also controls Whether client supports handling
    /// of progress notifications. If set servers are allowed to report a
    /// `workDoneProgress` property in the request specific server
    /// capabilities.
    ///
    /// @since 3.15.0
    pub work_done_progress: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the showMessage request.
    ///
    /// @since 3.16.0
    pub show_message: Option<ShowMessageRequestClientCapabilities>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Capabilities specific to the showDocument request.
    ///
    /// @since 3.16.0
    pub show_document: Option<ShowDocumentClientCapabilities>,
}
