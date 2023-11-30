use database_reflection::metadata::WithMetadata;
use std::collections::HashMap;

struct TestMetadata {
    metadata: HashMap<String, String>,
}

impl TestMetadata {
    pub fn new() -> TestMetadata {
        TestMetadata {
            metadata: HashMap::new(),
        }
    }
}

impl WithMetadata for TestMetadata {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

#[test]
fn test_consts_are_public() {
    let mut test_metadata = TestMetadata::new();

    test_metadata.set_meta("test_key", "test_value");
    test_metadata.set_meta_flag("test_flag");

    assert_eq!(
        test_metadata.meta("test_key"),
        Some(String::from("test_value"))
    );
    assert!(test_metadata.meta_flag("test_flag"));
}
