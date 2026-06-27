use std::{collections::BTreeMap, env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=builders/");

    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");

    let builders_dir = Path::new(&manifest_dir).join("builders");

    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();

    if builders_dir.exists() {
        for entry in fs::read_dir(&builders_dir).expect("failed to read builders directory") {
            let entry = entry.expect("failed to read builder entry");
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let Some(name) = path.file_name().and_then(|v| v.to_str()) else {
                continue;
            };

            let Some((language, version)) = name.split_once('-') else {
                continue;
            };

            groups
                .entry(language.to_string())
                .or_default()
                .push(version.to_string());
        }
    }

    for versions in groups.values_mut() {
        versions.sort();
        versions.dedup();
    }

    let mut output = String::new();

    output.push_str("use serde::Serialize;\n");
    output.push_str("#[derive(Debug, Clone, Serialize)]\n");
    output.push_str("pub struct BuilderLanguage {\n");
    output.push_str("    pub language: &'static str,\n");
    output.push_str("    pub versions: &'static [&'static str],\n");
    output.push_str("}\n\n");

    output.push_str("pub const BUILDER_LANGUAGES: &[BuilderLanguage] = &[\n");

    for (language, versions) in groups {
        output.push_str("    BuilderLanguage {\n");
        output.push_str(&format!("        language: {:?},\n", language));
        output.push_str("        versions: &[\n");

        for version in versions {
            output.push_str(&format!("            {:?},\n", version));
        }

        output.push_str("        ],\n");
        output.push_str("    },\n");
    }

    output.push_str("];\n");

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");

    fs::write(Path::new(&out_dir).join("builders.rs"), output)
        .expect("failed to write builders.rs");
}
