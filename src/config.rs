use schemars::JsonSchema;
use schemars::schema::{
    InstanceType, ObjectValidation, RootSchema, Schema, SchemaObject, SingleOrVec,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Top-level licenserc configuration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header: Option<Header>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dependency: Option<Dependency>,
}

/// License header configuration. Accepts a single header config (V1) or an
/// array of header configs (V2) for multi-module projects.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Header {
    V1(HeaderConfig),
    V2(Vec<HeaderConfig>),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct HeaderConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    /// Glob patterns specifying which files to check. Supports doublestar (**) syntax.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,

    /// Glob patterns specifying which files to exclude. .git and .gitignore entries are automatically ignored.
    #[serde(
        default,
        rename = "paths-ignore",
        skip_serializing_if = "Option::is_none"
    )]
    pub paths_ignore: Option<Vec<String>>,

    /// Controls PR commenting behavior for CI/GitHub Actions integration.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment: Option<CommentOption>,

    /// Maximum number of characters from the start of a file where the license header can be located. A header beyond this threshold is treated as missing.
    #[serde(
        default,
        rename = "license-location-threshold",
        skip_serializing_if = "Option::is_none"
    )]
    pub license_location_threshold: Option<u32>,

    /// Override or customize comment styles for specific languages.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<HashMap<String, Language>>,
}

/// License header definition.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct License {
    /// SPDX license identifier. Used to select a built-in header template.
    #[serde(default, rename = "spdx-id", skip_serializing_if = "Option::is_none")]
    pub spdx_id: Option<String>,

    /// Copyright holder name. Replaces [owner] in SPDX templates.
    #[serde(
        default,
        rename = "copyright-owner",
        skip_serializing_if = "Option::is_none"
    )]
    pub copyright_owner: Option<String>,

    /// Copyright year or range (e.g. "2024", "2020-2024"). Replaces [year] in templates. Defaults to current year.
    #[serde(
        default,
        rename = "copyright-year",
        skip_serializing_if = "Option::is_none"
    )]
    pub copyright_year: Option<String>,

    /// Project name. Replaces [software-name] in templates.
    #[serde(
        default,
        rename = "software-name",
        skip_serializing_if = "Option::is_none"
    )]
    pub software_name: Option<String>,

    /// Full license header text. Takes precedence over spdx-id when both are specified. Supports [year], [owner], and [software-name] placeholders.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// Regex pattern to match existing license headers. Enables flexible validation of varying header formats.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

/// PR commenting behavior.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum CommentOption {
    #[serde(rename = "on-failure")]
    OnFailure,
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "never")]
    Never,
}

/// Language-specific comment style configuration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Language {
    /// File extensions that trigger this language config (e.g. [".go"]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,

    /// Specific filenames to match (e.g. ["Makefile"]).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filenames: Option<Vec<String>>,

    /// Reference to a built-in comment style.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_style_id: Option<CommentStyleId>,
}

/// Built-in comment style identifiers.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub enum CommentStyleId {
    DoubleSlash,
    SlashAsterisk,
    Hashtag,
    AngleBracket,
    PhpTag,
    Remark,
    CurlyBracketDash,
    CurlyBracketHashtag,
    CurlyBracketAsterisk,
    DoubleDash,
    RoundBracketAsterisk,
    Semicolon,
    Percent,
    Apostrophe,
    DoubleDot,
    Quotes,
    PythonStyle,
    PythonDocStringStyle,
}

/// Dependency license compatibility checking configuration.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    /// Dependency manifest files to scan (e.g. go.mod, Cargo.toml, package.json). Paths are relative to the config file location.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,

    /// Manually declare licenses for dependencies that cannot be automatically identified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<DependencyLicense>>,

    /// Dependencies to exclude from license analysis.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<DependencyExclude>>,

    /// Minimum percentage of a file's content that must contain license text for the license to be identified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<u32>,

    /// When true, only FSF Free/Libre-marked licenses are considered compatible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_fsf_free: Option<bool>,

    /// When true, only OSI-approved licenses are considered compatible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_osi_approved: Option<bool>,
}

/// Manual license declaration for a dependency.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DependencyLicense {
    /// Dependency identifier. Format varies by ecosystem: package path (Go), GroupID:ArtifactID (Maven), package name (NPM).
    pub name: String,

    /// Comma-separated versions (e.g. "1.0,2.0,3.0"). Empty matches all versions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// SPDX license identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

/// Dependency exclusion rule.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DependencyExclude {
    /// Dependency identifier.
    pub name: String,

    /// Specific version(s) to exclude. Empty excludes all versions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Exclude transitive dependencies too (Maven only).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recursive: Option<bool>,
}

const SCHEMA_ID: &str =
    "https://raw.githubusercontent.com/notpeter/licenserc/main/licenserc.schema.json";

/// Generate the JSON schema for the licenserc config, with post-processing
/// to match the hand-crafted schema style.
pub fn generate_schema() -> RootSchema {
    let mut root = schemars::schema_for!(Config);

    // Set $id and title on the root schema.
    let meta = root.schema.metadata.get_or_insert_with(Default::default);
    meta.id = Some(SCHEMA_ID.to_string());
    meta.title = Some("licenserc configuration".to_string());
    meta.description = Some(
        "Configuration for licenserc license header and dependency checking tool.".to_string(),
    );

    // Strip null from all Option<T> fields throughout the schema, add
    // defaults and constraints, then lowercase definition names.
    strip_nulls_from_schema(&mut root.schema);
    for (_name, def) in &mut root.definitions {
        if let Schema::Object(obj) = def {
            strip_nulls_from_schema(obj);
        }
    }

    add_defaults_and_constraints(&mut root);
    lowercase_definition_keys(&mut root);

    root
}

/// Convert schemars' `Option<T>` representations back to plain types:
///   - `anyOf: [{$ref: X}, {type: null}]` → `{$ref: X}`
///   - `type: ["string", "null"]` → `type: "string"`
fn strip_nulls_from_schema(schema: &mut SchemaObject) {
    // Handle properties recursively.
    if let Some(obj) = &mut schema.object {
        for (_key, prop) in &mut obj.properties {
            if let Schema::Object(prop_obj) = prop {
                strip_nulls_from_schema(prop_obj);
            }
        }
        if let Some(Schema::Object(additional)) = obj.additional_properties.as_deref_mut() {
            strip_nulls_from_schema(additional);
        }
    }

    // Handle array items.
    if let Some(arr) = &mut schema.array {
        if let Some(Schema::Object(items)) = arr.items.as_mut().and_then(|i| match i {
            SingleOrVec::Single(s) => Some(s.as_mut()),
            _ => None,
        }) {
            strip_nulls_from_schema(items);
        }
    }

    // Handle `anyOf: [{$ref: X}, {type: null}]` → inline the $ref.
    if let Some(subschemas) = &schema.subschemas {
        if let Some(any_of) = &subschemas.any_of {
            if any_of.len() == 2 {
                let (ref_schema, null_schema) = (&any_of[0], &any_of[1]);
                let is_null = matches!(null_schema, Schema::Object(s) if
                    s.instance_type == Some(SingleOrVec::Single(Box::new(InstanceType::Null))));
                if is_null {
                    if let Schema::Object(ref_obj) = ref_schema {
                        // Preserve any description from the parent before replacing.
                        let desc = schema.metadata.as_ref().and_then(|m| m.description.clone());
                        *schema = ref_obj.clone();
                        if let Some(d) = desc {
                            schema
                                .metadata
                                .get_or_insert_with(Default::default)
                                .description = Some(d);
                        }
                        // Recurse into the replaced schema.
                        strip_nulls_from_schema(schema);
                        return;
                    }
                }
            }
        }
    }

    // Handle `anyOf` used for untagged enums (Header): rename to `oneOf`.
    if let Some(subschemas) = &mut schema.subschemas {
        if let Some(any_of) = subschemas.any_of.take() {
            let has_null = any_of.iter().any(|s| {
                matches!(s, Schema::Object(o) if
                    o.instance_type == Some(SingleOrVec::Single(Box::new(InstanceType::Null))))
            });
            if !has_null {
                subschemas.one_of = Some(any_of);
            } else {
                subschemas.any_of = Some(any_of);
            }
        }
    }

    // Handle `type: ["string", "null"]` → `type: "string"`.
    if let Some(SingleOrVec::Vec(types)) = &schema.instance_type {
        let non_null: Vec<_> = types
            .iter()
            .filter(|t| **t != InstanceType::Null)
            .cloned()
            .collect();
        if non_null.len() == 1 {
            schema.instance_type = Some(SingleOrVec::Single(Box::new(non_null[0])));
        }
    }
}

/// Add default values and numeric constraints that schemars doesn't infer.
fn add_defaults_and_constraints(root: &mut RootSchema) {
    // Helper to get a mutable definition by name.
    fn get_def<'a>(root: &'a mut RootSchema, name: &str) -> Option<&'a mut SchemaObject> {
        root.definitions.get_mut(name).and_then(|s| match s {
            Schema::Object(o) => Some(o),
            _ => None,
        })
    }

    fn set_prop_default(obj: &mut ObjectValidation, key: &str, default: serde_json::Value) {
        if let Some(Schema::Object(prop)) = obj.properties.get_mut(key) {
            prop.metadata.get_or_insert_with(Default::default).default = Some(default);
        }
    }

    // HeaderConfig defaults.
    if let Some(header) = get_def(root, "HeaderConfig") {
        if let Some(obj) = &mut header.object {
            set_prop_default(obj, "paths", serde_json::json!(["**"]));
            set_prop_default(obj, "license-location-threshold", serde_json::json!(80));
        }
    }

    // CommentOption default.
    if let Some(comment) = get_def(root, "CommentOption") {
        comment
            .metadata
            .get_or_insert_with(Default::default)
            .default = Some(serde_json::json!("on-failure"));
    }

    // Dependency defaults and constraints.
    if let Some(dep) = get_def(root, "Dependency") {
        if let Some(obj) = &mut dep.object {
            set_prop_default(obj, "threshold", serde_json::json!(75));
            set_prop_default(obj, "require_fsf_free", serde_json::json!(false));
            set_prop_default(obj, "require_osi_approved", serde_json::json!(false));

            // Add maximum: 100 to threshold.
            if let Some(Schema::Object(threshold)) = obj.properties.get_mut("threshold") {
                let num = threshold.number.get_or_insert_with(Default::default);
                num.maximum = Some(100.0);
            }
        }
    }

    // Header V2 array: add minItems: 1.
    if let Some(hf) = get_def(root, "Header") {
        if let Some(sub) = &mut hf.subschemas {
            if let Some(one_of) = &mut sub.one_of {
                for variant in one_of.iter_mut() {
                    if let Schema::Object(obj) = variant {
                        if obj.instance_type
                            == Some(SingleOrVec::Single(Box::new(InstanceType::Array)))
                        {
                            let arr = obj.array.get_or_insert_with(Default::default);
                            arr.min_items = Some(1);
                        }
                    }
                }
            }
        }
    }
}

/// Rename definition keys from PascalCase to camelCase to match the
/// hand-crafted schema convention.
fn lowercase_definition_keys(root: &mut RootSchema) {
    fn to_camel(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            Some(c) => c.to_lowercase().to_string() + chars.as_str(),
            None => String::new(),
        }
    }

    // Build old→new name mapping.
    let renames: HashMap<String, String> = root
        .definitions
        .keys()
        .map(|k| (k.clone(), to_camel(k)))
        .filter(|(old, new)| old != new)
        .collect();

    // Rename keys in the definitions map.
    for (old, new) in &renames {
        if let Some(schema) = root.definitions.remove(old) {
            root.definitions.insert(new.clone(), schema);
        }
    }

    // Update all $ref pointers throughout.
    fn update_refs(schema: &mut Schema, renames: &HashMap<String, String>) {
        match schema {
            Schema::Object(obj) => update_refs_obj(obj, renames),
            Schema::Bool(_) => {}
        }
    }

    fn update_refs_obj(obj: &mut SchemaObject, renames: &HashMap<String, String>) {
        if let Some(ref mut r) = obj.reference {
            for (old, new) in renames {
                let old_ref = format!("#/definitions/{old}");
                let new_ref = format!("#/definitions/{new}");
                if *r == old_ref {
                    *r = new_ref;
                    break;
                }
            }
        }
        if let Some(sub) = &mut obj.subschemas {
            for list in [&mut sub.any_of, &mut sub.one_of, &mut sub.all_of] {
                if let Some(schemas) = list {
                    for s in schemas {
                        update_refs(s, renames);
                    }
                }
            }
            if let Some(s) = &mut sub.not {
                update_refs(s, renames);
            }
            if let Some(s) = &mut sub.if_schema {
                update_refs(s, renames);
            }
            if let Some(s) = &mut sub.then_schema {
                update_refs(s, renames);
            }
            if let Some(s) = &mut sub.else_schema {
                update_refs(s, renames);
            }
        }
        if let Some(o) = &mut obj.object {
            for prop in o.properties.values_mut() {
                update_refs(prop, renames);
            }
            if let Some(additional) = &mut o.additional_properties {
                update_refs(additional, renames);
            }
        }
        if let Some(arr) = &mut obj.array {
            if let Some(items) = &mut arr.items {
                match items {
                    SingleOrVec::Single(s) => update_refs(s, renames),
                    SingleOrVec::Vec(v) => {
                        for s in v {
                            update_refs(s, renames);
                        }
                    }
                }
            }
        }
    }

    update_refs_obj(&mut root.schema, &renames);
    // We need to iterate over definitions values. Collect keys first.
    let keys: Vec<String> = root.definitions.keys().cloned().collect();
    for key in keys {
        if let Some(schema) = root.definitions.get_mut(&key) {
            update_refs(schema, &renames);
        }
    }
}
