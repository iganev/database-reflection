use crate::metadata::with_metadata::WithMetadata;
use crate::reflection::datatypes::{Datatype, DefaultValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Column<'n> {
    pub(super) table: &'n str,
    pub(super) name: &'n str,
    datatype: Datatype,
    #[serde(skip_serializing_if = "Option::is_none")]
    default: Option<DefaultValue>,
    metadata: HashMap<String, String>,
}

impl<'n> WithMetadata for Column<'n> {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl<'n> Column<'n> {
    pub fn new(table: &'n str, name: &'n str, datatype: Datatype) -> Column<'n> {
        Column {
            table,
            name,
            datatype,
            ..Default::default()
        }
    }

    pub fn set_default(&mut self, value: Option<DefaultValue>) -> &mut Column<'n> {
        self.default = value;
        self
    }

    pub fn table(&self) -> &str {
        self.table
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn datatype(&self) -> &Datatype {
        &self.datatype
    }

    pub fn default(&self) -> Option<DefaultValue> {
        self.default.clone()
    }
}
