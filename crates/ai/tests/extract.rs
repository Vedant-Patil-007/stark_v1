use stark_ai::provider::extract_json;

#[test]
fn extracts_bare_json() {
    let raw = r#"{"action":"CREATE_TASK","title":"X"}"#;
    assert_eq!(extract_json(raw).unwrap(), raw);
}

#[test]
fn extracts_from_markdown_fence() {
    let raw = "```json\n{\"action\":\"CREATE_TASK\",\"title\":\"X\"}\n```";
    assert_eq!(
        extract_json(raw).unwrap(),
        r#"{"action":"CREATE_TASK","title":"X"}"#
    );
}

#[test]
fn extracts_when_model_adds_prose() {
    let raw = "Sure! Here you go:\n{\"action\":\"COMPLETE_TASK\",\"task_ref\":\"DSA\"}\nHope that helps.";
    assert_eq!(
        extract_json(raw).unwrap(),
        r#"{"action":"COMPLETE_TASK","task_ref":"DSA"}"#
    );
}

#[test]
fn handles_nested_objects() {
    let raw = r#"{"action":"X","meta":{"a":{"b":1}}}"#;
    assert_eq!(extract_json(raw).unwrap(), raw);
}

#[test]
fn handles_braces_inside_strings() {
    let raw = r#"{"action":"CREATE_TASK","title":"fix the {bug}"}"#;
    assert_eq!(extract_json(raw).unwrap(), raw);
}

#[test]
fn rejects_no_json() {
    assert!(extract_json("I'm sorry, I can't do that.").is_err());
}

#[test]
fn rejects_unbalanced() {
    assert!(extract_json(r#"{"action":"X""#).is_err());
}