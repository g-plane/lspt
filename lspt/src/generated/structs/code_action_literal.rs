// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCodeActionLiteralOptions {
    /// The code action kind is support with the following value
    /// set.
    pub code_action_kind: ClientCodeActionKindOptions,
}
