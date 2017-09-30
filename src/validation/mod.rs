use std::collections::BTreeMap;

/// Convenience type alias for errors map
pub type ValidationMap = BTreeMap<&'static str, Vec<&'static str>>;

/// Struct for keeping validation errors
/// Generated JSON begins with "errors": { "x": "v" }
#[derive(Serialize)]
pub struct ValidationError {
    errors: ValidationMap,
}

impl ValidationError {
    /// Initialize errors with map
    pub fn new(errors: ValidationMap) -> ValidationError {
        ValidationError { errors: errors }
    }

    /// Initialize errors map from key and errors
    pub fn new_map(key: &'static str, errors: Vec<&'static str>) -> ValidationError {
        let mut errors_map: ValidationMap = BTreeMap::new();
        errors_map.insert(key, errors);
        ValidationError { errors: errors_map }
    }
}

#[test]
fn test_serde() {
    use serde_json;

    assert_eq!(
        serde_json::to_string(&ValidationError::new_map("hi", vec!["e1", "e2"])).unwrap(),
        r#"{"errors":{"hi":["e1","e2"]}}"#
    );
}
