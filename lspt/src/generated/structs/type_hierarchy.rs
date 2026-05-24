// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.17.0
pub struct TypeHierarchyItem {
    /// The name of this item.
    pub name: String,

    /// The kind of this item.
    pub kind: SymbolKind,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// Tags for this item.
    pub tags: Option<Vec<SymbolTag>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// More detail for this item, e.g. the signature of a function.
    pub detail: Option<String>,

    /// The resource identifier of this item.
    pub uri: Uri,

    /// The range enclosing this symbol not including leading/trailing whitespace
    /// but everything else, e.g. comments and code.
    pub range: Range,

    /// The range that should be selected and revealed when this symbol is being
    /// picked, e.g. the name of a function. Must be contained by the
    /// {@link TypeHierarchyItem.range `range`}.
    pub selection_range: Range,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// A data entry field that is preserved between a type hierarchy prepare and
    /// supertypes or subtypes requests. It could also be used to identify the
    /// type hierarchy in the server, helping improve the performance on
    /// resolving supertypes and subtypes.
    pub data: Option<serde_json::Value>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Type hierarchy options used during static or dynamic registration.
///
/// @since 3.17.0
pub struct TypeHierarchyRegistrationOptions {
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
/// Type hierarchy options used during static registration.
///
/// @since 3.17.0
pub struct TypeHierarchyOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_done_progress: Option<bool>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// @since 3.17.0
pub struct TypeHierarchyClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether implementation supports dynamic registration. If this is set to `true`
    /// the client supports the new `(TextDocumentRegistrationOptions & StaticRegistrationOptions)`
    /// return value for the corresponding server capability as well.
    pub dynamic_registration: Option<bool>,
}

pub type RegistrationOptions = TypeHierarchyRegistrationOptions;

pub type Options = TypeHierarchyOptions;

pub type ClientCapabilities = TypeHierarchyClientCapabilities;