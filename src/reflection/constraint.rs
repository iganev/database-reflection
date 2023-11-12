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
pub struct Constraint {
    pub(super) name: Rc<String>,
    local: Rc<Column>,
    foreign: Rc<Column>,
    metadata: HashMap<String, String>,
}

impl WithMetadata for Constraint {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl Constraint {
    pub fn new(name: impl ToString, local: Rc<Column>, foreign: Rc<Column>) -> Self {
        Constraint {
            name: Rc::new(name.to_string()),
            local,
            foreign,
            ..Default::default()
        }
    }

    pub fn name(&self) -> Rc<String> {
        self.name.clone()
    }

    pub fn local(&self) -> &Column {
        &self.local
    }

    pub fn foreign(&self) -> &Column {
        &self.foreign
    }
}
