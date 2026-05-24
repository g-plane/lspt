// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Rename file operation
pub struct RenameFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    pub annotation_id: Option<ChangeAnnotationIdentifier>,

    /// A rename
    pub kind: String,

    /// The old (existing) location.
    pub old_uri: Uri,

    /// The new location.
    pub new_uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Rename options.
    pub options: Option<RenameFileOptions>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Rename file options
pub struct RenameFileOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Overwrite target if existing. Overwrite wins over `ignoreIfExists`
    pub overwrite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ignores if target exists.
    pub ignore_if_exists: Option<bool>,
}

pub type Options = RenameFileOptions;