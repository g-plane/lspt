// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressBegin {
    pub kind: String,

    /// Mandatory title of the progress operation. Used to briefly inform about
    /// the kind of operation being performed.
    ///
    /// Examples: "Indexing" or "Linking dependencies".
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Controls if a cancel button should show to allow the user to cancel the
    /// long running operation. Clients that don't support cancellation are allowed
    /// to ignore the setting.
    pub cancellable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional, more detailed associated progress message. Contains
    /// complementary information to the `title`.
    ///
    /// Examples: "3/25 files", "project/src/module2", "node_modules/some_dep".
    /// If unset, the previous progress message (if any) is still valid.
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional progress percentage to display (value 100 is considered 100%).
    /// If not provided infinite progress is assumed and clients are allowed
    /// to ignore the `percentage` value in subsequent in report notifications.
    ///
    /// The value should be steadily rising. Clients are free to ignore values
    /// that are not following this rule. The value range is [0, 100].
    pub percentage: Option<u32>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressReport {
    pub kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Controls enablement state of a cancel button.
    ///
    /// Clients that don't support cancellation or don't support controlling the button's
    /// enablement state are allowed to ignore the property.
    pub cancellable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional, more detailed associated progress message. Contains
    /// complementary information to the `title`.
    ///
    /// Examples: "3/25 files", "project/src/module2", "node_modules/some_dep".
    /// If unset, the previous progress message (if any) is still valid.
    pub message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional progress percentage to display (value 100 is considered 100%).
    /// If not provided infinite progress is assumed and clients are allowed
    /// to ignore the `percentage` value in subsequent in report notifications.
    ///
    /// The value should be steadily rising. Clients are free to ignore values
    /// that are not following this rule. The value range is [0, 100]
    pub percentage: Option<u32>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressEnd {
    pub kind: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Optional, a final message indicating to for example indicate the outcome
    /// of the operation.
    pub message: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkDoneProgressParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}
