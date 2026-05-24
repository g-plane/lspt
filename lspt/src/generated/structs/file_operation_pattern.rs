// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A pattern to describe in which file operation requests or notifications
/// the server is interested in receiving.
///
/// @since 3.16.0
pub struct FileOperationPattern {
    /// The glob pattern to match. Glob patterns can have the following syntax:
    /// - `*` to match one or more characters in a path segment
    /// - `?` to match on one character in a path segment
    /// - `**` to match any number of path segments, including none
    /// - `{}` to group sub patterns into an OR expression. (e.g. `**​/*.{ts,js}` matches all TypeScript and JavaScript files)
    /// - `[]` to declare a range of characters to match in a path segment (e.g., `example.[0-9]` to match on `example.0`, `example.1`, …)
    /// - `[!...]` to negate a range of characters to match in a path segment (e.g., `example.[!0-9]` to match on `example.a`, `example.b`, but not `example.0`)
    pub glob: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether to match files or folders with this pattern.
    ///
    /// Matches both if undefined.
    pub matches: Option<FileOperationPatternKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional options used during matching.
    pub options: Option<FileOperationPatternOptions>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Matching options for the file operation pattern.
///
/// @since 3.16.0
pub struct FileOperationPatternOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The pattern should be matched ignoring casing.
    pub ignore_case: Option<bool>,
}

pub type Options = FileOperationPatternOptions;