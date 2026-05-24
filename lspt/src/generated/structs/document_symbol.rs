// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Represents programming constructs like variables, classes, interfaces etc.
/// that appear in a document. Document symbols can be hierarchical and they
/// have two ranges: one that encloses its definition and one that points to
/// its most interesting range, e.g. the range of an identifier.
pub struct DocumentSymbol {
    /// The name of this symbol. Will be displayed in the user interface and therefore must not be
    /// an empty string or a string only consisting of white spaces.
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// More detail for this symbol, e.g the signature of a function.
    pub detail: Option<String>,

    /// The kind of this symbol.
    pub kind: SymbolKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this document symbol.
    ///
    /// @since 3.16.0
    pub tags: Option<Vec<SymbolTag>>,

    /// The range enclosing this symbol not including leading/trailing whitespace but everything else
    /// like comments. This information is typically used to determine if the clients cursor is
    /// inside the symbol to reveal in the symbol in the UI.
    pub range: Range,

    /// The range that should be selected and revealed when this symbol is being picked, e.g the name of a function.
    /// Must be contained by the `range`.
    pub selection_range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Children of this symbol, e.g. properties of a class.
    pub children: Option<Vec<DocumentSymbol>>,
}


mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Parameters for a {@link DocumentSymbolRequest}.
    pub struct DocumentSymbolParams {
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
    /// Registration options for a {@link DocumentSymbolRequest}.
    pub struct DocumentSymbolRegistrationOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// A document selector to identify the scope of the registration. If set to null
        /// the document selector provided on the client side will be used.
        pub document_selector: Option<DocumentSelector>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// A human-readable string that is shown when multiple outlines trees
        /// are shown for the same document.
        ///
        /// @since 3.16.0
        pub label: Option<String>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Provider options for a {@link DocumentSymbolRequest}.
    pub struct DocumentSymbolOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// A human-readable string that is shown when multiple outlines trees
        /// are shown for the same document.
        ///
        /// @since 3.16.0
        pub label: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub work_done_progress: Option<bool>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Client Capabilities for a {@link DocumentSymbolRequest}.
    pub struct DocumentSymbolClientCapabilities {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// Whether document symbol supports dynamic registration.
        pub dynamic_registration: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// Specific capabilities for the `SymbolKind` in the
        /// `textDocument/documentSymbol` request.
        pub symbol_kind: Option<ClientSymbolKindOptions>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client supports hierarchical document symbols.
        pub hierarchical_document_symbol_support: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client supports tags on `SymbolInformation`. Tags are supported on
        /// `DocumentSymbol` if `hierarchicalDocumentSymbolSupport` is set to true.
        /// Clients supporting tags have to handle unknown tags gracefully.
        ///
        /// @since 3.16.0
        pub tag_support: Option<ClientSymbolTagOptions>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The client supports an additional label presented in the UI when
        /// registering a document symbol provider.
        ///
        /// @since 3.16.0
        pub label_support: Option<bool>,
    }
}

pub type Params = raw::DocumentSymbolParams;

pub type RegistrationOptions = raw::DocumentSymbolRegistrationOptions;

pub type Options = raw::DocumentSymbolOptions;

pub type ClientCapabilities = raw::DocumentSymbolClientCapabilities;