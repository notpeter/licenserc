# licenserc: License Rules Checker

Tool for enforcing license rules for a given repo via `.licenserc.yaml` files.

Note: `.licenserc.yaml` ([Apache SkyWalking Eyes](https://github.com/apache/skywalking-eyes/))
is not compatible with `licenserc.toml` ([HawkEye](github.com/korandoru/hawkeye)).

## Schema

JSON Schema for `.licenserc.yaml` is defined in: [licenserc.schema.json](./licenserc.schema.json)

If you are using yaml-language-server (Zed, VScode with [redhat.vscode-yaml](https://marketplace.visualstudio.com/items?itemName=redhat.vscode-yaml), etc) you can link to a schema and get autocomplete/validation in your editor.

Just add this as the first line of your `.licenserc.yaml`:
```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/notpeter/licenserc/main/licenserc.schema.json
```

TODO: Submit to [JSON Schema Store](https://www.schemastore.org/)

## License

[Apache 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.
