// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageActionItem {
    /// A short title like 'Retry', 'Open Log' etc.
    pub title: String,
}


mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ShowMessageRequestParams {
        #[serde(rename = "type")]
        /// The message type. See {@link MessageType}
        pub ty: MessageType,

        /// The actual message.
        pub message: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The message action items to present.
        pub actions: Option<Vec<MessageActionItem>>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Show message request client capabilities
    pub struct ShowMessageRequestClientCapabilities {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Capabilities specific to the `MessageActionItem` type.
        pub message_action_item: Option<ClientShowMessageActionItemOptions>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// @since 3.18.0
    pub struct ClientShowMessageActionItemOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether the client supports additional attributes which
        /// are preserved and send back to the server in the
        /// request's response.
        pub additional_properties_support: Option<bool>,
    }
}

pub type Params = raw::ShowMessageRequestParams;

pub type ClientCapabilities = raw::ShowMessageRequestClientCapabilities;

pub type Options = raw::ClientShowMessageActionItemOptions;