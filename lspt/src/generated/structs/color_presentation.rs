// DO NOT EDIT THIS GENERATED FILE.

#![allow(unused_imports)]

use crate::{HashMap, Uri};
use serde::{Deserialize, Serialize};
use super::*;
use super::super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorPresentation {
    /// The label of this color presentation. It will be shown on the color
    /// picker header. By default this is also the text that is inserted when selecting
    /// this color presentation.
    pub label: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An {@link TextEdit edit} which is applied to a document when selecting
    /// this presentation for the color.  When `falsy` the {@link ColorPresentation.label label}
    /// is used.
    pub text_edit: Option<TextEdit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    /// An optional array of additional {@link TextEdit text edits} that are applied when
    /// selecting this color presentation. Edits must not overlap with the main {@link ColorPresentation.textEdit edit} nor with themselves.
    pub additional_text_edits: Option<Vec<TextEdit>>,
}


mod raw {
    #![allow(unused_imports)]

    use crate::{HashMap, Uri};
    use serde::{Deserialize, Serialize};
    use super::*;
    use super::super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    /// Parameters for a {@link ColorPresentationRequest}.
    pub struct ColorPresentationParams {
        /// The text document.
        pub text_document: TextDocumentIdentifier,

        /// The color to request presentations for.
        pub color: Color,

        /// The range where the color would be inserted. Serves as a context.
        pub range: Range,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// An optional token that a server can use to report work done progress.
        pub work_done_token: Option<ProgressToken>,

        #[serde(skip_serializing_if = "Option::is_none")]
        /// An optional token that a server can use to report partial results (e.g. streaming) to
        /// the client.
        pub partial_result_token: Option<ProgressToken>,
    }
}

pub type Params = raw::ColorPresentationParams;