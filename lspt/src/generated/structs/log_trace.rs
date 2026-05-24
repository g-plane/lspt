// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogTraceParams {
    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbose: Option<String>,
}

pub type Params = LogTraceParams;