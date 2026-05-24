// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Create file operation.
pub struct CreateFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional annotation identifier describing the operation.
    ///
    /// @since 3.16.0
    pub annotation_id: Option<ChangeAnnotationIdentifier>,

    /// A create
    pub kind: String,

    /// The resource to create.
    pub uri: Uri,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional options
    pub options: Option<CreateFileOptions>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Options to create a file.
pub struct CreateFileOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Overwrite existing file. Overwrite wins over `ignoreIfExists`
    pub overwrite: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Ignore if exists.
    pub ignore_if_exists: Option<bool>,
}

pub type Options = CreateFileOptions;