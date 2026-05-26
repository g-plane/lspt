// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// The change text document notification's parameters.
    pub struct DidChangeTextDocumentParams {
        /// The document that did change. The version number points
        /// to the version after all provided content changes have
        /// been applied.
        pub text_document: VersionedTextDocumentIdentifier,

        /// The actual content changes. The content changes describe single state changes
        /// to the document. So if there are two content changes c1 (at array index 0) and
        /// c2 (at array index 1) for a document in state S then c1 moves the document from
        /// S to S' and c2 from S' to S''. So c1 is computed on the state S and c2 is computed
        /// on the state S'.
        ///
        /// To mirror the content of a document using change events use the following approach:
        /// - start with the same initial content
        /// - apply the 'textDocument/didChange' notifications in the order you receive them.
        /// - apply the `TextDocumentContentChangeEvent`s in a single notification in the order
        ///   you receive them.
        pub content_changes: Vec<TextDocumentContentChangeEvent>,
    }


    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Describe options to be used when registered for text document change events.
    pub struct TextDocumentChangeRegistrationOptions {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// A document selector to identify the scope of the registration. If set to null
        /// the document selector provided on the client side will be used.
        pub document_selector: Option<DocumentSelector>,

        /// How documents are synced to the server.
        pub sync_kind: TextDocumentSyncKind,
    }
}

pub type Params = raw::DidChangeTextDocumentParams;

pub type RegistrationOptions = raw::TextDocumentChangeRegistrationOptions;