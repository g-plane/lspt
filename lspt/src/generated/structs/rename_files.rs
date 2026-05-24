// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters sent in notifications/requests for user-initiated renames of
/// files.
///
/// @since 3.16.0
pub struct RenameFilesParams {
    /// An array of all files/folders renamed in this operation. When a folder is renamed, only
    /// the folder will be included, and not its children.
    pub files: Vec<FileRename>,
}
