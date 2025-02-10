use serde::Deserialize;
use std::env;

fn main() -> anyhow::Result<()> {
    let agent = if let Ok(proxy) = env::var("https_proxy") {
        let proxy = ureq::Proxy::new(proxy)?;
        ureq::AgentBuilder::new().proxy(proxy).build()
    } else {
        ureq::Agent::new()
    };
    let lsp_def = agent
        .get("https://raw.githubusercontent.com/microsoft/lsprotocol/refs/heads/main/generator/lsp.json")
        .call()?
        .into_json::<LspDef>()?;
    dbg!(lsp_def);
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LspDef {
    requests: Vec<Request>,
    notifications: Vec<Notification>,
    structures: Vec<Structure>,
    enumerations: Vec<Enumeration>,
    type_aliases: Vec<TypeAlias>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Request {
    method: String,
    type_name: String,
    params: Option<TypeDef>,
    result: Option<TypeDef>,
    partial_result: Option<TypeDef>,
    registration_method: Option<String>,
    registration_options: Option<TypeDef>,
    documentation: Option<String>,
    #[serde(default)]
    proposed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Notification {
    method: String,
    type_name: String,
    params: Option<TypeDef>,
    registration_method: Option<String>,
    registration_options: Option<TypeDef>,
    documentation: Option<String>,
    #[serde(default)]
    proposed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Structure {
    name: String,
    #[serde(default)]
    properties: Vec<Property>,
    #[serde(default)]
    extends: Vec<TypeDef>,
    #[serde(default)]
    mixins: Vec<TypeDef>,
    documentation: Option<String>,
    deprecated: Option<String>,
    #[serde(default)]
    proposed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Property {
    name: String,
    #[serde(rename = "type")]
    ty: TypeDef,
    #[serde(default)]
    optional: bool,
    documentation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Enumeration {
    name: String,
    values: EnumerationValues,
    documentation: Option<String>,
    deprecated: Option<String>,
    #[serde(default)]
    proposed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum EnumerationValues {
    Int(Vec<EnumerationIntValue>),
    Str(Vec<EnumerationStrValue>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EnumerationStrValue {
    name: String,
    value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EnumerationIntValue {
    name: String,
    value: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TypeAlias {
    name: String,
    #[serde(rename = "type")]
    ty: TypeDef,
    documentation: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
enum TypeDef {
    Base {
        name: BaseType,
    },
    #[serde(rename = "reference")]
    Ref {
        name: String,
    },
    Map {
        key: Box<TypeDef>,
        value: Box<TypeDef>,
    },
    Or {
        items: Vec<TypeDef>,
    },
    And {
        items: Vec<TypeDef>,
    },
    Array {
        element: Box<TypeDef>,
    },
    Tuple {
        items: Vec<TypeDef>,
    },
    Literal,
    StringLiteral {
        value: String,
    },
}

#[derive(Debug, Deserialize)]
enum BaseType {
    #[serde(rename = "null")]
    Null,
    #[serde(rename = "uinteger")]
    Uinteger,
    #[serde(rename = "integer")]
    Integer,
    #[serde(rename = "string")]
    String,
    #[serde(rename = "boolean")]
    Boolean,
    DocumentUri,
    URI,
    #[serde(rename = "decimal")]
    Decimal,
}
