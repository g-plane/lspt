use heck::ToUpperCamelCase;
use itertools::Itertools;
use serde::Deserialize;
use std::{env, fmt::Write, fs};

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

    fs::write(
        "./lspt/src/enums.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

use serde::{{Deserialize, Deserializer, Serialize, Serializer}};

{}
",
            gen_enums(lsp_def.enumerations)
        ),
    )?;

    Ok(())
}

fn gen_enums(enumerations: Vec<Enumeration>) -> String {
    enumerations
        .into_iter()
        .map(|enumeration| match enumeration.values {
            EnumerationValues::Str(values) => {
                let variants = values
                    .into_iter()
                    .map(|value| {
                        format!(
                            "{}    #[serde(rename = \"{}\")]{}\n    {},\n",
                            if value.proposed {
                                "    #[cfg(feature = \"proposed\")]\n"
                            } else {
                                ""
                            },
                            value.value,
                            gen_doc(value.documentation.as_deref(), 4),
                            value.name.to_upper_camel_case()
                        )
                    })
                    .join("\n");
                format!(
                    "#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]{}\npub enum {} {{\n{}}}",
                    gen_doc(enumeration.documentation.as_deref(), 0),
                    enumeration.name,
                    variants
                )
            }
            EnumerationValues::Int(mut values) => {
                let name = enumeration.name;
                values.iter_mut().for_each(|value| {
                    value.name = value.name.to_upper_camel_case();
                });
                let variants = values.iter().fold(String::new(), |mut output, value| {
                    let _ = write!(
                        output,
                        "{}{}\n    {} = {},\n",
                        if value.proposed {
                            "\n    #[cfg(feature = \"proposed\")]"
                        } else {
                            ""
                        },
                        gen_doc(value.documentation.as_deref(), 4),
                        value.name,
                        value.value,
                    );
                    output
                });
                let enum_def = format!(
                    "#[derive(Clone, Debug, PartialEq, Eq)]{}\npub enum {name} {{{}}}",
                    gen_doc(enumeration.documentation.as_deref(), 0),
                    variants
                );
                let ty = if let TypeDef::Base {
                    name: BaseType::Uinteger,
                } = enumeration.ty
                {
                    "u32"
                } else {
                    "i32"
                };
                let ser = values.iter().fold(String::new(), |mut output, value| {
                    let _ = write!(
                        output,
                        "{}\n{}{name}::{} => serializer.serialize_{ty}({}),",
                        if value.proposed {
                            "\n            #[cfg(feature = \"proposed\")]"
                        } else {
                            ""
                        },
                        " ".repeat(12),
                        value.name,
                        value.value,
                    );
                    output
                });
                let de = values.iter().fold(String::new(), |mut output, value| {
                    let _ = write!(
                        output,
                        "{}\n{}{} => Ok({name}::{}),",
                        if value.proposed {
                            "\n            #[cfg(feature = \"proposed\")]"
                        } else {
                            ""
                        },
                        " ".repeat(12),
                        value.value,
                        value.name,
                    );
                    output
                });
                let expected = format!("one of {}", values.iter().map(|value| value.value).join(", "));
                format!(
                    "{enum_def}
impl Serialize for {name} {{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {{
        match self {{{ser}
        }}
    }}
}}
impl<'de> Deserialize<'de> for {name} {{
    fn deserialize<D>(deserializer: D) -> Result<{name}, D::Error>
    where
        D: Deserializer<'de>,
    {{
        let value = {ty}::deserialize(deserializer)?;
        match value {{{de}
            value => Err(serde::de::Error::invalid_value(
                serde::de::Unexpected::Signed(value as i64),
                &\"{expected}\",
            )),
        }}
    }}
}}",
                )
            }
        })
        .join("\n\n")
}

fn gen_doc(doc: Option<&str>, indent: usize) -> String {
    doc.map(|doc| {
        doc.lines().fold(String::new(), |mut output, line| {
            let _ = write!(
                output,
                "\n{}///{}{line}",
                " ".repeat(indent),
                if line.is_empty() { "" } else { " " }
            );
            output
        })
    })
    .unwrap_or_default()
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
    #[serde(rename = "type")]
    ty: TypeDef,
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
    documentation: Option<String>,
    #[serde(default)]
    proposed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EnumerationIntValue {
    name: String,
    value: i32,
    documentation: Option<String>,
    #[serde(default)]
    proposed: bool,
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
    #[serde(rename = "URI")]
    Uri,
    #[serde(rename = "decimal")]
    Decimal,
}
