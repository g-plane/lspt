// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The log message parameters.
pub struct LogMessageParams {
    #[serde(rename = "type")]
    /// The message type. See {@link MessageType}
    pub ty: MessageType,

    /// The actual message.
    pub message: String,
}
