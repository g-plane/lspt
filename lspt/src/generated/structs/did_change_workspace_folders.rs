// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a `workspace/didChangeWorkspaceFolders` notification.
pub struct DidChangeWorkspaceFoldersParams {
    /// The actual workspace folder change event.
    pub event: WorkspaceFoldersChangeEvent,
}
