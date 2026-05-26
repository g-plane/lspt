// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents a folding range. To be valid, start and end line must be bigger than zero and smaller
/// than the number of lines in the document. Clients are free to ignore invalid ranges.
pub struct FoldingRange {
    /// The zero-based start line of the range to fold. The folded area starts after the line's last character.
    /// To be valid, the end must be zero or larger and smaller than the number of lines in the document.
    pub start_line: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The zero-based character offset from where the folded range starts. If not defined, defaults to the length of the start line.
    pub start_character: Option<u32>,

    /// The zero-based end line of the range to fold. The folded area ends with the line's last character.
    /// To be valid, the end must be zero or larger and smaller than the number of lines in the document.
    pub end_line: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The zero-based character offset before the folded range ends. If not defined, defaults to the length of the end line.
    pub end_character: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Describes the kind of the folding range such as 'comment' or 'region'. The kind
    /// is used to categorize folding ranges and used by commands like 'Fold all comments'.
    /// See {@link FoldingRangeKind} for an enumeration of standardized kinds.
    pub kind: Option<FoldingRangeKind>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// The text that the client should show when the specified range is
    /// collapsed. If not defined or not supported by the client, a default
    /// will be chosen by the client.
    ///
    /// @since 3.17.0
    pub collapsed_text: Option<String>,
}


mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Parameters for a {@link FoldingRangeRequest}.
    pub struct FoldingRangeParams {
        /// The text document.
        pub text_document: TextDocumentIdentifier,

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
    pub struct FoldingRangeRegistrationOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// A document selector to identify the scope of the registration. If set to null
        /// the document selector provided on the client side will be used.
        pub document_selector: Option<DocumentSelector>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The id used to register the request. The id can be used to deregister
        /// the request again. See also Registration#id.
        pub id: Option<String>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FoldingRangeOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub work_done_progress: Option<bool>,
    }


    #[cfg(feature = "proposed")]
    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Client workspace capabilities specific to folding ranges
    ///
    /// @since 3.18.0
    /// @proposed
    pub struct FoldingRangeWorkspaceClientCapabilities {
        #[cfg(feature = "proposed")]
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether the client implementation supports a refresh request sent from the
        /// server to the client.
        ///
        /// Note that this event is global and will force the client to refresh all
        /// folding ranges currently shown. It should be used with absolute care and is
        /// useful for situation where a server for example detects a project wide
        /// change that requires such a calculation.
        ///
        /// @since 3.18.0
        /// @proposed
        pub refresh_support: Option<bool>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FoldingRangeClientCapabilities {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether implementation supports dynamic registration for folding range
        /// providers. If this is set to `true` the client supports the new
        /// `FoldingRangeRegistrationOptions` return value for the corresponding
        /// server capability as well.
        pub dynamic_registration: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The maximum number of folding ranges that the client prefers to receive
        /// per document. The value serves as a hint, servers are free to follow the
        /// limit.
        pub range_limit: Option<u32>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// If set, the client signals that it only supports folding complete lines.
        /// If set, client will ignore specified `startCharacter` and `endCharacter`
        /// properties in a FoldingRange.
        pub line_folding_only: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Specific options for the folding range kind.
        ///
        /// @since 3.17.0
        pub folding_range_kind: Option<ClientFoldingRangeKindOptions>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Specific options for the folding range.
        ///
        /// @since 3.17.0
        pub folding_range: Option<ClientFoldingRangeOptions>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// @since 3.18.0
    pub struct ClientFoldingRangeKindOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// The folding range kind values the client supports. When this
        /// property exists the client also guarantees that it will
        /// handle values outside its set gracefully and falls back
        /// to a default value when unknown.
        pub value_set: Option<Vec<FoldingRangeKind>>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// @since 3.18.0
    pub struct ClientFoldingRangeOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// If set, the client signals that it supports setting collapsedText on
        /// folding ranges to display custom labels instead of the default text.
        ///
        /// @since 3.17.0
        pub collapsed_text: Option<bool>,
    }
}

pub type Params = raw::FoldingRangeParams;

pub type RegistrationOptions = raw::FoldingRangeRegistrationOptions;

pub type Options = raw::FoldingRangeOptions;

#[cfg(feature = "proposed")]
pub type WorkspaceClientCapabilities = raw::FoldingRangeWorkspaceClientCapabilities;

pub mod client {
    pub type Capabilities = super::raw::FoldingRangeClientCapabilities;

    pub type KindOptions = super::raw::ClientFoldingRangeKindOptions;

    pub type Options = super::raw::ClientFoldingRangeOptions;
}