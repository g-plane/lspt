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
    /// The parameters of a notification message.
    pub struct ShowMessageParams {
        #[serde(rename = "type")]
        /// The message type. See {@link MessageType}
        pub ty: MessageType,

        /// The actual message.
        pub message: String,
    }
}

pub type Params = raw::ShowMessageParams;