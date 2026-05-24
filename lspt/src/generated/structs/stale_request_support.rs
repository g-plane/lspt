// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct StaleRequestSupportOptions {
    /// The client will actively cancel the request.
    pub cancel: bool,

    /// The list of requests for which the client
    /// will retry the request if it receives a
    /// response with error code `ContentModified`
    pub retry_on_content_modified: Vec<String>,
}

pub type Options = StaleRequestSupportOptions;