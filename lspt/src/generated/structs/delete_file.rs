// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Delete file operation
pub struct DeleteFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    pub annotation_id: Option<ChangeAnnotationIdentifier>,

    /// A delete
    pub kind: String,

    /// The file to delete.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Delete options.
    pub options: Option<DeleteFileOptions>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Delete file options
pub struct DeleteFileOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Delete the content recursively if a folder is denoted.
    pub recursive: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ignore the operation if the file doesn't exist.
    pub ignore_if_not_exists: Option<bool>,
}

pub type Options = DeleteFileOptions;