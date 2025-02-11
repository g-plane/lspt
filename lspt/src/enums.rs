// DO NOT EDIT THIS GENERATED FILE.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A set of predefined token types. This set is not fixed
/// an clients can specify additional token types via the
/// corresponding client capabilities.
///
/// @since 3.16.0
pub enum SemanticTokenTypes {
    #[serde(rename = "namespace")]
    Namespace,

    #[serde(rename = "type")]
    /// Represents a generic type. Acts as a fallback for types which can't be mapped to
    /// a specific type like class or enum.
    Type,

    #[serde(rename = "class")]
    Class,

    #[serde(rename = "enum")]
    Enum,

    #[serde(rename = "interface")]
    Interface,

    #[serde(rename = "struct")]
    Struct,

    #[serde(rename = "typeParameter")]
    TypeParameter,

    #[serde(rename = "parameter")]
    Parameter,

    #[serde(rename = "variable")]
    Variable,

    #[serde(rename = "property")]
    Property,

    #[serde(rename = "enumMember")]
    EnumMember,

    #[serde(rename = "event")]
    Event,

    #[serde(rename = "function")]
    Function,

    #[serde(rename = "method")]
    Method,

    #[serde(rename = "macro")]
    Macro,

    #[serde(rename = "keyword")]
    Keyword,

    #[serde(rename = "modifier")]
    Modifier,

    #[serde(rename = "comment")]
    Comment,

    #[serde(rename = "string")]
    String,

    #[serde(rename = "number")]
    Number,

    #[serde(rename = "regexp")]
    Regexp,

    #[serde(rename = "operator")]
    Operator,

    #[serde(rename = "decorator")]
    /// @since 3.17.0
    Decorator,

    #[serde(rename = "label")]
    /// @since 3.18.0
    Label,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A set of predefined token modifiers. This set is not fixed
/// an clients can specify additional token types via the
/// corresponding client capabilities.
///
/// @since 3.16.0
pub enum SemanticTokenModifiers {
    #[serde(rename = "declaration")]
    Declaration,

    #[serde(rename = "definition")]
    Definition,

    #[serde(rename = "readonly")]
    Readonly,

    #[serde(rename = "static")]
    Static,

    #[serde(rename = "deprecated")]
    Deprecated,

    #[serde(rename = "abstract")]
    Abstract,

    #[serde(rename = "async")]
    Async,

    #[serde(rename = "modification")]
    Modification,

    #[serde(rename = "documentation")]
    Documentation,

    #[serde(rename = "defaultLibrary")]
    DefaultLibrary,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// The document diagnostic report kinds.
///
/// @since 3.17.0
pub enum DocumentDiagnosticReportKind {
    #[serde(rename = "full")]
    /// A diagnostic report with a full
    /// set of problems.
    Full,

    #[serde(rename = "unchanged")]
    /// A report indicating that the last
    /// returned report is still accurate.
    Unchanged,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Predefined error codes.
pub enum ErrorCodes {
    ParseError = -32700,

    InvalidRequest = -32600,

    MethodNotFound = -32601,

    InvalidParams = -32602,

    InternalError = -32603,

    /// Error code indicating that a server received a notification or
    /// request before the server has received the `initialize` request.
    ServerNotInitialized = -32002,

    UnknownErrorCode = -32001,
}
impl Serialize for ErrorCodes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ErrorCodes::ParseError => serializer.serialize_i32(-32700),
            ErrorCodes::InvalidRequest => serializer.serialize_i32(-32600),
            ErrorCodes::MethodNotFound => serializer.serialize_i32(-32601),
            ErrorCodes::InvalidParams => serializer.serialize_i32(-32602),
            ErrorCodes::InternalError => serializer.serialize_i32(-32603),
            ErrorCodes::ServerNotInitialized => serializer.serialize_i32(-32002),
            ErrorCodes::UnknownErrorCode => serializer.serialize_i32(-32001),
        }
    }
}
impl<'de> Deserialize<'de> for ErrorCodes {
    fn deserialize<D>(deserializer: D) -> Result<ErrorCodes, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            -32700 => Ok(ErrorCodes::ParseError),
            -32600 => Ok(ErrorCodes::InvalidRequest),
            -32601 => Ok(ErrorCodes::MethodNotFound),
            -32602 => Ok(ErrorCodes::InvalidParams),
            -32603 => Ok(ErrorCodes::InternalError),
            -32002 => Ok(ErrorCodes::ServerNotInitialized),
            -32001 => Ok(ErrorCodes::UnknownErrorCode),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of -32700, -32600, -32601, -32602, -32603, -32002, -32001",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LSPErrorCodes {
    /// A request failed but it was syntactically correct, e.g the
    /// method name was known and the parameters were valid. The error
    /// message should contain human readable information about why
    /// the request failed.
    ///
    /// @since 3.17.0
    RequestFailed = -32803,

    /// The server cancelled the request. This error code should
    /// only be used for requests that explicitly support being
    /// server cancellable.
    ///
    /// @since 3.17.0
    ServerCancelled = -32802,

    /// The server detected that the content of a document got
    /// modified outside normal conditions. A server should
    /// NOT send this error code if it detects a content change
    /// in it unprocessed messages. The result even computed
    /// on an older state might still be useful for the client.
    ///
    /// If a client decides that a result is not of any use anymore
    /// the client should cancel the request.
    ContentModified = -32801,

    /// The client has canceled a request and a server has detected
    /// the cancel.
    RequestCancelled = -32800,
}
impl Serialize for LSPErrorCodes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LSPErrorCodes::RequestFailed => serializer.serialize_i32(-32803),
            LSPErrorCodes::ServerCancelled => serializer.serialize_i32(-32802),
            LSPErrorCodes::ContentModified => serializer.serialize_i32(-32801),
            LSPErrorCodes::RequestCancelled => serializer.serialize_i32(-32800),
        }
    }
}
impl<'de> Deserialize<'de> for LSPErrorCodes {
    fn deserialize<D>(deserializer: D) -> Result<LSPErrorCodes, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = i32::deserialize(deserializer)?;
        match value {
            -32803 => Ok(LSPErrorCodes::RequestFailed),
            -32802 => Ok(LSPErrorCodes::ServerCancelled),
            -32801 => Ok(LSPErrorCodes::ContentModified),
            -32800 => Ok(LSPErrorCodes::RequestCancelled),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of -32803, -32802, -32801, -32800",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A set of predefined range kinds.
pub enum FoldingRangeKind {
    #[serde(rename = "comment")]
    /// Folding range for a comment
    Comment,

    #[serde(rename = "imports")]
    /// Folding range for an import or include
    Imports,

    #[serde(rename = "region")]
    /// Folding range for a region (e.g. `#region`)
    Region,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A symbol kind.
pub enum SymbolKind {
    File = 1,

    Module = 2,

    Namespace = 3,

    Package = 4,

    Class = 5,

    Method = 6,

    Property = 7,

    Field = 8,

    Constructor = 9,

    Enum = 10,

    Interface = 11,

    Function = 12,

    Variable = 13,

    Constant = 14,

    String = 15,

    Number = 16,

    Boolean = 17,

    Array = 18,

    Object = 19,

    Key = 20,

    Null = 21,

    EnumMember = 22,

    Struct = 23,

    Event = 24,

    Operator = 25,

    TypeParameter = 26,
}
impl Serialize for SymbolKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SymbolKind::File => serializer.serialize_u32(1),
            SymbolKind::Module => serializer.serialize_u32(2),
            SymbolKind::Namespace => serializer.serialize_u32(3),
            SymbolKind::Package => serializer.serialize_u32(4),
            SymbolKind::Class => serializer.serialize_u32(5),
            SymbolKind::Method => serializer.serialize_u32(6),
            SymbolKind::Property => serializer.serialize_u32(7),
            SymbolKind::Field => serializer.serialize_u32(8),
            SymbolKind::Constructor => serializer.serialize_u32(9),
            SymbolKind::Enum => serializer.serialize_u32(10),
            SymbolKind::Interface => serializer.serialize_u32(11),
            SymbolKind::Function => serializer.serialize_u32(12),
            SymbolKind::Variable => serializer.serialize_u32(13),
            SymbolKind::Constant => serializer.serialize_u32(14),
            SymbolKind::String => serializer.serialize_u32(15),
            SymbolKind::Number => serializer.serialize_u32(16),
            SymbolKind::Boolean => serializer.serialize_u32(17),
            SymbolKind::Array => serializer.serialize_u32(18),
            SymbolKind::Object => serializer.serialize_u32(19),
            SymbolKind::Key => serializer.serialize_u32(20),
            SymbolKind::Null => serializer.serialize_u32(21),
            SymbolKind::EnumMember => serializer.serialize_u32(22),
            SymbolKind::Struct => serializer.serialize_u32(23),
            SymbolKind::Event => serializer.serialize_u32(24),
            SymbolKind::Operator => serializer.serialize_u32(25),
            SymbolKind::TypeParameter => serializer.serialize_u32(26),
        }
    }
}
impl<'de> Deserialize<'de> for SymbolKind {
    fn deserialize<D>(deserializer: D) -> Result<SymbolKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(SymbolKind::File),
            2 => Ok(SymbolKind::Module),
            3 => Ok(SymbolKind::Namespace),
            4 => Ok(SymbolKind::Package),
            5 => Ok(SymbolKind::Class),
            6 => Ok(SymbolKind::Method),
            7 => Ok(SymbolKind::Property),
            8 => Ok(SymbolKind::Field),
            9 => Ok(SymbolKind::Constructor),
            10 => Ok(SymbolKind::Enum),
            11 => Ok(SymbolKind::Interface),
            12 => Ok(SymbolKind::Function),
            13 => Ok(SymbolKind::Variable),
            14 => Ok(SymbolKind::Constant),
            15 => Ok(SymbolKind::String),
            16 => Ok(SymbolKind::Number),
            17 => Ok(SymbolKind::Boolean),
            18 => Ok(SymbolKind::Array),
            19 => Ok(SymbolKind::Object),
            20 => Ok(SymbolKind::Key),
            21 => Ok(SymbolKind::Null),
            22 => Ok(SymbolKind::EnumMember),
            23 => Ok(SymbolKind::Struct),
            24 => Ok(SymbolKind::Event),
            25 => Ok(SymbolKind::Operator),
            26 => Ok(SymbolKind::TypeParameter),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Symbol tags are extra annotations that tweak the rendering of a symbol.
///
/// @since 3.16
pub enum SymbolTag {
    /// Render a symbol as obsolete, usually using a strike-out.
    Deprecated = 1,
}
impl Serialize for SymbolTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SymbolTag::Deprecated => serializer.serialize_u32(1),
        }
    }
}
impl<'de> Deserialize<'de> for SymbolTag {
    fn deserialize<D>(deserializer: D) -> Result<SymbolTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(SymbolTag::Deprecated),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Moniker uniqueness level to define scope of the moniker.
///
/// @since 3.16.0
pub enum UniquenessLevel {
    #[serde(rename = "document")]
    /// The moniker is only unique inside a document
    Document,

    #[serde(rename = "project")]
    /// The moniker is unique inside a project for which a dump got created
    Project,

    #[serde(rename = "group")]
    /// The moniker is unique inside the group to which a project belongs
    Group,

    #[serde(rename = "scheme")]
    /// The moniker is unique inside the moniker scheme.
    Scheme,

    #[serde(rename = "global")]
    /// The moniker is globally unique
    Global,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// The moniker kind.
///
/// @since 3.16.0
pub enum MonikerKind {
    #[serde(rename = "import")]
    /// The moniker represent a symbol that is imported into a project
    Import,

    #[serde(rename = "export")]
    /// The moniker represents a symbol that is exported from a project
    Export,

    #[serde(rename = "local")]
    /// The moniker represents a symbol that is local to a project (e.g. a local
    /// variable of a function, a class not visible outside the project, ...)
    Local,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Inlay hint kinds.
///
/// @since 3.17.0
pub enum InlayHintKind {
    /// An inlay hint that for a type annotation.
    Type = 1,

    /// An inlay hint that is for a parameter.
    Parameter = 2,
}
impl Serialize for InlayHintKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InlayHintKind::Type => serializer.serialize_u32(1),
            InlayHintKind::Parameter => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for InlayHintKind {
    fn deserialize<D>(deserializer: D) -> Result<InlayHintKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(InlayHintKind::Type),
            2 => Ok(InlayHintKind::Parameter),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The message type
pub enum MessageType {
    /// An error message.
    Error = 1,

    /// A warning message.
    Warning = 2,

    /// An information message.
    Info = 3,

    /// A log message.
    Log = 4,

    #[cfg(feature = "proposed")]
    /// A debug message.
    ///
    /// @since 3.18.0
    /// @proposed
    Debug = 5,
}
impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MessageType::Error => serializer.serialize_u32(1),
            MessageType::Warning => serializer.serialize_u32(2),
            MessageType::Info => serializer.serialize_u32(3),
            MessageType::Log => serializer.serialize_u32(4),
            #[cfg(feature = "proposed")]
            MessageType::Debug => serializer.serialize_u32(5),
        }
    }
}
impl<'de> Deserialize<'de> for MessageType {
    fn deserialize<D>(deserializer: D) -> Result<MessageType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(MessageType::Error),
            2 => Ok(MessageType::Warning),
            3 => Ok(MessageType::Info),
            4 => Ok(MessageType::Log),
            #[cfg(feature = "proposed")]
            5 => Ok(MessageType::Debug),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3, 4, 5",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Defines how the host (editor) should sync
/// document changes to the language server.
pub enum TextDocumentSyncKind {
    /// Documents should not be synced at all.
    None = 0,

    /// Documents are synced by always sending the full content
    /// of the document.
    Full = 1,

    /// Documents are synced by sending the full content on open.
    /// After that only incremental updates to the document are
    /// send.
    Incremental = 2,
}
impl Serialize for TextDocumentSyncKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TextDocumentSyncKind::None => serializer.serialize_u32(0),
            TextDocumentSyncKind::Full => serializer.serialize_u32(1),
            TextDocumentSyncKind::Incremental => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for TextDocumentSyncKind {
    fn deserialize<D>(deserializer: D) -> Result<TextDocumentSyncKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            0 => Ok(TextDocumentSyncKind::None),
            1 => Ok(TextDocumentSyncKind::Full),
            2 => Ok(TextDocumentSyncKind::Incremental),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 0, 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Represents reasons why a text document is saved.
pub enum TextDocumentSaveReason {
    /// Manually triggered, e.g. by the user pressing save, by starting debugging,
    /// or by an API call.
    Manual = 1,

    /// Automatic after a delay.
    AfterDelay = 2,

    /// When the editor lost focus.
    FocusOut = 3,
}
impl Serialize for TextDocumentSaveReason {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            TextDocumentSaveReason::Manual => serializer.serialize_u32(1),
            TextDocumentSaveReason::AfterDelay => serializer.serialize_u32(2),
            TextDocumentSaveReason::FocusOut => serializer.serialize_u32(3),
        }
    }
}
impl<'de> Deserialize<'de> for TextDocumentSaveReason {
    fn deserialize<D>(deserializer: D) -> Result<TextDocumentSaveReason, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(TextDocumentSaveReason::Manual),
            2 => Ok(TextDocumentSaveReason::AfterDelay),
            3 => Ok(TextDocumentSaveReason::FocusOut),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The kind of a completion entry.
pub enum CompletionItemKind {
    Text = 1,

    Method = 2,

    Function = 3,

    Constructor = 4,

    Field = 5,

    Variable = 6,

    Class = 7,

    Interface = 8,

    Module = 9,

    Property = 10,

    Unit = 11,

    Value = 12,

    Enum = 13,

    Keyword = 14,

    Snippet = 15,

    Color = 16,

    File = 17,

    Reference = 18,

    Folder = 19,

    EnumMember = 20,

    Constant = 21,

    Struct = 22,

    Event = 23,

    Operator = 24,

    TypeParameter = 25,
}
impl Serialize for CompletionItemKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CompletionItemKind::Text => serializer.serialize_u32(1),
            CompletionItemKind::Method => serializer.serialize_u32(2),
            CompletionItemKind::Function => serializer.serialize_u32(3),
            CompletionItemKind::Constructor => serializer.serialize_u32(4),
            CompletionItemKind::Field => serializer.serialize_u32(5),
            CompletionItemKind::Variable => serializer.serialize_u32(6),
            CompletionItemKind::Class => serializer.serialize_u32(7),
            CompletionItemKind::Interface => serializer.serialize_u32(8),
            CompletionItemKind::Module => serializer.serialize_u32(9),
            CompletionItemKind::Property => serializer.serialize_u32(10),
            CompletionItemKind::Unit => serializer.serialize_u32(11),
            CompletionItemKind::Value => serializer.serialize_u32(12),
            CompletionItemKind::Enum => serializer.serialize_u32(13),
            CompletionItemKind::Keyword => serializer.serialize_u32(14),
            CompletionItemKind::Snippet => serializer.serialize_u32(15),
            CompletionItemKind::Color => serializer.serialize_u32(16),
            CompletionItemKind::File => serializer.serialize_u32(17),
            CompletionItemKind::Reference => serializer.serialize_u32(18),
            CompletionItemKind::Folder => serializer.serialize_u32(19),
            CompletionItemKind::EnumMember => serializer.serialize_u32(20),
            CompletionItemKind::Constant => serializer.serialize_u32(21),
            CompletionItemKind::Struct => serializer.serialize_u32(22),
            CompletionItemKind::Event => serializer.serialize_u32(23),
            CompletionItemKind::Operator => serializer.serialize_u32(24),
            CompletionItemKind::TypeParameter => serializer.serialize_u32(25),
        }
    }
}
impl<'de> Deserialize<'de> for CompletionItemKind {
    fn deserialize<D>(deserializer: D) -> Result<CompletionItemKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CompletionItemKind::Text),
            2 => Ok(CompletionItemKind::Method),
            3 => Ok(CompletionItemKind::Function),
            4 => Ok(CompletionItemKind::Constructor),
            5 => Ok(CompletionItemKind::Field),
            6 => Ok(CompletionItemKind::Variable),
            7 => Ok(CompletionItemKind::Class),
            8 => Ok(CompletionItemKind::Interface),
            9 => Ok(CompletionItemKind::Module),
            10 => Ok(CompletionItemKind::Property),
            11 => Ok(CompletionItemKind::Unit),
            12 => Ok(CompletionItemKind::Value),
            13 => Ok(CompletionItemKind::Enum),
            14 => Ok(CompletionItemKind::Keyword),
            15 => Ok(CompletionItemKind::Snippet),
            16 => Ok(CompletionItemKind::Color),
            17 => Ok(CompletionItemKind::File),
            18 => Ok(CompletionItemKind::Reference),
            19 => Ok(CompletionItemKind::Folder),
            20 => Ok(CompletionItemKind::EnumMember),
            21 => Ok(CompletionItemKind::Constant),
            22 => Ok(CompletionItemKind::Struct),
            23 => Ok(CompletionItemKind::Event),
            24 => Ok(CompletionItemKind::Operator),
            25 => Ok(CompletionItemKind::TypeParameter),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Completion item tags are extra annotations that tweak the rendering of a completion
/// item.
///
/// @since 3.15.0
pub enum CompletionItemTag {
    /// Render a completion as obsolete, usually using a strike-out.
    Deprecated = 1,
}
impl Serialize for CompletionItemTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CompletionItemTag::Deprecated => serializer.serialize_u32(1),
        }
    }
}
impl<'de> Deserialize<'de> for CompletionItemTag {
    fn deserialize<D>(deserializer: D) -> Result<CompletionItemTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CompletionItemTag::Deprecated),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Defines whether the insert text in a completion item should be interpreted as
/// plain text or a snippet.
pub enum InsertTextFormat {
    /// The primary text to be inserted is treated as a plain string.
    PlainText = 1,

    /// The primary text to be inserted is treated as a snippet.
    ///
    /// A snippet can define tab stops and placeholders with `$1`, `$2`
    /// and `${3:foo}`. `$0` defines the final tab stop, it defaults to
    /// the end of the snippet. Placeholders with equal identifiers are linked,
    /// that is typing in one will update others too.
    ///
    /// See also: https://microsoft.github.io/language-server-protocol/specifications/specification-current/#snippet_syntax
    Snippet = 2,
}
impl Serialize for InsertTextFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InsertTextFormat::PlainText => serializer.serialize_u32(1),
            InsertTextFormat::Snippet => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for InsertTextFormat {
    fn deserialize<D>(deserializer: D) -> Result<InsertTextFormat, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(InsertTextFormat::PlainText),
            2 => Ok(InsertTextFormat::Snippet),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// How whitespace and indentation is handled during completion
/// item insertion.
///
/// @since 3.16.0
pub enum InsertTextMode {
    /// The insertion or replace strings is taken as it is. If the
    /// value is multi line the lines below the cursor will be
    /// inserted using the indentation defined in the string value.
    /// The client will not apply any kind of adjustments to the
    /// string.
    AsIs = 1,

    /// The editor adjusts leading whitespace of new lines so that
    /// they match the indentation up to the cursor of the line for
    /// which the item is accepted.
    ///
    /// Consider a line like this: <2tabs><cursor><3tabs>foo. Accepting a
    /// multi line completion item is indented using 2 tabs and all
    /// following lines inserted will be indented using 2 tabs as well.
    AdjustIndentation = 2,
}
impl Serialize for InsertTextMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InsertTextMode::AsIs => serializer.serialize_u32(1),
            InsertTextMode::AdjustIndentation => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for InsertTextMode {
    fn deserialize<D>(deserializer: D) -> Result<InsertTextMode, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(InsertTextMode::AsIs),
            2 => Ok(InsertTextMode::AdjustIndentation),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A document highlight kind.
pub enum DocumentHighlightKind {
    /// A textual occurrence.
    Text = 1,

    /// Read-access of a symbol, like reading a variable.
    Read = 2,

    /// Write-access of a symbol, like writing to a variable.
    Write = 3,
}
impl Serialize for DocumentHighlightKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DocumentHighlightKind::Text => serializer.serialize_u32(1),
            DocumentHighlightKind::Read => serializer.serialize_u32(2),
            DocumentHighlightKind::Write => serializer.serialize_u32(3),
        }
    }
}
impl<'de> Deserialize<'de> for DocumentHighlightKind {
    fn deserialize<D>(deserializer: D) -> Result<DocumentHighlightKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(DocumentHighlightKind::Text),
            2 => Ok(DocumentHighlightKind::Read),
            3 => Ok(DocumentHighlightKind::Write),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A set of predefined code action kinds
pub enum CodeActionKind {
    #[serde(rename = "")]
    /// Empty kind.
    Empty,

    #[serde(rename = "quickfix")]
    /// Base kind for quickfix actions: 'quickfix'
    QuickFix,

    #[serde(rename = "refactor")]
    /// Base kind for refactoring actions: 'refactor'
    Refactor,

    #[serde(rename = "refactor.extract")]
    /// Base kind for refactoring extraction actions: 'refactor.extract'
    ///
    /// Example extract actions:
    ///
    /// - Extract method
    /// - Extract function
    /// - Extract variable
    /// - Extract interface from class
    /// - ...
    RefactorExtract,

    #[serde(rename = "refactor.inline")]
    /// Base kind for refactoring inline actions: 'refactor.inline'
    ///
    /// Example inline actions:
    ///
    /// - Inline function
    /// - Inline variable
    /// - Inline constant
    /// - ...
    RefactorInline,

    #[cfg(feature = "proposed")]
    #[serde(rename = "refactor.move")]
    /// Base kind for refactoring move actions: `refactor.move`
    ///
    /// Example move actions:
    ///
    /// - Move a function to a new file
    /// - Move a property between classes
    /// - Move method to base class
    /// - ...
    ///
    /// @since 3.18.0
    /// @proposed
    RefactorMove,

    #[serde(rename = "refactor.rewrite")]
    /// Base kind for refactoring rewrite actions: 'refactor.rewrite'
    ///
    /// Example rewrite actions:
    ///
    /// - Convert JavaScript function to class
    /// - Add or remove parameter
    /// - Encapsulate field
    /// - Make method static
    /// - Move method to base class
    /// - ...
    RefactorRewrite,

    #[serde(rename = "source")]
    /// Base kind for source actions: `source`
    ///
    /// Source code actions apply to the entire file.
    Source,

    #[serde(rename = "source.organizeImports")]
    /// Base kind for an organize imports source action: `source.organizeImports`
    SourceOrganizeImports,

    #[serde(rename = "source.fixAll")]
    /// Base kind for auto-fix source actions: `source.fixAll`.
    ///
    /// Fix all actions automatically fix errors that have a clear fix that do not require user input.
    /// They should not suppress errors or perform unsafe fixes such as generating new types or classes.
    ///
    /// @since 3.15.0
    SourceFixAll,

    #[serde(rename = "notebook")]
    /// Base kind for all code actions applying to the entire notebook's scope. CodeActionKinds using
    /// this should always begin with `notebook.`
    ///
    /// @since 3.18.0
    Notebook,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Code action tags are extra annotations that tweak the behavior of a code action.
///
/// @since 3.18.0 - proposed
pub enum CodeActionTag {
    /// Marks the code action as LLM-generated.
    LlmGenerated = 1,
}
impl Serialize for CodeActionTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CodeActionTag::LlmGenerated => serializer.serialize_u32(1),
        }
    }
}
impl<'de> Deserialize<'de> for CodeActionTag {
    fn deserialize<D>(deserializer: D) -> Result<CodeActionTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CodeActionTag::LlmGenerated),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceValue {
    #[serde(rename = "off")]
    /// Turn tracing off.
    Off,

    #[serde(rename = "messages")]
    /// Trace messages only.
    Messages,

    #[serde(rename = "verbose")]
    /// Verbose message tracing.
    Verbose,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Describes the content type that a client supports in various
/// result literals like `Hover`, `ParameterInfo` or `CompletionItem`.
///
/// Please note that `MarkupKinds` must not start with a `$`. This kinds
/// are reserved for internal usage.
pub enum MarkupKind {
    #[serde(rename = "plaintext")]
    /// Plain text is supported as a content format
    PlainText,

    #[serde(rename = "markdown")]
    /// Markdown is supported as a content format
    Markdown,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// Predefined Language kinds
/// @since 3.18.0
/// @proposed
pub enum LanguageKind {
    #[serde(rename = "abap")]
    Abap,

    #[serde(rename = "bat")]
    WindowsBat,

    #[serde(rename = "bibtex")]
    BibTeX,

    #[serde(rename = "clojure")]
    Clojure,

    #[serde(rename = "coffeescript")]
    Coffeescript,

    #[serde(rename = "c")]
    C,

    #[serde(rename = "cpp")]
    Cpp,

    #[serde(rename = "csharp")]
    CSharp,

    #[serde(rename = "css")]
    Css,

    #[cfg(feature = "proposed")]
    #[serde(rename = "d")]
    /// @since 3.18.0
    /// @proposed
    D,

    #[cfg(feature = "proposed")]
    #[serde(rename = "pascal")]
    /// @since 3.18.0
    /// @proposed
    Delphi,

    #[serde(rename = "diff")]
    Diff,

    #[serde(rename = "dart")]
    Dart,

    #[serde(rename = "dockerfile")]
    Dockerfile,

    #[serde(rename = "elixir")]
    Elixir,

    #[serde(rename = "erlang")]
    Erlang,

    #[serde(rename = "fsharp")]
    FSharp,

    #[serde(rename = "git-commit")]
    GitCommit,

    #[serde(rename = "rebase")]
    GitRebase,

    #[serde(rename = "go")]
    Go,

    #[serde(rename = "groovy")]
    Groovy,

    #[serde(rename = "handlebars")]
    Handlebars,

    #[serde(rename = "haskell")]
    Haskell,

    #[serde(rename = "html")]
    Html,

    #[serde(rename = "ini")]
    Ini,

    #[serde(rename = "java")]
    Java,

    #[serde(rename = "javascript")]
    JavaScript,

    #[serde(rename = "javascriptreact")]
    JavaScriptReact,

    #[serde(rename = "json")]
    Json,

    #[serde(rename = "latex")]
    LaTeX,

    #[serde(rename = "less")]
    Less,

    #[serde(rename = "lua")]
    Lua,

    #[serde(rename = "makefile")]
    Makefile,

    #[serde(rename = "markdown")]
    Markdown,

    #[serde(rename = "objective-c")]
    ObjectiveC,

    #[serde(rename = "objective-cpp")]
    ObjectiveCpp,

    #[cfg(feature = "proposed")]
    #[serde(rename = "pascal")]
    /// @since 3.18.0
    /// @proposed
    Pascal,

    #[serde(rename = "perl")]
    Perl,

    #[serde(rename = "perl6")]
    Perl6,

    #[serde(rename = "php")]
    Php,

    #[serde(rename = "powershell")]
    Powershell,

    #[serde(rename = "jade")]
    Pug,

    #[serde(rename = "python")]
    Python,

    #[serde(rename = "r")]
    R,

    #[serde(rename = "razor")]
    Razor,

    #[serde(rename = "ruby")]
    Ruby,

    #[serde(rename = "rust")]
    Rust,

    #[serde(rename = "scss")]
    Scss,

    #[serde(rename = "sass")]
    Sass,

    #[serde(rename = "scala")]
    Scala,

    #[serde(rename = "shaderlab")]
    ShaderLab,

    #[serde(rename = "shellscript")]
    ShellScript,

    #[serde(rename = "sql")]
    Sql,

    #[serde(rename = "swift")]
    Swift,

    #[serde(rename = "typescript")]
    TypeScript,

    #[serde(rename = "typescriptreact")]
    TypeScriptReact,

    #[serde(rename = "tex")]
    TeX,

    #[serde(rename = "vb")]
    VisualBasic,

    #[serde(rename = "xml")]
    Xml,

    #[serde(rename = "xsl")]
    Xsl,

    #[serde(rename = "yaml")]
    Yaml,
}

#[cfg(feature = "proposed")]
#[derive(Clone, Debug, PartialEq, Eq)]
/// Describes how an {@link InlineCompletionItemProvider inline completion provider} was triggered.
///
/// @since 3.18.0
/// @proposed
pub enum InlineCompletionTriggerKind {
    /// Completion was triggered explicitly by a user gesture.
    Invoked = 1,

    /// Completion was triggered automatically while editing.
    Automatic = 2,
}
impl Serialize for InlineCompletionTriggerKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            InlineCompletionTriggerKind::Invoked => serializer.serialize_u32(1),
            InlineCompletionTriggerKind::Automatic => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for InlineCompletionTriggerKind {
    fn deserialize<D>(deserializer: D) -> Result<InlineCompletionTriggerKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(InlineCompletionTriggerKind::Invoked),
            2 => Ok(InlineCompletionTriggerKind::Automatic),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A set of predefined position encoding kinds.
///
/// @since 3.17.0
pub enum PositionEncodingKind {
    #[serde(rename = "utf-8")]
    /// Character offsets count UTF-8 code units (e.g. bytes).
    Utf8,

    #[serde(rename = "utf-16")]
    /// Character offsets count UTF-16 code units.
    ///
    /// This is the default and must always be supported
    /// by servers
    Utf16,

    #[serde(rename = "utf-32")]
    /// Character offsets count UTF-32 code units.
    ///
    /// Implementation note: these are the same as Unicode codepoints,
    /// so this `PositionEncodingKind` may also be used for an
    /// encoding-agnostic representation of character offsets.
    Utf32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The file event type
pub enum FileChangeType {
    /// The file got created.
    Created = 1,

    /// The file got changed.
    Changed = 2,

    /// The file got deleted.
    Deleted = 3,
}
impl Serialize for FileChangeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            FileChangeType::Created => serializer.serialize_u32(1),
            FileChangeType::Changed => serializer.serialize_u32(2),
            FileChangeType::Deleted => serializer.serialize_u32(3),
        }
    }
}
impl<'de> Deserialize<'de> for FileChangeType {
    fn deserialize<D>(deserializer: D) -> Result<FileChangeType, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(FileChangeType::Created),
            2 => Ok(FileChangeType::Changed),
            3 => Ok(FileChangeType::Deleted),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WatchKind {
    /// Interested in create events.
    Create = 1,

    /// Interested in change events
    Change = 2,

    /// Interested in delete events
    Delete = 4,
}
impl Serialize for WatchKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            WatchKind::Create => serializer.serialize_u32(1),
            WatchKind::Change => serializer.serialize_u32(2),
            WatchKind::Delete => serializer.serialize_u32(4),
        }
    }
}
impl<'de> Deserialize<'de> for WatchKind {
    fn deserialize<D>(deserializer: D) -> Result<WatchKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(WatchKind::Create),
            2 => Ok(WatchKind::Change),
            4 => Ok(WatchKind::Delete),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 4",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The diagnostic's severity.
pub enum DiagnosticSeverity {
    /// Reports an error.
    Error = 1,

    /// Reports a warning.
    Warning = 2,

    /// Reports an information.
    Information = 3,

    /// Reports a hint.
    Hint = 4,
}
impl Serialize for DiagnosticSeverity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DiagnosticSeverity::Error => serializer.serialize_u32(1),
            DiagnosticSeverity::Warning => serializer.serialize_u32(2),
            DiagnosticSeverity::Information => serializer.serialize_u32(3),
            DiagnosticSeverity::Hint => serializer.serialize_u32(4),
        }
    }
}
impl<'de> Deserialize<'de> for DiagnosticSeverity {
    fn deserialize<D>(deserializer: D) -> Result<DiagnosticSeverity, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(DiagnosticSeverity::Error),
            2 => Ok(DiagnosticSeverity::Warning),
            3 => Ok(DiagnosticSeverity::Information),
            4 => Ok(DiagnosticSeverity::Hint),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3, 4",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The diagnostic tags.
///
/// @since 3.15.0
pub enum DiagnosticTag {
    /// Unused or unnecessary code.
    ///
    /// Clients are allowed to render diagnostics with this tag faded out instead of having
    /// an error squiggle.
    Unnecessary = 1,

    /// Deprecated or obsolete code.
    ///
    /// Clients are allowed to rendered diagnostics with this tag strike through.
    Deprecated = 2,
}
impl Serialize for DiagnosticTag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DiagnosticTag::Unnecessary => serializer.serialize_u32(1),
            DiagnosticTag::Deprecated => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for DiagnosticTag {
    fn deserialize<D>(deserializer: D) -> Result<DiagnosticTag, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(DiagnosticTag::Unnecessary),
            2 => Ok(DiagnosticTag::Deprecated),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// How a completion was triggered
pub enum CompletionTriggerKind {
    /// Completion was triggered by typing an identifier (24x7 code
    /// complete), manual invocation (e.g Ctrl+Space) or via API.
    Invoked = 1,

    /// Completion was triggered by a trigger character specified by
    /// the `triggerCharacters` properties of the `CompletionRegistrationOptions`.
    TriggerCharacter = 2,

    /// Completion was re-triggered as current completion list is incomplete
    TriggerForIncompleteCompletions = 3,
}
impl Serialize for CompletionTriggerKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CompletionTriggerKind::Invoked => serializer.serialize_u32(1),
            CompletionTriggerKind::TriggerCharacter => serializer.serialize_u32(2),
            CompletionTriggerKind::TriggerForIncompleteCompletions => serializer.serialize_u32(3),
        }
    }
}
impl<'de> Deserialize<'de> for CompletionTriggerKind {
    fn deserialize<D>(deserializer: D) -> Result<CompletionTriggerKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CompletionTriggerKind::Invoked),
            2 => Ok(CompletionTriggerKind::TriggerCharacter),
            3 => Ok(CompletionTriggerKind::TriggerForIncompleteCompletions),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Defines how values from a set of defaults and an individual item will be
/// merged.
///
/// @since 3.18.0
pub enum ApplyKind {
    /// The value from the individual item (if provided and not `null`) will be
    /// used instead of the default.
    Replace = 1,

    /// The value from the item will be merged with the default.
    ///
    /// The specific rules for mergeing values are defined against each field
    /// that supports merging.
    Merge = 2,
}
impl Serialize for ApplyKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ApplyKind::Replace => serializer.serialize_u32(1),
            ApplyKind::Merge => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for ApplyKind {
    fn deserialize<D>(deserializer: D) -> Result<ApplyKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(ApplyKind::Replace),
            2 => Ok(ApplyKind::Merge),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// How a signature help was triggered.
///
/// @since 3.15.0
pub enum SignatureHelpTriggerKind {
    /// Signature help was invoked manually by the user or by a command.
    Invoked = 1,

    /// Signature help was triggered by a trigger character.
    TriggerCharacter = 2,

    /// Signature help was triggered by the cursor moving or by the document content changing.
    ContentChange = 3,
}
impl Serialize for SignatureHelpTriggerKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SignatureHelpTriggerKind::Invoked => serializer.serialize_u32(1),
            SignatureHelpTriggerKind::TriggerCharacter => serializer.serialize_u32(2),
            SignatureHelpTriggerKind::ContentChange => serializer.serialize_u32(3),
        }
    }
}
impl<'de> Deserialize<'de> for SignatureHelpTriggerKind {
    fn deserialize<D>(deserializer: D) -> Result<SignatureHelpTriggerKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(SignatureHelpTriggerKind::Invoked),
            2 => Ok(SignatureHelpTriggerKind::TriggerCharacter),
            3 => Ok(SignatureHelpTriggerKind::ContentChange),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2, 3",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// The reason why code actions were requested.
///
/// @since 3.17.0
pub enum CodeActionTriggerKind {
    /// Code actions were explicitly requested by the user or by an extension.
    Invoked = 1,

    /// Code actions were requested automatically.
    ///
    /// This typically happens when current selection in a file changes, but can
    /// also be triggered when file content changes.
    Automatic = 2,
}
impl Serialize for CodeActionTriggerKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            CodeActionTriggerKind::Invoked => serializer.serialize_u32(1),
            CodeActionTriggerKind::Automatic => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for CodeActionTriggerKind {
    fn deserialize<D>(deserializer: D) -> Result<CodeActionTriggerKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(CodeActionTriggerKind::Invoked),
            2 => Ok(CodeActionTriggerKind::Automatic),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
/// A pattern kind describing if a glob pattern matches a file a folder or
/// both.
///
/// @since 3.16.0
pub enum FileOperationPatternKind {
    #[serde(rename = "file")]
    /// The pattern matches a file only.
    File,

    #[serde(rename = "folder")]
    /// The pattern matches a folder only.
    Folder,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// A notebook cell kind.
///
/// @since 3.17.0
pub enum NotebookCellKind {
    /// A markup-cell is formatted source that is used for display.
    Markup = 1,

    /// A code-cell is source code.
    Code = 2,
}
impl Serialize for NotebookCellKind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            NotebookCellKind::Markup => serializer.serialize_u32(1),
            NotebookCellKind::Code => serializer.serialize_u32(2),
        }
    }
}
impl<'de> Deserialize<'de> for NotebookCellKind {
    fn deserialize<D>(deserializer: D) -> Result<NotebookCellKind, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(NotebookCellKind::Markup),
            2 => Ok(NotebookCellKind::Code),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1, 2",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceOperationKind {
    #[serde(rename = "create")]
    /// Supports creating new files and folders.
    Create,

    #[serde(rename = "rename")]
    /// Supports renaming existing files and folders.
    Rename,

    #[serde(rename = "delete")]
    /// Supports deleting existing files and folders.
    Delete,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum FailureHandlingKind {
    #[serde(rename = "abort")]
    /// Applying the workspace change is simply aborted if one of the changes provided
    /// fails. All operations executed before the failing operation stay executed.
    Abort,

    #[serde(rename = "transactional")]
    /// All operations are executed transactional. That means they either all
    /// succeed or no changes at all are applied to the workspace.
    Transactional,

    #[serde(rename = "textOnlyTransactional")]
    /// If the workspace edit contains only textual file changes they are executed transactional.
    /// If resource changes (create, rename or delete file) are part of the change the failure
    /// handling strategy is abort.
    TextOnlyTransactional,

    #[serde(rename = "undo")]
    /// The client tries to undo the operations already executed. But there is no
    /// guarantee that this is succeeding.
    Undo,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrepareSupportDefaultBehavior {
    /// The client's default behavior is to select the identifier
    /// according the to language's syntax rule.
    Identifier = 1,
}
impl Serialize for PrepareSupportDefaultBehavior {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PrepareSupportDefaultBehavior::Identifier => serializer.serialize_u32(1),
        }
    }
}
impl<'de> Deserialize<'de> for PrepareSupportDefaultBehavior {
    fn deserialize<D>(deserializer: D) -> Result<PrepareSupportDefaultBehavior, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(PrepareSupportDefaultBehavior::Identifier),
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &"one of 1",
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenFormat {
    #[serde(rename = "relative")]
    Relative,
}
