// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnregistrationParams {
    pub unregisterations: Vec<Unregistration>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// General parameters to unregister a request or notification.
pub struct Unregistration {
    /// The id used to unregister the request or notification. Usually an id
    /// provided during the register request.
    pub id: String,

    /// The method to unregister for.
    pub method: String,
}
