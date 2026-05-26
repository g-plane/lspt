// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a color range from a document.
pub struct ColorInformation {
    /// The range in the document where this color appears.
    pub range: Range,

    /// The actual color value for this color range.
    pub color: Color,
}


mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Parameters for a {@link DocumentColorRequest}.
    pub struct DocumentColorParams {
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
    pub struct DocumentColorRegistrationOptions {
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
    pub struct DocumentColorOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub work_done_progress: Option<bool>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct DocumentColorClientCapabilities {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether implementation supports dynamic registration. If this is set to `true`
        /// the client supports the new `DocumentColorRegistrationOptions` return value
        /// for the corresponding server capability as well.
        pub dynamic_registration: Option<bool>,
    }
}

pub type Params = raw::DocumentColorParams;

pub type RegistrationOptions = raw::DocumentColorRegistrationOptions;

pub type Options = raw::DocumentColorOptions;

pub type ClientCapabilities = raw::DocumentColorClientCapabilities;