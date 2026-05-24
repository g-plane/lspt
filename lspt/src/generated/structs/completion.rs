// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Completion parameters
pub struct CompletionParams {
    /// The text document.
    pub text_document: TextDocumentIdentifier,

    /// The position inside the text document.
    pub position: Position,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The completion context. This is only available it the client specifies
    /// to send this using the client capability `textDocument.completion.contextSupport === true`
    pub context: Option<CompletionContext>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report work done progress.
    pub work_done_token: Option<ProgressToken>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional token that a server can use to report partial results (e.g. streaming) to
    /// the client.
    pub partial_result_token: Option<ProgressToken>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a collection of {@link CompletionItem completion items} to be presented
/// in the editor.
pub struct CompletionList {
    /// This list it not complete. Further typing results in recomputing this list.
    ///
    /// Recomputed lists have all their items replaced (not appended) in the
    /// incomplete completion sessions.
    pub is_incomplete: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// In many cases the items of an actual completion result share the same
    /// value for properties like `commitCharacters` or the range of a text
    /// edit. A completion list can therefore define item defaults which will
    /// be used if a completion item itself doesn't specify the value.
    ///
    /// If a completion list specifies a default value and a completion item
    /// also specifies a corresponding value, the rules for combining these are
    /// defined by `applyKinds` (if the client supports it), defaulting to
    /// ApplyKind.Replace.
    ///
    /// Servers are only allowed to return default values if the client
    /// signals support for this via the `completionList.itemDefaults`
    /// capability.
    ///
    /// @since 3.17.0
    pub item_defaults: Option<CompletionItemDefaults>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies how fields from a completion item should be combined with those
    /// from `completionList.itemDefaults`.
    ///
    /// If unspecified, all fields will be treated as ApplyKind.Replace.
    ///
    /// If a field's value is ApplyKind.Replace, the value from a completion item
    /// (if provided and not `null`) will always be used instead of the value
    /// from `completionItem.itemDefaults`.
    ///
    /// If a field's value is ApplyKind.Merge, the values will be merged using
    /// the rules defined against each field below.
    ///
    /// Servers are only allowed to return `applyKind` if the client
    /// signals support for this via the `completionList.applyKindSupport`
    /// capability.
    ///
    /// @since 3.18.0
    pub apply_kind: Option<CompletionItemApplyKinds>,

    /// The completion items.
    pub items: Vec<CompletionItem>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link CompletionRequest}.
pub struct CompletionRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Most tools trigger completion request automatically without explicitly requesting
    /// it using a keyboard shortcut (e.g. Ctrl+Space). Typically they do so when the user
    /// starts to type an identifier. For example if the user types `c` in a JavaScript file
    /// code complete will automatically pop up present `console` besides others as a
    /// completion item. Characters that make up identifiers don't need to be listed here.
    ///
    /// If code complete should automatically be trigger on characters not being valid inside
    /// an identifier (for example `.` in JavaScript) list them in `triggerCharacters`.
    pub trigger_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The list of all possible characters that commit a completion. This field can be used
    /// if clients don't support individual commit characters per completion item. See
    /// `ClientCapabilities.textDocument.completion.completionItem.commitCharactersSupport`
    ///
    /// If a server provides both `allCommitCharacters` and commit characters on an individual
    /// completion item the ones on the completion item win.
    ///
    /// @since 3.2.0
    pub all_commit_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a completion item.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server supports the following `CompletionItem` specific
    /// capabilities.
    ///
    /// @since 3.17.0
    pub completion_item: Option<ServerCompletionItemOptions>,
}


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Contains additional information about the context in which a completion request is triggered.
pub struct CompletionContext {
    /// How the completion was triggered.
    pub trigger_kind: CompletionTriggerKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The trigger character (a single character) that has trigger code complete.
    /// Is undefined if `triggerKind !== CompletionTriggerKind.TriggerCharacter`
    pub trigger_character: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Completion options.
pub struct CompletionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Most tools trigger completion request automatically without explicitly requesting
    /// it using a keyboard shortcut (e.g. Ctrl+Space). Typically they do so when the user
    /// starts to type an identifier. For example if the user types `c` in a JavaScript file
    /// code complete will automatically pop up present `console` besides others as a
    /// completion item. Characters that make up identifiers don't need to be listed here.
    ///
    /// If code complete should automatically be trigger on characters not being valid inside
    /// an identifier (for example `.` in JavaScript) list them in `triggerCharacters`.
    pub trigger_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The list of all possible characters that commit a completion. This field can be used
    /// if clients don't support individual commit characters per completion item. See
    /// `ClientCapabilities.textDocument.completion.completionItem.commitCharactersSupport`
    ///
    /// If a server provides both `allCommitCharacters` and commit characters on an individual
    /// completion item the ones on the completion item win.
    ///
    /// @since 3.2.0
    pub all_commit_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a completion item.
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server supports the following `CompletionItem` specific
    /// capabilities.
    ///
    /// @since 3.17.0
    pub completion_item: Option<ServerCompletionItemOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Completion client capabilities
pub struct CompletionClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether completion supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the following `CompletionItem` specific
    /// capabilities.
    pub completion_item: Option<ClientCompletionItemOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_item_kind: Option<ClientCompletionItemOptionsKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Defines how the client handles whitespace and indentation
    /// when accepting a completion item that uses multi line
    /// text in either `insertText` or `textEdit`.
    ///
    /// @since 3.17.0
    pub insert_text_mode: Option<InsertTextMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports to send additional context information for a
    /// `textDocument/completion` request.
    pub context_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the following `CompletionList` specific
    /// capabilities.
    ///
    /// @since 3.17.0
    pub completion_list: Option<CompletionListCapabilities>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The client supports the following `CompletionList` specific
/// capabilities.
///
/// @since 3.17.0
pub struct CompletionListCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the following itemDefaults on
    /// a completion list.
    ///
    /// The value lists the supported property names of the
    /// `CompletionList.itemDefaults` object. If omitted
    /// no properties are supported.
    ///
    /// @since 3.17.0
    pub item_defaults: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies whether the client supports `CompletionList.applyKind` to
    /// indicate how supported values from `completionList.itemDefaults`
    /// and `completion` will be combined.
    ///
    /// If a client supports `applyKind` it must support it for all fields
    /// that it supports that are listed in `CompletionList.applyKind`. This
    /// means when clients add support for new/future fields in completion
    /// items the MUST also support merge for them if those fields are
    /// defined in `CompletionList.applyKind`.
    ///
    /// @since 3.18.0
    pub apply_kind_support: Option<bool>,
}
