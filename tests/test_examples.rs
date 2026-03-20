use licenserc::Config;

fn yaml_examples() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "daytonaio_daytona",
            include_str!("examples/daytonaio_daytona.licenserc.yaml"),
        ),
        (
            "kubearmor_KubeArmor",
            include_str!("examples/kubearmor_KubeArmor.licenserc.yaml"),
        ),
        (
            "milvus-io_milvus",
            include_str!("examples/milvus-io_milvus.licenserc.yaml"),
        ),
        (
            "ruby-oauth_oauth",
            include_str!("examples/ruby-oauth_oauth.licenserc.yaml"),
        ),
    ]
}

#[test]
fn parse_yaml_examples() {
    for (name, content) in yaml_examples() {
        let config: Config = serde_saphyr::from_str(content)
            .unwrap_or_else(|e| panic!("failed to parse {name}: {e}"));
        assert!(config.header.is_some(), "{name} should have a header");
    }
}

#[test]
fn yaml_roundtrip() {
    for (name, content) in yaml_examples() {
        let config: Config = serde_saphyr::from_str(content)
            .unwrap_or_else(|e| panic!("failed to parse {name}: {e}"));
        let json = serde_json::to_string(&config).expect("serialize to json");
        let _: Config =
            serde_json::from_str(&json).unwrap_or_else(|e| panic!("roundtrip failed {name}: {e}"));
    }
}
