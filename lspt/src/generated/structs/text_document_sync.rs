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

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TextDocumentSyncOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Open and close notifications are sent to the server. If omitted open close notification should not
        /// be sent.
        pub open_close: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Change notifications are sent to the server. See TextDocumentSyncKind.None, TextDocumentSyncKind.Full
        /// and TextDocumentSyncKind.Incremental. If omitted it defaults to TextDocumentSyncKind.None.
        pub change: Option<TextDocumentSyncKind>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// If present will save notifications are sent to the server. If omitted the notification should not be
        /// sent.
        pub will_save: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// If present will save wait until requests are sent to the server. If omitted the request should not be
        /// sent.
        pub will_save_wait_until: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// If present save notifications are sent to the server. If omitted the notification should not be
        /// sent.
        pub save: Option<TextDocumentSyncSave>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TextDocumentSyncClientCapabilities {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether text document synchronization supports dynamic registration.
        pub dynamic_registration: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client supports sending will save notifications.
        pub will_save: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client supports sending a will save request and
        /// waits for a response providing text edits which will
        /// be applied to the document before it is saved.
        pub will_save_wait_until: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client supports did save notifications.
        pub did_save: Option<bool>,
    }
}

pub type Options = raw::TextDocumentSyncOptions;

pub type ClientCapabilities = raw::TextDocumentSyncClientCapabilities;