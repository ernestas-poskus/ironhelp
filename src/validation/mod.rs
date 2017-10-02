use std::collections::BTreeMap;
use iron::typemap;
use std::fmt;
use serde_json;
use std::error::Error as StdError;

/// Convenience type alias for errors map
pub type ValidationMap = BTreeMap<&'static str, Vec<&'static str>>;

/// Struct for keeping validation errors
/// Generated JSON begins with "errors": { "x": "v" }
#[derive(Serialize, Debug)]
pub struct ValidationError {
    /// Collected errors
    pub errors: ValidationMap,
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

impl typemap::Key for ValidationError {
    type Value = ValidationError;
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&serde_json::to_string(&self).expect(
            "fmt::Display ValidationMap serde_json serialization failed",
        ))
    }
}

impl StdError for ValidationError {
    fn description(&self) -> &str {
        "Validation errors"
    }
}

#[test]
fn test_serde() {
    assert_eq!(
        ValidationError::new_map("hi", vec!["e1", "e2"]).to_string(),
        r#"{"errors":{"hi":["e1","e2"]}}"#
    );
}
