// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Documentation for a class of code actions.
///
/// @since 3.18.0
/// @proposed
pub struct CodeActionKindDocumentation {
    /// The kind of the code action being documented.
    ///
    /// If the kind is generic, such as `CodeActionKind.Refactor`, the documentation will be shown whenever any
    /// refactorings are returned. If the kind if more specific, such as `CodeActionKind.RefactorExtract`, the
    /// documentation will only be shown when extract refactoring code actions are returned.
    pub kind: CodeActionKind,

    /// Command that is ued to display the documentation to the user.
    ///
    /// The title of this documentation code action is taken from {@linkcode Command.title}
    pub command: Command,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCodeActionKindOptions {
    /// The code action kind values the client supports. When this
    /// property exists the client also guarantees that it will
    /// handle values outside its set gracefully and falls back
    /// to a default value when unknown.
    pub value_set: Vec<CodeActionKind>,
}
