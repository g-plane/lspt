// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistrationParams {
    pub registrations: Vec<Registration>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// General parameters to register for a notification or to register a provider.
pub struct Registration {
    /// The id used to register the request. The id can be used to deregister
    /// the request again.
    pub id: String,

    /// The method / capability to register for.
    pub method: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Options necessary for the registration.
    pub register_options: Option<serde_json::Value>,
}

pub type Params = RegistrationParams;