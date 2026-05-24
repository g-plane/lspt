// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Value-object describing what options formatting should use.
pub struct FormattingOptions {
    /// Size of a tab in spaces.
    pub tab_size: u32,

    /// Prefer spaces over tabs.
    pub insert_spaces: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Trim trailing whitespace on a line.
    ///
    /// @since 3.15.0
    pub trim_trailing_whitespace: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Insert a newline character at the end of the file if one does not exist.
    ///
    /// @since 3.15.0
    pub insert_final_newline: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Trim all newlines after the final newline at the end of the file.
    ///
    /// @since 3.15.0
    pub trim_final_newlines: Option<bool>,
}
