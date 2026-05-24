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
    /// The parameters of a {@link DocumentFormattingRequest}.
    pub struct DocumentFormattingParams {
        /// The document to format.
        pub text_document: TextDocumentIdentifier,

        /// The format options.
        pub options: FormattingOptions,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// An optional token that a server can use to report work done progress.
        pub work_done_token: Option<ProgressToken>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Registration options for a {@link DocumentFormattingRequest}.
    pub struct DocumentFormattingRegistrationOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// A document selector to identify the scope of the registration. If set to null
        /// the document selector provided on the client side will be used.
        pub document_selector: Option<DocumentSelector>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Provider options for a {@link DocumentFormattingRequest}.
    pub struct DocumentFormattingOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub work_done_progress: Option<bool>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Client capabilities of a {@link DocumentFormattingRequest}.
    pub struct DocumentFormattingClientCapabilities {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether formatting supports dynamic registration.
        pub dynamic_registration: Option<bool>,
    }
}

pub type Params = raw::DocumentFormattingParams;

pub type RegistrationOptions = raw::DocumentFormattingRegistrationOptions;

pub type Options = raw::DocumentFormattingOptions;

pub type ClientCapabilities = raw::DocumentFormattingClientCapabilities;