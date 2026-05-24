// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in notifications/requests for user-initiated creation of
/// files.
///
/// @since 3.16.0
pub struct CreateFilesParams {
    /// An array of all files/folders created in this operation.
    pub files: Vec<FileCreate>,
}
