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
    pub struct WorkDoneProgressCreateParams {
        /// The token to be used to report progress.
        pub token: ProgressToken,
    }


    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WorkDoneProgressCancelParams {
        /// The token to be used to report progress.
        pub token: ProgressToken,
    }
}

pub type CreateParams = raw::WorkDoneProgressCreateParams;

pub type CancelParams = raw::WorkDoneProgressCancelParams;