// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Additional data about a workspace edit.
///
/// @since 3.18.0
/// @proposed
pub struct WorkspaceEditMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Signal to the editor that this edit is a refactoring.
    pub is_refactoring: Option<bool>,
}


mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The parameters passed via an apply workspace edit request.
    pub struct ApplyWorkspaceEditParams {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// An optional label of the workspace edit. This label is
        /// presented in the user interface for example on an undo
        /// stack to undo the workspace edit.
        pub label: Option<String>,

        /// The edits to apply.
        pub edit: WorkspaceEdit,

        #[cfg(feature = "proposed")]
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Additional data about the edit.
        ///
        /// @since 3.18.0
        /// @proposed
        pub metadata: Option<WorkspaceEditMetadata>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The result returned from the apply workspace edit request.
    ///
    /// @since 3.17 renamed from ApplyWorkspaceEditResponse
    pub struct ApplyWorkspaceEditResult {
        /// Indicates whether the edit was applied or not.
        pub applied: bool,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// An optional textual description for why the edit was not applied.
        /// This may be used by the server for diagnostic logging or to provide
        /// a suitable error for a request that triggered the edit.
        pub failure_reason: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Depending on the client's failure handling strategy `failedChange` might
        /// contain the index of the change that failed. This property is only available
        /// if the client signals a `failureHandlingStrategy` in its client capabilities.
        pub failed_change: Option<u32>,
    }
}

pub type Params = raw::ApplyWorkspaceEditParams;

pub type Result = raw::ApplyWorkspaceEditResult;