// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link CodeActionRequest}.
pub struct CodeActionParams {
    /// The document in which the command was invoked.
    pub text_document: TextDocumentIdentifier,

    /// The range for which the command was invoked.
    pub range: Range,

    /// Context carrying additional information.
    pub context: CodeActionContext,

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
/// A code action represents a change that can be performed in code, e.g. to fix a problem or
/// to refactor code.
///
/// A CodeAction must set either `edit` and/or a `command`. If both are supplied, the `edit` is applied first, then the `command` is executed.
pub struct CodeAction {
    /// A short, human-readable, title for this code action.
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The kind of the code action.
    ///
    /// Used to filter code actions.
    pub kind: Option<CodeActionKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The diagnostics that this code action resolves.
    pub diagnostics: Option<Vec<Diagnostic>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Marks this as a preferred action. Preferred actions are used by the `auto fix` command and can be targeted
    /// by keybindings.
    ///
    /// A quick fix should be marked preferred if it properly addresses the underlying error.
    /// A refactoring should be marked preferred if it is the most reasonable choice of actions to take.
    ///
    /// @since 3.15.0
    pub is_preferred: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Marks that the code action cannot currently be applied.
    ///
    /// Clients should follow the following guidelines regarding disabled code actions:
    ///
    ///   - Disabled code actions are not shown in automatic [lightbulbs](https://code.visualstudio.com/docs/editor/editingevolved#_code-action)
    ///     code action menus.
    ///
    ///   - Disabled actions are shown as faded out in the code action menu when the user requests a more specific type
    ///     of code action, such as refactorings.
    ///
    ///   - If the user has a [keybinding](https://code.visualstudio.com/docs/editor/refactoring#_keybindings-for-code-actions)
    ///     that auto applies a code action and only disabled code actions are returned, the client should show the user an
    ///     error message with `reason` in the editor.
    ///
    /// @since 3.16.0
    pub disabled: Option<CodeActionDisabled>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The workspace edit this code action performs.
    pub edit: Option<WorkspaceEdit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A command this code action executes. If a code action
    /// provides an edit and a command, first the edit is
    /// executed and then the command.
    pub command: Option<Command>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved on a code action between
    /// a `textDocument/codeAction` and a `codeAction/resolve` request.
    ///
    /// @since 3.16.0
    pub data: Option<serde_json::Value>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this code action.
    ///
    /// @since 3.18.0 - proposed
    pub tags: Option<Vec<CodeActionTag>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link CodeActionRequest}.
pub struct CodeActionRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// CodeActionKinds that this server may return.
    ///
    /// The list of kinds may be generic, such as `CodeActionKind.Refactor`, or the server
    /// may list out every specific kind they provide.
    pub code_action_kinds: Option<Vec<CodeActionKind>>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Static documentation for a class of code actions.
    ///
    /// Documentation from the provider should be shown in the code actions menu if either:
    ///
    /// - Code actions of `kind` are requested by the editor. In this case, the editor will show the documentation that
    ///   most closely matches the requested code action kind. For example, if a provider has documentation for
    ///   both `Refactor` and `RefactorExtract`, when the user requests code actions for `RefactorExtract`,
    ///   the editor will use the documentation for `RefactorExtract` instead of the documentation for `Refactor`.
    ///
    /// - Any code actions of `kind` are returned by the provider.
    ///
    /// At most one documentation entry should be shown per provider.
    ///
    /// @since 3.18.0
    /// @proposed
    pub documentation: Option<Vec<CodeActionKindDocumentation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a code action.
    ///
    /// @since 3.16.0
    pub resolve_provider: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Contains additional diagnostic information about the context in which
/// a {@link CodeActionProvider.provideCodeActions code action} is run.
pub struct CodeActionContext {
    /// An array of diagnostics known on the client side overlapping the range provided to the
    /// `textDocument/codeAction` request. They are provided so that the server knows which
    /// errors are currently presented to the user for the given range. There is no guarantee
    /// that these accurately reflect the error state of the resource. The primary parameter
    /// to compute code actions is the provided range.
    pub diagnostics: Vec<Diagnostic>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Requested kind of actions to return.
    ///
    /// Actions not of this kind are filtered out by the client before being shown. So servers
    /// can omit computing them.
    pub only: Option<Vec<CodeActionKind>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The reason why code actions were requested.
    ///
    /// @since 3.17.0
    pub trigger_kind: Option<CodeActionTriggerKind>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Captures why the code action is currently disabled.
///
/// @since 3.18.0
pub struct CodeActionDisabled {
    /// Human readable description of why the code action is currently disabled.
    ///
    /// This is displayed in the code actions UI.
    pub reason: String,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link CodeActionRequest}.
pub struct CodeActionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// CodeActionKinds that this server may return.
    ///
    /// The list of kinds may be generic, such as `CodeActionKind.Refactor`, or the server
    /// may list out every specific kind they provide.
    pub code_action_kinds: Option<Vec<CodeActionKind>>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Static documentation for a class of code actions.
    ///
    /// Documentation from the provider should be shown in the code actions menu if either:
    ///
    /// - Code actions of `kind` are requested by the editor. In this case, the editor will show the documentation that
    ///   most closely matches the requested code action kind. For example, if a provider has documentation for
    ///   both `Refactor` and `RefactorExtract`, when the user requests code actions for `RefactorExtract`,
    ///   the editor will use the documentation for `RefactorExtract` instead of the documentation for `Refactor`.
    ///
    /// - Any code actions of `kind` are returned by the provider.
    ///
    /// At most one documentation entry should be shown per provider.
    ///
    /// @since 3.18.0
    /// @proposed
    pub documentation: Option<Vec<CodeActionKindDocumentation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The server provides support to resolve additional
    /// information for a code action.
    ///
    /// @since 3.16.0
    pub resolve_provider: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The Client Capabilities of a {@link CodeActionRequest}.
pub struct CodeActionClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports dynamic registration.
    pub dynamic_registration: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The client support code action literals of type `CodeAction` as a valid
    /// response of the `textDocument/codeAction` request. If the property is not
    /// set the request can only return `Command` literals.
    ///
    /// @since 3.8.0
    pub code_action_literal_support: Option<ClientCodeActionLiteralOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports the `isPreferred` property.
    ///
    /// @since 3.15.0
    pub is_preferred_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports the `disabled` property.
    ///
    /// @since 3.16.0
    pub disabled_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether code action supports the `data` property which is
    /// preserved between a `textDocument/codeAction` and a
    /// `codeAction/resolve` request.
    ///
    /// @since 3.16.0
    pub data_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports resolving additional code action
    /// properties via a separate `codeAction/resolve` request.
    ///
    /// @since 3.16.0
    pub resolve_support: Option<ClientCodeActionResolveOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client honors the change annotations in
    /// text edits and resource operations returned via the
    /// `CodeAction#edit` property by for example presenting
    /// the workspace edit in the user interface and asking
    /// for confirmation.
    ///
    /// @since 3.16.0
    pub honors_change_annotations: Option<bool>,

    #[cfg(feature = "proposed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether the client supports documentation for a class of
    /// code actions.
    ///
    /// @since 3.18.0
    /// @proposed
    pub documentation_support: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Client supports the tag property on a code action. Clients
    /// supporting tags have to handle unknown tags gracefully.
    ///
    /// @since 3.18.0 - proposed
    pub tag_support: Option<CodeActionTagOptions>,
}

pub type Params = CodeActionParams;

pub type RegistrationOptions = CodeActionRegistrationOptions;

pub type Options = CodeActionOptions;

pub type ClientCapabilities = CodeActionClientCapabilities;