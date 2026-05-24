// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// A completion item represents a text snippet that is
/// proposed to complete text that is being typed.
pub struct CompletionItem {
    /// The label of this completion item.
    ///
    /// The label property is also by default the text that
    /// is inserted when selecting this completion.
    ///
    /// If label details are provided the label itself should
    /// be an unqualified name of the completion item.
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Additional details for the label
    ///
    /// @since 3.17.0
    pub label_details: Option<CompletionItemLabelDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The kind of this completion item. Based of the kind
    /// an icon is chosen by the editor.
    pub kind: Option<CompletionItemKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this completion item.
    ///
    /// @since 3.15.0
    pub tags: Option<Vec<CompletionItemTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string with additional information
    /// about this item, like type or symbol information.
    pub detail: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A human-readable string that represents a doc-comment.
    pub documentation: Option<CompletionItemDocumentation>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Select this item when showing.
    ///
    /// *Note* that only one completion item can be selected and that the
    /// tool / client decides which item that is. The rule is that the *first*
    /// item of those that match best is selected.
    pub preselect: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A string that should be used when comparing this item
    /// with other items. When `falsy` the {@link CompletionItem.label label}
    /// is used.
    pub sort_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A string that should be used when filtering a set of
    /// completion items. When `falsy` the {@link CompletionItem.label label}
    /// is used.
    pub filter_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A string that should be inserted into a document when selecting
    /// this completion. When `falsy` the {@link CompletionItem.label label}
    /// is used.
    ///
    /// The `insertText` is subject to interpretation by the client side.
    /// Some tools might not take the string literally. For example
    /// VS Code when code complete is requested in this example
    /// `con<cursor position>` and a completion item with an `insertText` of
    /// `console` is provided it will only insert `sole`. Therefore it is
    /// recommended to use `textEdit` instead since it avoids additional client
    /// side interpretation.
    pub insert_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The format of the insert text. The format applies to both the
    /// `insertText` property and the `newText` property of a provided
    /// `textEdit`. If omitted defaults to `InsertTextFormat.PlainText`.
    ///
    /// Please note that the insertTextFormat doesn't apply to
    /// `additionalTextEdits`.
    pub insert_text_format: Option<InsertTextFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// How whitespace and indentation is handled during completion
    /// item insertion. If not provided the clients default value depends on
    /// the `textDocument.completion.insertTextMode` client capability.
    ///
    /// @since 3.16.0
    pub insert_text_mode: Option<InsertTextMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An {@link TextEdit edit} which is applied to a document when selecting
    /// this completion. When an edit is provided the value of
    /// {@link CompletionItem.insertText insertText} is ignored.
    ///
    /// Most editors support two different operations when accepting a completion
    /// item. One is to insert a completion text and the other is to replace an
    /// existing text with a completion text. Since this can usually not be
    /// predetermined by a server it can report both ranges. Clients need to
    /// signal support for `InsertReplaceEdits` via the
    /// `textDocument.completion.insertReplaceSupport` client capability
    /// property.
    ///
    /// *Note 1:* The text edit's range as well as both ranges from an insert
    /// replace edit must be a [single line] and they must contain the position
    /// at which completion has been requested.
    /// *Note 2:* If an `InsertReplaceEdit` is returned the edit's insert range
    /// must be a prefix of the edit's replace range, that means it must be
    /// contained and starting at the same position.
    ///
    /// @since 3.16.0 additional type `InsertReplaceEdit`
    pub text_edit: Option<CompletionItemTextEdit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The edit text used if the completion item is part of a CompletionList and
    /// CompletionList defines an item default for the text edit range.
    ///
    /// Clients will only honor this property if they opt into completion list
    /// item defaults using the capability `completionList.itemDefaults`.
    ///
    /// If not provided and a list's default range is provided the label
    /// property is used as a text.
    ///
    /// @since 3.17.0
    pub text_edit_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional array of additional {@link TextEdit text edits} that are applied when
    /// selecting this completion. Edits must not overlap (including the same insert position)
    /// with the main {@link CompletionItem.textEdit edit} nor with themselves.
    ///
    /// Additional text edits should be used to change text unrelated to the current cursor position
    /// (for example adding an import statement at the top of the file if the completion item will
    /// insert an unqualified type).
    pub additional_text_edits: Option<Vec<TextEdit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional set of characters that when pressed while this completion is active will accept it first and
    /// then type that character. *Note* that all commit characters should have `length=1` and that superfluous
    /// characters will be ignored.
    pub commit_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional {@link Command command} that is executed *after* inserting this completion. *Note* that
    /// additional modifications to the current document should be described with the
    /// {@link CompletionItem.additionalTextEdits additionalTextEdits}-property.
    pub command: Option<Command>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on a completion item between a
    /// {@link CompletionRequest} and a {@link CompletionResolveRequest}.
    pub data: Option<serde_json::Value>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Additional details for a completion item label.
///
/// @since 3.17.0
pub struct CompletionItemLabelDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional string which is rendered less prominently directly after {@link CompletionItem.label label},
    /// without any spacing. Should be used for function signatures and type annotations.
    pub detail: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional string which is rendered less prominently after {@link CompletionItem.detail}. Should be used
    /// for fully qualified names and file paths.
    pub description: Option<String>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
pub struct CompletionItemDefaults {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A default commit character set.
    ///
    /// @since 3.17.0
    pub commit_characters: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A default edit range.
    ///
    /// @since 3.17.0
    pub edit_range: Option<CompletionItemDefaultsEditRange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A default insert text format.
    ///
    /// @since 3.17.0
    pub insert_text_format: Option<InsertTextFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A default insert text mode.
    ///
    /// @since 3.17.0
    pub insert_text_mode: Option<InsertTextMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A default data value.
    ///
    /// @since 3.17.0
    pub data: Option<serde_json::Value>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Specifies how fields from a completion item should be combined with those
/// from `completionList.itemDefaults`.
///
/// If unspecified, all fields will be treated as ApplyKind.Replace.
///
/// If a field's value is ApplyKind.Replace, the value from a completion item (if
/// provided and not `null`) will always be used instead of the value from
/// `completionItem.itemDefaults`.
///
/// If a field's value is ApplyKind.Merge, the values will be merged using the rules
/// defined against each field below.
///
/// Servers are only allowed to return `applyKind` if the client
/// signals support for this via the `completionList.applyKindSupport`
/// capability.
///
/// @since 3.18.0
pub struct CompletionItemApplyKinds {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies whether commitCharacters on a completion will replace or be
    /// merged with those in `completionList.itemDefaults.commitCharacters`.
    ///
    /// If ApplyKind.Replace, the commit characters from the completion item will
    /// always be used unless not provided, in which case those from
    /// `completionList.itemDefaults.commitCharacters` will be used. An
    /// empty list can be used if a completion item does not have any commit
    /// characters and also should not use those from
    /// `completionList.itemDefaults.commitCharacters`.
    ///
    /// If ApplyKind.Merge the commitCharacters for the completion will be the
    /// union of all values in both `completionList.itemDefaults.commitCharacters`
    /// and the completion's own `commitCharacters`.
    ///
    /// @since 3.18.0
    pub commit_characters: Option<ApplyKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Specifies whether the `data` field on a completion will replace or
    /// be merged with data from `completionList.itemDefaults.data`.
    ///
    /// If ApplyKind.Replace, the data from the completion item will be used if
    /// provided (and not `null`), otherwise
    /// `completionList.itemDefaults.data` will be used. An empty object can
    /// be used if a completion item does not have any data but also should
    /// not use the value from `completionList.itemDefaults.data`.
    ///
    /// If ApplyKind.Merge, a shallow merge will be performed between
    /// `completionList.itemDefaults.data` and the completion's own data
    /// using the following rules:
    ///
    /// - If a completion's `data` field is not provided (or `null`), the
    ///   entire `data` field from `completionList.itemDefaults.data` will be
    ///   used as-is.
    /// - If a completion's `data` field is provided, each field will
    ///   overwrite the field of the same name in
    ///   `completionList.itemDefaults.data` but no merging of nested fields
    ///   within that value will occur.
    ///
    /// @since 3.18.0
    pub data: Option<ApplyKind>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.18.0
pub struct ClientCompletionItemOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports snippets as insert text.
    ///
    /// A snippet can define tab stops and placeholders with `$1`, `$2`
    /// and `${3:foo}`. `$0` defines the final tab stop, it defaults to
    /// the end of the snippet. Placeholders with equal identifiers are linked,
    /// that is typing in one will update others too.
    pub snippet_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports commit characters on a completion item.
    pub commit_characters_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the following content formats for the documentation
    /// property. The order describes the preferred format of the client.
    pub documentation_format: Option<Vec<MarkupKind>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the deprecated property on a completion item.
    pub deprecated_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the preselect property on a completion item.
    pub preselect_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the tag property on a completion item. Clients supporting
    /// tags have to handle unknown tags gracefully. Clients especially need to
    /// preserve unknown tags when sending a completion item back to the server in
    /// a resolve call.
    ///
    /// @since 3.15.0
    pub tag_support: Option<CompletionItemTagOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client support insert replace edit to control different behavior if a
    /// completion item is inserted in the text or should replace text.
    ///
    /// @since 3.16.0
    pub insert_replace_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Indicates which properties a client can resolve lazily on a completion
    /// item. Before version 3.16.0 only the predefined properties `documentation`
    /// and `details` could be resolved lazily.
    ///
    /// @since 3.16.0
    pub resolve_support: Option<ClientCompletionItemResolveOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client supports the `insertTextMode` property on
    /// a completion item to override the whitespace handling mode
    /// as defined by the client (see `insertTextMode`).
    ///
    /// @since 3.16.0
    pub insert_text_mode_support: Option<ClientCompletionItemInsertTextModeOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client has support for completion item label
    /// details (see also `CompletionItemLabelDetails`).
    ///
    /// @since 3.17.0
    pub label_details_support: Option<bool>,
}
