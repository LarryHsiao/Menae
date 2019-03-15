use serde_json::{Result, Value};

#[test]
fn parse_exist() {
    let input =
        r#"
        {
            "key1": "value1",
            "key2": 1
        }
        "#;

    let output: Value = serde_json::from_str(input).unwrap();

    assert_eq!(
        "value1",
        output["key1"].as_str().unwrap()
    )
}

#[test]
fn parse_not_exist() {
    let input =
        r#"
        {
            "key1": "value1",
            "key2": 1
        }
        "#;

    let output: Value = serde_json::from_str(input).unwrap();

    assert_eq!(
        true,
        output["key12"].is_null()
    )
}

