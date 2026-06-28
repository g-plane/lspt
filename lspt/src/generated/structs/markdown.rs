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

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Client capabilities specific to the used markdown parser.
    ///
    /// @since 3.16.0
    pub struct MarkdownClientCapabilities {
        /// The name of the parser.
        pub parser: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The version of the parser.
        pub version: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// A list of HTML tags that the client allows / supports in
        /// Markdown.
        ///
        /// @since 3.17.0
        pub allowed_tags: Option<Vec<String>>,
    }
}

pub type ClientCapabilities = raw::MarkdownClientCapabilities;