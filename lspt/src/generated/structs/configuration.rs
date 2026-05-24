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
    /// The parameters of a configuration request.
    pub struct ConfigurationParams {
        pub items: Vec<ConfigurationItem>,
    }


    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ConfigurationItem {
        #[serde(skip_serializing_if = "Option::is_none")]
        /// The scope to get the configuration section for.
        pub scope_uri: Option<Uri>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// The configuration section asked for.
        pub section: Option<String>,
    }
}

pub type Params = raw::ConfigurationParams;

pub type Item = raw::ConfigurationItem;