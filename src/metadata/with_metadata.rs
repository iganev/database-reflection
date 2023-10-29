use std::collections::HashMap;

pub trait WithMetadata {
    fn get_metadata(&self) -> &HashMap<String, String>;

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String>;

    fn set_meta(&mut self, meta_key: impl ToString, meta_value: impl ToString) -> &mut Self {
        self.get_metadata_mut()
            .insert(meta_key.to_string(), meta_value.to_string());

        self
    }

    fn set_meta_flag(&mut self, meta_flag: impl ToString) -> &mut Self {
        self.get_metadata_mut()
            .insert(meta_flag.to_string(), "1".to_string());

        self
    }

    fn meta_flag(&self, flag: &str) -> bool {
        self.get_metadata().contains_key(flag)
    }

    fn meta(&self, key: &str) -> Option<String> {
        self.get_metadata().get(key).cloned()
    }
}
