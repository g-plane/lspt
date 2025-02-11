#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]

pub use crate::{enums::*, notifications::*, structs::*, type_aliases::*};
use serde::{Deserialize, Serialize};

mod enums;
mod notifications;
mod structs;
mod type_aliases;

#[cfg(all(feature = "indexmap", feature = "rustc-hash"))]
pub type HashMap<K, V> = indexmap::IndexMap<K, V, rustc_hash::FxBuildHasher>;
#[cfg(all(feature = "indexmap", not(feature = "rustc-hash")))]
pub type HashMap<K, V> = indexmap::IndexMap<K, V>;
#[cfg(all(feature = "rustc-hash", not(feature = "indexmap")))]
pub type HashMap<K, V> = rustc_hash::FxHashMap<K, V>;
#[cfg(all(not(feature = "indexmap"), not(feature = "rustc-hash")))]
pub type HashMap<K, V> = std::collections::HashMap<K, V>;

#[cfg(feature = "url")]
pub type Uri = url::Url;
#[cfg(not(feature = "url"))]
pub type Uri = String;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Union2<A, B> {
    A(A),
    B(B),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Union3<A, B, C> {
    A(A),
    B(B),
    C(C),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Union4<A, B, C, D> {
    A(A),
    B(B),
    C(C),
    D(D),
}
