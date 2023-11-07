use crate::metadata::WithMetadata;
use crate::reflection::column::Column;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintSide {
    Local,
    Foreign,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Constraint<'n> {
    pub(super) name: &'n str,
    local: Rc<Column<'n>>,
    foreign: Rc<Column<'n>>,
    metadata: HashMap<String, String>,
}

impl<'n> WithMetadata for Constraint<'n> {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl<'n> Constraint<'n> {
    pub fn new(name: &'n str, local: Rc<Column<'n>>, foreign: Rc<Column<'n>>) -> Self {
        Constraint {
            name,
            local,
            foreign,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn local(&self) -> &Column {
        &self.local
    }

    pub fn foreign(&self) -> &Column {
        &self.foreign
    }
}
