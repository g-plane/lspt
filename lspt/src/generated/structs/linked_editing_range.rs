// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The result of a linked editing range request.
///
/// @since 3.16.0
pub struct LinkedEditingRanges {
    /// A list of ranges that can be edited together. The ranges must have
    /// identical length and contain identical text content. The ranges cannot overlap.
    pub ranges: Vec<Range>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional word pattern (regular expression) that describes valid contents for
    /// the given ranges. If no pattern is provided, the client configuration's word
    /// pattern will be used.
    pub word_pattern: Option<String>,
}


mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LinkedEditingRangeParams {
        /// The text document.
        pub text_document: TextDocumentIdentifier,

        /// The position inside the text document.
        pub position: Position,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// An optional token that a server can use to report work done progress.
        pub work_done_token: Option<ProgressToken>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LinkedEditingRangeRegistrationOptions {
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
    pub struct LinkedEditingRangeOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub work_done_progress: Option<bool>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Client capabilities for the linked editing range request.
    ///
    /// @since 3.16.0
    pub struct LinkedEditingRangeClientCapabilities {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether implementation supports dynamic registration. If this is set to `true`
        /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
        /// return value for the corresponding server capability as well.
        pub dynamic_registration: Option<bool>,
    }
}

pub type Params = raw::LinkedEditingRangeParams;

pub type RegistrationOptions = raw::LinkedEditingRangeRegistrationOptions;

pub type Options = raw::LinkedEditingRangeOptions;

pub type ClientCapabilities = raw::LinkedEditingRangeClientCapabilities;