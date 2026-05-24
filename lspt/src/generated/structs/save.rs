// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Save options.
pub struct SaveOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client is supposed to include the content on save.
    pub include_text: Option<bool>,
}

pub type Options = SaveOptions;