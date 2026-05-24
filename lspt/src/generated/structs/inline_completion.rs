// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A parameter literal used in inline completion requests.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    /// Additional information about the context in which inline completions were
    /// requested.
    pub context: InlineCompletionContext,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a collection of {@link InlineCompletionItem inline completion items} to be presented in the editor.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionList {
    /// The inline completion items
    pub items: Vec<InlineCompletionItem>,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// An inline completion item represents a text snippet that is proposed inline to complete text that is being typed.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionItem {
    /// The text to replace the range with. Must be set.
    pub insert_text: InlineCompletionItemInsertText,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A text that is used to decide if this inline completion should be shown. When `falsy` the {@link InlineCompletionItem.insertText} is used.
    pub filter_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The range to replace. Must begin and end on the same line.
    pub range: Option<Range>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional {@link Command} that is executed *after* inserting this completion.
    pub command: Option<Command>,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inline completion options used during static or dynamic registration.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The id used to register the request. The id can be used to deregister
    /// the request again. See also Registration#id.
    pub id: Option<String>,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provides information about the context in which an inline completion was requested.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionContext {
    /// Describes how the inline completion was triggered.
    pub trigger_kind: InlineCompletionTriggerKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Provides information about the currently selected item in the autocomplete widget if it is visible.
    pub selected_completion_info: Option<SelectedCompletionInfo>,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Inline completion options used during static registration.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Describes the currently selected completion item.
///
/// @since 3.18.0
/// @proposed
pub struct SelectedCompletionInfo {
    /// The range that will be replaced if this completion item is accepted.
    pub range: Range,

    /// The text the range will be replaced with if this completion is accepted.
    pub text: String,
}


#[cfg(feature = "proposed")]
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities specific to inline completions.
///
/// @since 3.18.0
/// @proposed
pub struct InlineCompletionClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration for inline completion providers.
    pub dynamic_registration: Option<bool>,
}

#[cfg(feature = "proposed")]
pub type Params = InlineCompletionParams;

#[cfg(feature = "proposed")]
pub type List = InlineCompletionList;

#[cfg(feature = "proposed")]
pub type Item = InlineCompletionItem;

#[cfg(feature = "proposed")]
pub type RegistrationOptions = InlineCompletionRegistrationOptions;

#[cfg(feature = "proposed")]
pub type Context = InlineCompletionContext;

#[cfg(feature = "proposed")]
pub type Options = InlineCompletionOptions;

#[cfg(feature = "proposed")]
pub type ClientCapabilities = InlineCompletionClientCapabilities;