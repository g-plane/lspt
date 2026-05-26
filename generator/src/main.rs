use heck::{ToSnakeCase, ToUpperCamelCase};
use indexmap::IndexSet;
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
    let mut unions = UnionRegistry::default();

    let type_aliases = gen_type_aliases(&lsp_def, &mut unions);
    let requests = gen_requests(&lsp_def, &mut unions);
    let notifications = gen_notifications(&lsp_def);
    let structs = gen_structs(&lsp_def, &mut unions);
    let enums = gen_enums(&lsp_def);
    let unions = gen_unions(&unions);

    fs::write(
        "./lspt/src/generated/request.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

use serde::Serialize;
use super::*;

pub trait Request {{
    const METHOD: &'static str;
    type Params: serde::de::DeserializeOwned + Serialize + Send + Sync + 'static;
    type Result: serde::de::DeserializeOwned + Serialize + Send + Sync + 'static;
}}
{}
{}",
            gen_request_macros(&lsp_def),
            requests,
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
{}
{}",
            gen_notification_macros(&lsp_def),
            notifications,
        ),
    )?;

    fs::write(
        "./lspt/src/generated/structs.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

use crate::{{HashMap, Uri}};
use serde::{{Deserialize, Serialize}};
use super::*;
{}",
            structs,
        ),
    )?;

    fs::write(
        "./lspt/src/generated/enums.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

use serde::{{Deserialize, Deserializer, Serialize, Serializer}};

{}
",
            enums,
        ),
    )?;

    fs::write(
        "./lspt/src/generated/unions.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

#![allow(deprecated)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::large_enum_variant)]
#![allow(rustdoc::invalid_codeblock_attributes)]
#![allow(unused_imports)]

use crate::{{HashMap, Uri}};
use serde::{{Deserialize, Serialize}};
use super::*;

{}",
            unions,
        ),
    )?;

    fs::write(
        "./lspt/src/generated/type_aliases.rs",
        format!(
            "// DO NOT EDIT THIS GENERATED FILE.

use super::*;
{}",
            type_aliases,
        ),
    )?;

    Ok(())
}

fn gen_request_macros(lsp_def: &LspDef) -> String {
    let stable_arms = gen_method_macro_arms(
        lsp_def
            .requests
            .iter()
            .filter(|request| !request.proposed)
            .map(|request| (&request.method, &request.type_name)),
        "request",
    );
    let all_arms = gen_method_macro_arms(
        lsp_def
            .requests
            .iter()
            .map(|request| (&request.method, &request.type_name)),
        "request",
    );

    format!(
        r#"
#[cfg(all(feature = "macros", not(feature = "proposed")))]
#[macro_export]
macro_rules! lsp_request {{
{stable_arms}}}

#[cfg(all(feature = "macros", feature = "proposed"))]
#[macro_export]
macro_rules! lsp_request {{
{all_arms}}}
"#
    )
}

fn gen_notification_macros(lsp_def: &LspDef) -> String {
    let stable_arms = gen_method_macro_arms(
        lsp_def
            .notifications
            .iter()
            .filter(|notification| !notification.proposed)
            .map(|notification| (&notification.method, &notification.type_name)),
        "notification",
    );
    let all_arms = gen_method_macro_arms(
        lsp_def
            .notifications
            .iter()
            .map(|notification| (&notification.method, &notification.type_name)),
        "notification",
    );

    format!(
        r#"
#[cfg(all(feature = "macros", not(feature = "proposed")))]
#[macro_export]
macro_rules! lsp_notification {{
{stable_arms}}}

#[cfg(all(feature = "macros", feature = "proposed"))]
#[macro_export]
macro_rules! lsp_notification {{
{all_arms}}}
"#
    )
}

fn gen_method_macro_arms<'a>(methods: impl Iterator<Item = (&'a String, &'a String)>, module: &str) -> String {
    methods
        .map(|(method, type_name)| format!("    (\"{method}\") => {{ $crate::{module}::{type_name} }};\n"))
        .join("")
}

fn gen_requests(lsp_def: &LspDef, unions: &mut UnionRegistry) -> String {
    lsp_def.requests.iter().fold(String::new(), |mut output, request| {
        if request.proposed {
            output.push_str("\n#[cfg(feature = \"proposed\")]");
        }
        let _ = write!(output, "\npub enum {} {{}}", request.type_name);
        if request.proposed {
            output.push_str("\n#[cfg(feature = \"proposed\")]");
        }
        let result = gen_request_result(request, unions);
        let _ = write!(
            output,
            "
impl Request for {} {{
    const METHOD: &'static str = \"{}\";
    type Params = {};
    type Result = {result};
}}
",
            request.type_name,
            request.method,
            if let Some(TypeRef { name }) = &request.params {
                if name == "LSPAny" { "serde_json::Value" } else { name }
            } else {
                "()"
            }
        );
        output
    })
}

fn gen_request_result(request: &Request, unions: &mut UnionRegistry) -> String {
    let mut optional = false;
    let mut result_types = IndexSet::new();
    if let Some(result) = &request.result {
        extend_result_types(result, &mut result_types, &mut optional);
    }
    if let Some(partial_result) = &request.partial_result {
        extend_result_types(partial_result, &mut result_types, &mut optional);
    }

    if result_types.is_empty() {
        return "()".into();
    }

    let response_name = gen_request_response_name(&request.type_name);
    let result = gen_type_def(
        &TypeDef::Or {
            items: result_types.into_iter().collect(),
        },
        unions,
        &UnionContext::request_response(&response_name),
        request.proposed,
        None,
    );
    if optional { format!("Option<{result}>") } else { result }
}

fn extend_result_types(type_def: &TypeDef, result_types: &mut IndexSet<TypeDef>, optional: &mut bool) {
    match type_def {
        TypeDef::Or { items } => {
            for item in items {
                if matches!(item, TypeDef::Base { name: BaseType::Null }) {
                    *optional = true;
                } else {
                    result_types.insert(item.clone());
                }
            }
        }
        TypeDef::Base { name: BaseType::Null } => {}
        type_def => {
            result_types.insert(type_def.clone());
        }
    }
}

fn gen_request_response_name(type_name: &str) -> String {
    format!("{}Response", type_name.strip_suffix("Request").unwrap_or(type_name))
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
                    if name == "LSPAny" { "serde_json::Value" } else { name }
                } else {
                    "()"
                }
            );
            output
        })
}

fn gen_structs(lsp_def: &LspDef, unions: &mut UnionRegistry) -> String {
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
                .filter(|property| property.deprecated.is_none())
                .fold(&mut output, |output, property| {
                    if property.proposed {
                        output.push_str("\n    #[cfg(feature = \"proposed\")]");
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
                    let mut ty = gen_type_def(
                        &type_def,
                        unions,
                        &UnionContext::struct_field(&structure.name, &property.name),
                        structure.proposed || property.proposed,
                        None,
                    );
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

fn gen_type_aliases(lsp_def: &LspDef, unions: &mut UnionRegistry) -> String {
    lsp_def
        .type_aliases
        .iter()
        .filter(|type_alias| !type_alias.name.starts_with("LSP"))
        .fold(String::new(), |mut output, type_alias| {
            let ty = gen_type_def(
                &type_alias.ty,
                unions,
                &UnionContext::type_alias(&type_alias.name),
                false,
                type_alias.documentation.as_deref(),
            );
            if !is_multi_union(&type_alias.ty) {
                output.push_str(&gen_doc(type_alias.documentation.as_deref(), 0));
                let _ = write!(output, "\npub type {} = {ty};", type_alias.name);
                output.push('\n');
            }
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

#[derive(Default)]
struct UnionRegistry {
    names: IndexSet<String>,
    definitions: Vec<UnionDef>,
    aliases: Vec<UnionAlias>,
    entries: Vec<UnionEntry>,
}

struct UnionDef {
    requested_name: String,
    name: String,
    variants: Vec<UnionVariant>,
    documentation: Option<String>,
    proposed: bool,
}

struct UnionAlias {
    requested_name: String,
    name: String,
    target: String,
    documentation: Option<String>,
    proposed: bool,
}

enum UnionEntry {
    Definition(usize),
    Alias(usize),
}

#[derive(Clone, PartialEq, Eq)]
struct UnionVariant {
    name: String,
    original_name: Option<String>,
    ty: String,
    proposed: bool,
}

impl UnionRegistry {
    fn push(
        &mut self,
        name: String,
        variants: Vec<UnionVariant>,
        documentation: Option<&str>,
        proposed: bool,
    ) -> String {
        if let Some(target_name) = reusable_union_name(&variants) {
            self.push_reusable(name, target_name, variants, documentation, proposed)
        } else {
            self.push_definition(name, variants, documentation, proposed)
        }
    }

    fn push_reusable(
        &mut self,
        requested_name: String,
        target_name: &str,
        variants: Vec<UnionVariant>,
        documentation: Option<&str>,
        proposed: bool,
    ) -> String {
        let target_documentation = if requested_name == target_name {
            documentation
        } else {
            None
        };
        let target = self.push_definition(target_name.into(), variants, target_documentation, proposed);
        if requested_name == target {
            target
        } else {
            self.push_alias(requested_name, target, documentation, proposed)
        }
    }

    fn push_definition(
        &mut self,
        name: String,
        variants: Vec<UnionVariant>,
        documentation: Option<&str>,
        proposed: bool,
    ) -> String {
        let requested_name = name;
        if let Some(definition) = self
            .definitions
            .iter_mut()
            .find(|definition| definition.requested_name == requested_name && definition.variants == variants)
        {
            definition.proposed &= proposed;
            return definition.name.clone();
        }

        let name = self.unique_name(requested_name.clone());
        let index = self.definitions.len();
        self.definitions.push(UnionDef {
            requested_name,
            name: name.clone(),
            variants,
            documentation: documentation.map(str::to_string),
            proposed,
        });
        self.entries.push(UnionEntry::Definition(index));
        name
    }

    fn push_alias(
        &mut self,
        requested_name: String,
        target: String,
        documentation: Option<&str>,
        proposed: bool,
    ) -> String {
        if let Some(alias) = self
            .aliases
            .iter_mut()
            .find(|alias| alias.requested_name == requested_name && alias.target == target)
        {
            alias.proposed &= proposed;
            return alias.name.clone();
        }

        let name = self.unique_name(requested_name.clone());
        let index = self.aliases.len();
        self.aliases.push(UnionAlias {
            requested_name,
            name: name.clone(),
            target,
            documentation: documentation.map(str::to_string),
            proposed,
        });
        self.entries.push(UnionEntry::Alias(index));
        name
    }

    fn unique_name(&mut self, name: String) -> String {
        if self.names.insert(name.clone()) {
            return name;
        }

        panic!("duplicate generated union name `{name}`")
    }
}

struct ReusableUnion {
    name: &'static str,
    variants: &'static [ReusableUnionVariant],
}

struct ReusableUnionVariant {
    name: &'static str,
    ty: &'static str,
}

const REUSABLE_UNIONS: &[ReusableUnion] = &[
    ReusableUnion {
        name: "NumberOrString",
        variants: &[
            ReusableUnionVariant {
                name: "Integer",
                ty: "i32",
            },
            ReusableUnionVariant {
                name: "String",
                ty: "String",
            },
        ],
    },
    ReusableUnion {
        name: "StringOrMarkupContent",
        variants: &[
            ReusableUnionVariant {
                name: "String",
                ty: "String",
            },
            ReusableUnionVariant {
                name: "MarkupContent",
                ty: "MarkupContent",
            },
        ],
    },
];

fn reusable_union_name(variants: &[UnionVariant]) -> Option<&'static str> {
    REUSABLE_UNIONS
        .iter()
        .find(|reusable| {
            variants.len() == reusable.variants.len()
                && variants
                    .iter()
                    .zip(reusable.variants)
                    .all(|(variant, reusable_variant)| {
                        variant.name == reusable_variant.name && variant.ty == reusable_variant.ty
                    })
        })
        .map(|reusable| reusable.name)
}

#[derive(Clone)]
enum UnionContext {
    TypeAlias(String),
    RequestResponse(String),
    StructField { parent: String, field: String },
    MapKey(Box<UnionContext>),
    MapValue(Box<UnionContext>),
    ArrayItem(Box<UnionContext>),
    TupleItem { parent: Box<UnionContext>, index: usize },
    Variant { parent: Box<UnionContext>, name: String },
}

impl UnionContext {
    fn type_alias(name: &str) -> Self {
        Self::TypeAlias(name.into())
    }

    fn request_response(name: &str) -> Self {
        Self::RequestResponse(name.into())
    }

    fn struct_field(parent: &str, field: &str) -> Self {
        Self::StructField {
            parent: parent.into(),
            field: field.into(),
        }
    }

    fn map_key(&self) -> Self {
        Self::MapKey(Box::new(self.clone()))
    }

    fn map_value(&self) -> Self {
        Self::MapValue(Box::new(self.clone()))
    }

    fn array_item(&self) -> Self {
        Self::ArrayItem(Box::new(self.clone()))
    }

    fn tuple_item(&self, index: usize) -> Self {
        Self::TupleItem {
            parent: Box::new(self.clone()),
            index,
        }
    }

    fn variant(&self, name: &str) -> Self {
        Self::Variant {
            parent: Box::new(self.clone()),
            name: name.into(),
        }
    }

    fn union_name(&self) -> String {
        match self {
            Self::ArrayItem(parent) if parent.is_notebook_selector_field() => "NotebookSelectorItem".into(),
            Self::StructField { parent, field } if should_shorten_server_capabilities_field(parent, field) => {
                field.to_upper_camel_case()
            }
            Self::MapValue(parent) if parent.is_related_diagnostic_documents() => {
                "RelatedDocumentDiagnosticReport".into()
            }
            Self::StructField { parent, field } if is_notebook_filter_notebook_field(parent, field) => {
                "NotebookDocumentFilterNotebook".into()
            }
            Self::StructField { parent, field } => shorten_field_union_name(parent, field),
            _ => self.fallback_name(),
        }
    }

    fn fallback_name(&self) -> String {
        match self {
            Self::TypeAlias(name) | Self::RequestResponse(name) => name.to_upper_camel_case(),
            Self::StructField { parent, field } => format!("{}{}", parent, field.to_upper_camel_case()),
            Self::MapKey(parent) => format!("{}Key", parent.union_name()),
            Self::MapValue(parent) => format!("{}Value", parent.union_name()),
            Self::ArrayItem(parent) => format!("{}Item", parent.union_name()),
            Self::TupleItem { parent, index } => format!("{}Item{index}", parent.union_name()),
            Self::Variant { parent, name } => format!("{}{}", parent.union_name(), name),
        }
    }

    fn is_related_diagnostic_documents(&self) -> bool {
        matches!(
            self,
            Self::StructField { parent, field }
                if field == "relatedDocuments"
                    && matches!(
                        parent.as_str(),
                        "DocumentDiagnosticReportPartialResult"
                            | "RelatedFullDocumentDiagnosticReport"
                            | "RelatedUnchangedDocumentDiagnosticReport"
                    )
        )
    }

    fn is_notebook_selector_field(&self) -> bool {
        matches!(self, Self::StructField { field, .. } if field == "notebookSelector")
    }
}

fn should_shorten_server_capabilities_field(parent: &str, field: &str) -> bool {
    parent == "ServerCapabilities" && (field.ends_with("Provider") || field.ends_with("Sync"))
}

fn is_notebook_filter_notebook_field(parent: &str, field: &str) -> bool {
    field == "notebook"
        && matches!(
            parent,
            "NotebookDocumentFilterWithNotebook" | "NotebookDocumentFilterWithCells" | "NotebookCellTextDocumentFilter"
        )
}

fn shorten_field_union_name(parent: &str, field: &str) -> String {
    let field = field.to_upper_camel_case();
    if let Some(stem) = parent
        .strip_suffix("RegistrationOptions")
        .or_else(|| parent.strip_suffix("ClientCapabilities"))
        .or_else(|| parent.strip_suffix("ServerCapabilities"))
        .or_else(|| parent.strip_suffix("Options"))
        && !stem.is_empty()
    {
        format!("{stem}{field}")
    } else {
        format!("{parent}{field}")
    }
}

fn gen_unions(unions: &UnionRegistry) -> String {
    unions
        .entries
        .iter()
        .map(|entry| match entry {
            UnionEntry::Definition(index) => gen_union_def(&unions.definitions[*index]),
            UnionEntry::Alias(index) => gen_union_alias(&unions.aliases[*index]),
        })
        .join("\n\n")
}

fn gen_union_def(union: &UnionDef) -> String {
    let doc = gen_doc(union.documentation.as_deref(), 0);
    let cfg = gen_cfg(union.proposed);
    let variants = union
        .variants
        .iter()
        .map(|variant| {
            let cfg = if variant.proposed {
                "    #[cfg(feature = \"proposed\")]\n"
            } else {
                ""
            };
            let original_name = variant
                .original_name
                .as_ref()
                .map(|name| format!("    /// `{name}`.\n"))
                .unwrap_or_default();
            format!("{cfg}{original_name}    {}({}),", variant.name, variant.ty)
        })
        .join("\n");
    let from_impls = gen_union_from_impls(union);
    format!(
        "{cfg}#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]\n#[serde(untagged)]{}\npub enum {} {{\n{}\n}}{from_impls}",
        doc, union.name, variants,
    )
}

fn gen_union_alias(alias: &UnionAlias) -> String {
    let doc = gen_doc(alias.documentation.as_deref(), 0);
    let cfg = gen_cfg(alias.proposed);
    format!(
        "{cfg}{}{}pub type {} = {};",
        doc,
        if doc.is_empty() { "" } else { "\n" },
        alias.name,
        alias.target
    )
}

fn gen_cfg(proposed: bool) -> &'static str {
    if proposed {
        "#[cfg(feature = \"proposed\")]\n"
    } else {
        ""
    }
}

fn gen_union_from_impls(union: &UnionDef) -> String {
    union
        .variants
        .iter()
        .filter(|variant| variant.ty != union.name && has_unique_variant_type(union, &variant.ty))
        .map(|variant| {
            let cfg = gen_cfg(union.proposed || variant.proposed);
            format!(
                "\n\n{cfg}impl From<{}> for {} {{\n    fn from(value: {}) -> Self {{\n        Self::{}(value)\n    }}\n}}",
                variant.ty, union.name, variant.ty, variant.name
            )
        })
        .join("")
}

fn has_unique_variant_type(union: &UnionDef, ty: &str) -> bool {
    union.variants.iter().filter(|variant| variant.ty == ty).count() == 1
}

fn gen_type_def(
    type_def: &TypeDef,
    unions: &mut UnionRegistry,
    context: &UnionContext,
    proposed: bool,
    documentation: Option<&str>,
) -> String {
    match type_def {
        TypeDef::Base { name } => gen_base_type(name).into(),
        TypeDef::Ref(TypeRef { name }) => match &**name {
            "LSPAny" => "serde_json::Value",
            "LSPObject" => "HashMap<String, serde_json::Value>",
            name => name,
        }
        .into(),
        TypeDef::Map { key, value } => format!(
            "HashMap<{}, {}>",
            gen_type_def(key, unions, &context.map_key(), proposed, None),
            gen_type_def(value, unions, &context.map_value(), proposed, None)
        ),
        TypeDef::Or { items } => {
            let items = items.to_vec();
            if items.len() == 1 {
                gen_type_def(items.first().unwrap(), unions, context, proposed, documentation)
            } else {
                let union_name = context.union_name();
                let original_variant_names = items.iter().map(gen_variant_name).collect::<Vec<_>>();
                let variant_names = shorten_variant_names(&items, original_variant_names.clone(), &union_name);
                let variants = items
                    .iter()
                    .zip(original_variant_names)
                    .zip(variant_names)
                    .map(|((item, original_name), variant_name)| {
                        let ty = gen_type_def(item, unions, &context.variant(&original_name), proposed, None);
                        let original_name = (original_name != variant_name).then_some(original_name);
                        UnionVariant {
                            name: variant_name,
                            original_name,
                            ty,
                            proposed: variant_requires_proposed_feature(item),
                        }
                    })
                    .collect();
                unions.push(union_name, variants, documentation, proposed)
            }
        }
        TypeDef::Array { element } => format!(
            "Vec<{}>",
            gen_type_def(element, unions, &context.array_item(), proposed, None)
        ),
        TypeDef::Tuple { items } => format!(
            "({})",
            items
                .iter()
                .enumerate()
                .map(|(index, item)| gen_type_def(item, unions, &context.tuple_item(index), proposed, None))
                .join(", ")
        ),
        TypeDef::Literal => "serde_json::Value".into(),
        TypeDef::StringLiteral => "String".into(),
        _ => unreachable!(),
    }
}

fn is_multi_union(type_def: &TypeDef) -> bool {
    matches!(type_def, TypeDef::Or { items } if items.len() > 1)
}

fn variant_requires_proposed_feature(type_def: &TypeDef) -> bool {
    is_ref(type_def, "SnippetTextEdit")
}

fn is_ref(type_def: &TypeDef, expected: &str) -> bool {
    matches!(type_def, TypeDef::Ref(TypeRef { name }) if name == expected)
}

fn gen_variant_name(type_def: &TypeDef) -> String {
    match type_def {
        TypeDef::Base { name } => match name {
            BaseType::Null => "Null",
            BaseType::Uinteger => "UInteger",
            BaseType::Integer => "Integer",
            BaseType::String => "String",
            BaseType::Boolean => "Bool",
            BaseType::DocumentUri => "DocumentUri",
            BaseType::Uri => "Uri",
            BaseType::Decimal => "Decimal",
        }
        .into(),
        TypeDef::Ref(TypeRef { name }) => match &**name {
            "LSPAny" => "Value",
            "LSPObject" => "Object",
            name => name,
        }
        .to_upper_camel_case(),
        TypeDef::Map { value, .. } => format!("{}Map", gen_variant_name(value)),
        TypeDef::Or { .. } => "Union".into(),
        TypeDef::Array { element } => format!("{}List", gen_variant_name(element)),
        TypeDef::Tuple { .. } => "Tuple".into(),
        TypeDef::Literal => "Object".into(),
        TypeDef::StringLiteral => "String".into(),
        _ => unreachable!(),
    }
}

fn shorten_variant_names(type_defs: &[TypeDef], names: Vec<String>, enum_name: &str) -> Vec<String> {
    debug_assert_eq!(type_defs.len(), names.len());

    let segments = names
        .iter()
        .map(|name| split_upper_camel_case(name))
        .collect::<Vec<_>>();
    let named_variant_indices = type_defs
        .iter()
        .enumerate()
        .filter_map(|(index, type_def)| (!is_base_variant_type(type_def)).then_some(index))
        .collect::<Vec<_>>();
    let candidate_segments = shortened_variant_segments(&segments, &named_variant_indices);
    let enum_segments = split_upper_camel_case(enum_name);

    names
        .into_iter()
        .enumerate()
        .fold(IndexSet::new(), |mut shortened_names, (index, name)| {
            let is_base_variant = is_base_variant_type(&type_defs[index]);
            let candidates = [
                (!is_base_variant)
                    .then(|| {
                        shortened_variant_against_enum(&segments[index], &enum_segments, named_variant_indices.len())
                    })
                    .flatten(),
                candidate_segments
                    .as_ref()
                    .and_then(|segments| segments.get(&index))
                    .map(|segments| segments.concat()),
            ];
            let name = choose_variant_name(&shortened_names, &name, candidates.into_iter().flatten(), enum_name);
            shortened_names.insert(name);
            shortened_names
        })
        .into_iter()
        .collect()
}

fn shortened_variant_segments(
    segments: &[Vec<String>],
    indices: &[usize],
) -> Option<std::collections::HashMap<usize, Vec<String>>> {
    if indices.len() < 2 {
        return None;
    }

    let selected = indices
        .iter()
        .map(|index| segments[*index].as_slice())
        .collect::<Vec<_>>();
    let prefix_len = common_prefix_len_all(&selected);
    let mut shortened = indices
        .iter()
        .map(|index| {
            let segments = &segments[*index];
            let start = if prefix_len > 0 && prefix_len < segments.len() {
                prefix_len
            } else {
                0
            };
            (*index, segments[start..].to_vec())
        })
        .collect::<std::collections::HashMap<_, _>>();

    let selected = shortened.values().map(Vec::as_slice).collect::<Vec<_>>();
    let suffix_len = common_suffix_len_all(&selected);
    if suffix_len >= 2 && selected.iter().all(|segments| suffix_len < segments.len()) {
        for segments in shortened.values_mut() {
            segments.truncate(segments.len() - suffix_len);
        }
    }

    shortened
        .iter()
        .any(|(index, shortened_segments)| shortened_segments.as_slice() != segments[*index].as_slice())
        .then_some(shortened)
}

fn shortened_variant_against_enum(
    segments: &[String],
    enum_segments: &[String],
    named_variant_count: usize,
) -> Option<String> {
    if named_variant_count != 1 {
        return None;
    }

    let prefix_len = common_prefix_len(segments, enum_segments);
    if prefix_len == 0 || prefix_len >= segments.len() {
        return None;
    }

    let candidate = segments[prefix_len..].concat();
    (!is_generic_variant_name(&candidate)).then_some(candidate)
}

fn split_upper_camel_case(name: &str) -> Vec<String> {
    let chars = name.char_indices().collect::<Vec<_>>();
    let mut segments = Vec::new();
    let mut segment_start = 0;

    for (index, &(byte_index, ch)) in chars.iter().enumerate().skip(1) {
        let previous = chars[index - 1].1;
        let next = chars.get(index + 1).map(|(_, ch)| *ch);
        if ch.is_uppercase() && (!previous.is_uppercase() || next.is_some_and(char::is_lowercase)) {
            segments.push(name[segment_start..byte_index].to_string());
            segment_start = byte_index;
        }
    }

    segments.push(name[segment_start..].to_string());
    segments
}

fn common_prefix_len_all(items: &[&[String]]) -> usize {
    let Some(first) = items.first() else {
        return 0;
    };

    first
        .iter()
        .enumerate()
        .take_while(|(index, segment)| items.iter().all(|item| item.get(*index) == Some(segment)))
        .count()
}

fn common_prefix_len(left: &[String], right: &[String]) -> usize {
    left.iter().zip(right).take_while(|(left, right)| left == right).count()
}

fn common_suffix_len_all(items: &[&[String]]) -> usize {
    let Some(first) = items.first() else {
        return 0;
    };

    first
        .iter()
        .rev()
        .enumerate()
        .take_while(|(index, segment)| items.iter().all(|item| item.iter().rev().nth(*index) == Some(segment)))
        .count()
}

fn is_base_variant_type(type_def: &TypeDef) -> bool {
    matches!(type_def, TypeDef::Base { .. })
}

fn is_generic_variant_name(name: &str) -> bool {
    matches!(
        name,
        "Full"
            | "Id"
            | "Item"
            | "Items"
            | "Kind"
            | "Label"
            | "Object"
            | "Params"
            | "Range"
            | "Result"
            | "Tooltip"
            | "Value"
    )
}

fn choose_variant_name(
    names: &IndexSet<String>,
    original: &str,
    candidates: impl IntoIterator<Item = String>,
    enum_name: &str,
) -> String {
    for candidate in candidates.into_iter().chain([original.to_string()]) {
        if !candidate.is_empty() && !names.contains(&candidate) {
            return candidate;
        }
    }

    panic!("duplicate generated variant `{original}` in union `{enum_name}`")
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
    result: Option<TypeDef>,
    partial_result: Option<TypeDef>,
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

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
struct TypeRef {
    name: String,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
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
