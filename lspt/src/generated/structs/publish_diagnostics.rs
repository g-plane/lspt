// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The publish diagnostic notification's parameters.
    pub struct PublishDiagnosticsParams {
        /// The URI for which diagnostic information is reported.
        pub uri: Uri,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Optional the version number of the document the diagnostics are published for.
        ///
        /// @since 3.15.0
        pub version: Option<i32>,

        /// An array of diagnostic information items.
        pub diagnostics: Vec<Diagnostic>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The publish diagnostic client capabilities.
    pub struct PublishDiagnosticsClientCapabilities {
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
        /// Whether the client interprets the version property of the
        /// `textDocument/publishDiagnostics` notification's parameter.
        ///
        /// @since 3.15.0
        pub version_support: Option<bool>,
    }
}

pub type Params = raw::PublishDiagnosticsParams;

pub type ClientCapabilities = raw::PublishDiagnosticsClientCapabilities;