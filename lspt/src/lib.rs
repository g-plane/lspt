#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]

pub use enums::*;

mod enums;

pub type LSPAny = serde_json::Value;
