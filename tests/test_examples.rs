use licenserc::Config;

fn yaml_examples() -> Vec<(String, String)> {
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let pattern = manifest_dir.join("tests/examples/*.licenserc.yaml");

    glob::glob(pattern.to_str().unwrap())
        .expect("failed to read glob pattern")
        .map(|entry| {
            let path = entry.expect("failed to read glob entry");
            let name = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_end_matches(".licenserc.yaml")
                .to_string();
            let content = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
            (name, content)
        })
        .collect()
}

#[test]
fn parse_yaml_examples() {
    for (name, content) in yaml_examples() {
        let config: Config = serde_saphyr::from_str(&content)
            .unwrap_or_else(|e| panic!("failed to parse {name}: {e}"));
        assert!(config.header.is_some(), "{name} should have a header");
    }
}

#[test]
fn yaml_roundtrip() {
    for (name, content) in yaml_examples() {
        let config: Config = serde_saphyr::from_str(&content)
            .unwrap_or_else(|e| panic!("failed to parse {name}: {e}"));
        let json = serde_json::to_string(&config).expect("serialize to json");
        let _: Config =
            serde_json::from_str(&json).unwrap_or_else(|e| panic!("roundtrip failed {name}: {e}"));
    }
}
