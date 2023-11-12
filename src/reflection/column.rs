use crate::metadata::WithMetadata;
use crate::reflection::datatypes::{Datatype, DefaultValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub(super) table: Rc<String>,
    pub(super) name: Rc<String>,
    datatype: Datatype,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<DefaultValue>,
    metadata: HashMap<String, String>,
}

impl WithMetadata for Column {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl Column {
    pub fn new(table: impl ToString, name: impl ToString, datatype: Datatype) -> Column {
        Column {
            table: Rc::new(table.to_string()),
            name: Rc::new(name.to_string()),
            datatype,
            ..Default::default()
        }
    }

    pub fn set_default(&mut self, value: Option<DefaultValue>) -> &mut Column {
        self.default = value;
        self
    }

    pub fn table(&self) -> Rc<String> {
        self.table.clone()
    }

    pub fn name(&self) -> Rc<String> {
        self.name.clone()
    }

    pub fn datatype(&self) -> &Datatype {
        &self.datatype
    }

    pub fn default(&self) -> Option<DefaultValue> {
        self.default.clone()
    }
}
