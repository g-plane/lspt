use heck::{ToSnakeCase, ToUpperCamelCase};
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
        "./lspt/src/generated/request.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

use serde::Serialize;
use super::*;

pub trait Request {{
    const METHOD: &'static str;
    type Params: serde::de::DeserializeOwned + Serialize + Send + Sync + 'static;
}}
{}",
            gen_requests(&lsp_def),
        ),
    )?;

    fs::write(
        "./lspt/src/generated/notification.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

use serde::Serialize;
use super::*;

pub trait Notification {{
    const METHOD: &'static str;
    type Params: serde::de::DeserializeOwned + Serialize + Send + Sync + 'static;
}}
{}",
            gen_notifications(&lsp_def),
        ),
    )?;

    fs::write(
        "./lspt/src/generated/structs.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

use crate::{{HashMap, Union2, Union3, Union4, Uri}};
use serde::{{Deserialize, Serialize}};
use super::*;
{}",
            gen_structs(&lsp_def),
        ),
    )?;

    fs::write(
        "./lspt/src/generated/enums.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

use serde::{{Deserialize, Deserializer, Serialize, Serializer}};

{}
",
            gen_enums(&lsp_def),
        ),
    )?;

    fs::write(
        "./lspt/src/generated/type_aliases.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

use super::*;
use crate::{{Union2, Union3}};
{}",
            gen_type_aliases(&lsp_def),
        ),
    )?;

    Ok(())
}

fn gen_requests(lsp_def: &LspDef) -> String {
    lsp_def.requests.iter().fold(String::new(), |mut output, request| {
        if request.proposed {
            output.push_str("\n#[cfg(feature = \"proposed\")]");
        }
        let _ = write!(output, "\npub enum {} {{}}", request.type_name);
        if request.proposed {
            output.push_str("\n#[cfg(feature = \"proposed\")]");
        }
        let _ = write!(
            output,
            "
impl Request for {} {{
    const METHOD: &'static str = \"{}\";
    type Params = {};
}}
",
            request.type_name,
            request.method,
            if let Some(TypeRef { name }) = &request.params {
                if name == "LSPAny" {
                    "serde_json::Value"
                } else {
                    name
                }
            } else {
                "()"
            }
        );
        output
    })
}

fn gen_notifications(lsp_def: &LspDef) -> String {
    lsp_def
        .notifications
        .iter()
        .fold(String::new(), |mut output, notification| {
            if notification.proposed {
                output.push_str("\n#[cfg(feature = \"proposed\")]");
            }
            let _ = write!(output, "\npub enum {} {{}}", notification.type_name);
            if notification.proposed {
                output.push_str("\n#[cfg(feature = \"proposed\")]");
            }
            let _ = write!(
                output,
                "
impl Notification for {} {{
    const METHOD: &'static str = \"{}\";
    type Params = {};
}}
",
                notification.type_name,
                notification.method,
                if let Some(TypeRef { name }) = &notification.params {
                    if name == "LSPAny" {
                        "serde_json::Value"
                    } else {
                        name
                    }
                } else {
                    "()"
                }
            );
            output
        })
}

fn gen_structs(lsp_def: &LspDef) -> String {
    lsp_def
        .structures
        .iter()
        .filter(|structure| {
            !matches!(
                &*structure.name,
                "TextDocumentPositionParams"
                    | "_InitializeParams"
                    | "TextDocumentRegistrationOptions"
                    | "WorkDoneProgressOptions"
                    | "PartialResultParams"
                    | "StaticRegistrationOptions"
            )
        })
        .fold(String::new(), |mut output, structure| {
            if structure.proposed {
                output.push_str("\n#[cfg(feature = \"proposed\")]");
            }
            if let Some(deprecated) = &structure.deprecated {
                let _ = write!(output, "\n#[deprecated = \"{deprecated}\"]");
            }
            let default = if can_derive_default(structure, lsp_def) {
                "Default, "
            } else {
                ""
            };
            let additional_derives = match &*structure.name {
                "Position" => ", Copy, PartialOrd, Ord",
                "Range" => ", Copy",
                _ => "",
            };
            let _ = write!(
                output,
                "\n#[derive(Clone, Debug, {default}PartialEq, Eq, Serialize, Deserialize{additional_derives})]"
            );
            output.push_str("\n#[serde(rename_all = \"camelCase\")]");
            output.push_str(&gen_doc(structure.documentation.as_deref(), 0));
            let _ = write!(output, "\npub struct {} {{", structure.name);
            get_extends(structure, lsp_def)
                .iter()
                .chain(structure.properties.iter())
                .chain(get_mixins(structure, lsp_def).iter())
                .fold(&mut output, |output, property| {
                    if property.proposed {
                        output.push_str("\n    #[cfg(feature = \"proposed\")]");
                    }
                    if let Some(deprecated) = &property.deprecated {
                        let _ = write!(output, "\n    #[deprecated = \"{deprecated}\"]");
                    }
                    let mut name = property.name.to_snake_case();
                    if name == "type" {
                        output.push_str("\n    #[serde(rename = \"type\")]");
                        name = "ty".into();
                    }
                    let mut optional = property.optional;
                    let type_def = if let TypeDef::Or { items } = &property.ty {
                        let filtered_items = items
                            .iter()
                            .filter(|item| !matches!(item, TypeDef::Base { name: BaseType::Null }))
                            .collect::<Vec<_>>();
                        if filtered_items.len() == items.len() {
                            property.ty.clone()
                        } else {
                            optional = true;
                            TypeDef::Or {
                                items: filtered_items.into_iter().cloned().collect(),
                            }
                        }
                    } else {
                        property.ty.clone()
                    };
                    let mut ty = gen_type_def(&type_def);
                    match &type_def {
                        TypeDef::Ref(TypeRef { name }) if name == &structure.name => {
                            ty = format!("Box<{ty}>");
                        }
                        _ => {}
                    }
                    if optional {
                        output.push_str("\n    #[serde(skip_serializing_if = \"Option::is_none\")]");
                        output.push_str(&gen_doc(property.documentation.as_deref(), 4));
                        let _ = write!(output, "\n    pub {name}: Option<{ty}>,");
                    } else {
                        output.push_str(&gen_doc(property.documentation.as_deref(), 4));
                        let _ = write!(output, "\n    pub {name}: {ty},");
                    }
                    output.push('\n');
                    output
                });
            output.push_str("}\n");
            output
        })
        .replace(
            "Vec<Union3<TextEdit, AnnotatedTextEdit, SnippetTextEdit>>",
            "Vec<Union2<TextEdit, AnnotatedTextEdit>>",
        ) // hard-coded workaround
}
fn can_derive_default(structure: &Structure, lsp_def: &LspDef) -> bool {
    !structure
        .extends
        .iter()
        .any(|extend| matches!(&*extend.name, "TextDocumentIdentifier" | "TextDocumentPositionParams"))
        && structure.properties.iter().all(|prop| {
            prop.optional
                || match &prop.ty {
                    TypeDef::Ref(TypeRef { name }) => {
                        lsp_def.enumerations.iter().all(|enumeration| &enumeration.name != name)
                            && lsp_def
                                .structures
                                .iter()
                                .find(|structure| &structure.name == name)
                                .is_some_and(|structure| can_derive_default(structure, lsp_def))
                    }
                    TypeDef::Or { items } => {
                        items
                            .iter()
                            .filter(|item| !matches!(item, TypeDef::Base { name: BaseType::Null }))
                            .count()
                            <= 1
                    }
                    TypeDef::Base {
                        name: BaseType::DocumentUri | BaseType::Uri,
                    } => false,
                    _ => true,
                }
        })
}
fn get_extends(structure: &Structure, lsp_def: &LspDef) -> Vec<Property> {
    structure
        .extends
        .iter()
        .filter_map(|extend| lsp_def.structures.iter().find(|it| it.name == extend.name))
        .fold(Vec::new(), |mut output, extend| {
            output.append(
                &mut extend
                    .properties
                    .iter()
                    .filter(|extend_property| {
                        !structure
                            .properties
                            .iter()
                            .any(|property| property.name == extend_property.name)
                    })
                    .cloned()
                    .collect(),
            );
            output.append(&mut get_extends(extend, lsp_def));
            output
        })
}
fn get_mixins(structure: &Structure, lsp_def: &LspDef) -> Vec<Property> {
    structure
        .mixins
        .iter()
        .filter_map(|mixin| lsp_def.structures.iter().find(|it| it.name == mixin.name))
        .fold(Vec::new(), |mut output, mixin| {
            output.append(&mut mixin.properties.clone());
            output.append(&mut get_mixins(mixin, lsp_def));
            output
        })
}

fn gen_type_aliases(lsp_def: &LspDef) -> String {
    lsp_def
        .type_aliases
        .iter()
        .filter(|type_alias| !type_alias.name.starts_with("LSP"))
        .fold(String::new(), |mut output, type_alias| {
            output.push_str(&gen_doc(type_alias.documentation.as_deref(), 0));
            let _ = write!(
                output,
                "\npub type {} = {};",
                type_alias.name,
                gen_type_def(&type_alias.ty)
            );
            output.push('\n');
            output
        })
}

fn gen_enums(lsp_def: &LspDef) -> String {
    lsp_def
        .enumerations
        .iter()
        .map(|enumeration| {
            let deprecated = if let Some(deprecated) = &enumeration.deprecated {
                format!("#[deprecated = \"{deprecated}\"]\n")
            } else {
                "".into()
            };
            match &enumeration.values {
                EnumerationValues::Str(values) => {
                    let mut variants = values
                        .iter()
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
                    if enumeration.supports_custom_values {
                        variants.push_str("\n    #[serde(untagged)]\n    Custom_(String),\n");
                    }
                    format!(
                        "{deprecated}#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]{}\npub enum {} {{\n{}}}",
                        gen_doc(enumeration.documentation.as_deref(), 0),
                        enumeration.name,
                        variants
                    )
                }
                EnumerationValues::Int(values) => {
                    let name = &enumeration.name;
                    let values = values
                        .iter()
                        .map(|value| {
                            let mut value = value.clone();
                            value.name = value.name.to_upper_camel_case();
                            value
                        })
                        .collect::<Vec<_>>();
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
                    let additional_derives = match &*enumeration.name {
                        "DiagnosticSeverity" => ", Copy",
                        _ => "",
                    };
                    let enum_def = format!(
                        "{deprecated}#[derive(Clone, Debug, PartialEq, Eq{additional_derives})]{}\npub enum {name} {{{}}}",
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
            }
        })
        .join("\n\n")
}

fn gen_type_def(type_def: &TypeDef) -> String {
    match type_def {
        TypeDef::Base { name } => gen_base_type(name).into(),
        TypeDef::Ref(TypeRef { name }) => match &**name {
            "LSPAny" => "serde_json::Value",
            "LSPObject" => "HashMap<String, serde_json::Value>",
            name => name,
        }
        .into(),
        TypeDef::Map { key, value } => format!("HashMap<{}, {}>", gen_type_def(key), gen_type_def(value)),
        TypeDef::Or { items } => {
            if items.len() == 1 {
                gen_type_def(items.first().unwrap())
            } else {
                format!("Union{}<{}>", items.len(), items.iter().map(gen_type_def).join(", "))
            }
        }
        TypeDef::Array { element } => format!("Vec<{}>", gen_type_def(element)),
        TypeDef::Tuple { items } => format!("({})", items.iter().map(gen_type_def).join(", ")),
        TypeDef::Literal => "serde_json::Value".into(),
        TypeDef::StringLiteral { .. } => "String".into(),
        _ => unreachable!(),
    }
}

fn gen_base_type(base_type: &BaseType) -> &'static str {
    match base_type {
        BaseType::Null => "serde_json::Value",
        BaseType::Uinteger => "u32",
        BaseType::Integer | BaseType::Decimal => "i32",
        BaseType::String => "String",
        BaseType::Boolean => "bool",
        BaseType::DocumentUri | BaseType::Uri => "Uri",
    }
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
    params: Option<TypeRef>,
    #[serde(default)]
    proposed: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Notification {
    method: String,
    type_name: String,
    params: Option<TypeRef>,
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
    extends: Vec<TypeRef>,
    #[serde(default)]
    mixins: Vec<TypeRef>,
    documentation: Option<String>,
    deprecated: Option<String>,
    #[serde(default)]
    proposed: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Property {
    name: String,
    #[serde(rename = "type")]
    ty: TypeDef,
    #[serde(default)]
    optional: bool,
    documentation: Option<String>,
    deprecated: Option<String>,
    #[serde(default)]
    proposed: bool,
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
    supports_custom_values: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum EnumerationValues {
    Int(Vec<EnumerationIntValue>),
    Str(Vec<EnumerationStrValue>),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EnumerationStrValue {
    name: String,
    value: String,
    documentation: Option<String>,
    #[serde(default)]
    proposed: bool,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
struct TypeRef {
    name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
enum TypeDef {
    Base {
        name: BaseType,
    },
    #[serde(rename = "reference")]
    Ref(TypeRef),
    Map {
        key: Box<TypeDef>,
        value: Box<TypeDef>,
    },
    Or {
        items: Vec<TypeDef>,
    },
    And,
    Array {
        element: Box<TypeDef>,
    },
    Tuple {
        items: Vec<TypeDef>,
    },
    Literal,
    StringLiteral,
}

#[derive(Clone, Debug, Deserialize)]
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
