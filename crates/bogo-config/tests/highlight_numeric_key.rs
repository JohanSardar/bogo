use bogo_config::{Config, Format, PathElem, Secrets};

#[test]
fn old_text_highlight_keeps_numeric_object_key() {
    // Existing clients send e.g. {"totals":{"2025":"Annual total"}}.
    // With only old formats, the signed hint path must remain unchanged.
    let config = Config::parse(
        r#"
version = 1
[server]
port = 8790
[defaults]
policy = "approve"
max_pending = 8
[upstreams.reports]
command = "reports-mcp"
[tools.reports__publish]
highlight = { "totals.2025" = "text" }
"#,
        &Secrets::default(),
    )
    .unwrap();

    assert_eq!(
        config.tools["reports__publish"].hints,
        vec![(
            vec![PathElem::Key("totals".into()), PathElem::Key("2025".into())],
            Format::Text,
        )],
        "an existing text hint must still address the object member and keep its wire bytes"
    );
}
