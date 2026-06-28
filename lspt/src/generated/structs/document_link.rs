// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A document link is a range in a text document that links to an internal or external resource, like another
/// text document or a web site.
pub struct DocumentLink {
    /// The range this link applies to.
    pub range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The uri this link points to. If missing a resolve request is sent later.
    pub target: Option<Uri>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The tooltip text when you hover over this link.
    ///
    /// If a tooltip is provided, is will be displayed in a string that includes instructions on how to
    /// trigger the link, such as `{0} (ctrl + click)`. The specific instructions vary depending on OS,
    /// user settings, and localization.
    ///
    /// @since 3.15.0
    pub tooltip: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on a document link between a
    /// DocumentLinkRequest and a DocumentLinkResolveRequest.
    pub data: Option<serde_json::Value>,
}


mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The parameters of a {@link DocumentLinkRequest}.
    pub struct DocumentLinkParams {
        /// The document to provide document links for.
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
    /// Registration options for a {@link DocumentLinkRequest}.
    pub struct DocumentLinkRegistrationOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// A document selector to identify the scope of the registration. If set to null
        /// the document selector provided on the client side will be used.
        pub document_selector: Option<DocumentSelector>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Document links have a resolve provider as well.
        pub resolve_provider: Option<bool>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Provider options for a {@link DocumentLinkRequest}.
    pub struct DocumentLinkOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Document links have a resolve provider as well.
        pub resolve_provider: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub work_done_progress: Option<bool>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The client capabilities of a {@link DocumentLinkRequest}.
    pub struct DocumentLinkClientCapabilities {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether document link supports dynamic registration.
        pub dynamic_registration: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether the client supports the `tooltip` property on `DocumentLink`.
        ///
        /// @since 3.15.0
        pub tooltip_support: Option<bool>,
    }
}

pub type Params = raw::DocumentLinkParams;

pub type RegistrationOptions = raw::DocumentLinkRegistrationOptions;

pub type Options = raw::DocumentLinkOptions;

pub type ClientCapabilities = raw::DocumentLinkClientCapabilities;