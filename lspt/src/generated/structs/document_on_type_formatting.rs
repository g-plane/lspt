// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// The parameters of a {@link DocumentOnTypeFormattingRequest}.
pub struct DocumentOnTypeFormattingParams {
    /// The document to format.
    pub text_document: TextDocumentIdentifier,

    /// The position around which the on type formatting should happen.
    /// This is not necessarily the exact position where the character denoted
    /// by the property `ch` got typed.
    pub position: Position,

    /// The character that has been typed that triggered the formatting
    /// on type request. That is not necessarily the last character that
    /// got inserted into the document since the client could auto insert
    /// characters as well (e.g. like automatic brace completion).
    pub ch: String,

    /// The formatting options.
    pub options: FormattingOptions,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Registration options for a {@link DocumentOnTypeFormattingRequest}.
pub struct DocumentOnTypeFormattingRegistrationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// A document selector to identify the scope of the registration. If set to null
    /// the document selector provided on the client side will be used.
    pub document_selector: Option<DocumentSelector>,

    /// A character on which formatting should be triggered, like `{`.
    pub first_trigger_character: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// More trigger characters.
    pub more_trigger_character: Option<Vec<String>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Provider options for a {@link DocumentOnTypeFormattingRequest}.
pub struct DocumentOnTypeFormattingOptions {
    /// A character on which formatting should be triggered, like `{`.
    pub first_trigger_character: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// More trigger characters.
    pub more_trigger_character: Option<Vec<String>>,
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Client capabilities of a {@link DocumentOnTypeFormattingRequest}.
pub struct DocumentOnTypeFormattingClientCapabilities {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// Whether on type formatting supports dynamic registration.
    pub dynamic_registration: Option<bool>,
}

pub type Params = DocumentOnTypeFormattingParams;

pub type RegistrationOptions = DocumentOnTypeFormattingRegistrationOptions;

pub type Options = DocumentOnTypeFormattingOptions;

pub type ClientCapabilities = DocumentOnTypeFormattingClientCapabilities;