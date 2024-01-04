use crate::metadata::consts::METADATA_FLAG_UNSIGNED;
use crate::metadata::WithMetadata;
use crate::reflection::datatypes::{DefaultValue, JsonDatatype, RustDatatype, SqlDatatype};
use crate::reflection::SqlSigned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    table: Arc<String>,
    name: Arc<String>,
    datatype: SqlDatatype,
    datatype_json: JsonDatatype,
    datatype_rust: RustDatatype,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<DefaultValue>,
    metadata: HashMap<String, String>,
}

impl WithMetadata for Column {
    /// Borrow metadata container for reading
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Borrow metadata container for writing
    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl Column {
    /// Create a new column by supplying at minimum its name, type and table
    pub fn new(table: impl ToString, name: impl ToString, datatype: SqlDatatype) -> Column {
        let mut c = Column {
            table: Arc::new(table.to_string()),
            name: Arc::new(name.to_string()),
            datatype: datatype.clone(),
            datatype_json: (&datatype).into(),
            datatype_rust: (&datatype).into(),
            ..Default::default()
        };

        if datatype.sign() == Some(SqlSigned::Unsigned) {
            c.set_meta_flag(METADATA_FLAG_UNSIGNED);
        }

        c
    }

    /// Set an optional default value
    pub fn set_default(&mut self, value: Option<DefaultValue>) -> &mut Column {
        self.default = value;
        self
    }

    /// Get table name
    pub fn table(&self) -> Arc<String> {
        self.table.clone()
    }

    /// Get column name
    pub fn name(&self) -> Arc<String> {
        self.name.clone()
    }

    /// Get datatype
    pub fn datatype(&self) -> &SqlDatatype {
        &self.datatype
    }

    /// Get JS/JSON datatype
    pub fn datatype_json(&self) -> &JsonDatatype {
        &self.datatype_json
    }

    /// Get rust datatype
    pub fn datatype_rust(&self) -> &RustDatatype {
        &self.datatype_rust
    }

    /// Get default value if available
    pub fn default(&self) -> Option<DefaultValue> {
        self.default.clone()
    }
}
