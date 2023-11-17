use crate::metadata::WithMetadata;
use crate::reflection::datatypes::{Datatype, DefaultValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    table: Rc<String>,
    name: Rc<String>,
    datatype: Datatype,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<DefaultValue>,
    metadata: HashMap<String, String>,
}

impl WithMetadata for Column {
    /// Borrow meadata container for reading
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
    pub fn new(table: impl ToString, name: impl ToString, datatype: Datatype) -> Column {
        Column {
            table: Rc::new(table.to_string()),
            name: Rc::new(name.to_string()),
            datatype,
            ..Default::default()
        }
    }

    /// Set an optional default value
    pub fn set_default(&mut self, value: Option<DefaultValue>) -> &mut Column {
        self.default = value;
        self
    }

    /// Get table name
    pub fn table(&self) -> Rc<String> {
        self.table.clone()
    }

    /// Get column name
    pub fn name(&self) -> Rc<String> {
        self.name.clone()
    }

    /// Get datatype
    pub fn datatype(&self) -> &Datatype {
        &self.datatype
    }

    /// Get default value if available
    pub fn default(&self) -> Option<DefaultValue> {
        self.default.clone()
    }
}
