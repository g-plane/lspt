// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in notifications/requests for user-initiated deletes of
/// files.
///
/// @since 3.16.0
pub struct DeleteFilesParams {
    /// An array of all files/folders deleted in this operation.
    pub files: Vec<FileDelete>,
}

pub type Params = DeleteFilesParams;