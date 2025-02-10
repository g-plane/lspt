#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]

pub use enums::*;

mod enums;

pub type LSPAny = serde_json::Value;

#[cfg(all(feature = "indexmap", feature = "rustc-hash"))]
compile_error!("only one hash map implementation can be selected at the same time");
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
