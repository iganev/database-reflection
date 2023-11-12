use crate::metadata::WithMetadata;
use crate::reflection::column::Column;
use crate::reflection::constraint::Constraint;
use crate::reflection::index::Index;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Table {
    pub(super) name: Rc<String>,
    columns: IndexMap<Rc<String>, Rc<Column>>,
    constraints: HashMap<Rc<String>, Rc<Constraint>>,
    indexes: IndexMap<Rc<String>, Index>,
    metadata: HashMap<String, String>,
}

impl<'n> WithMetadata for Table {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl Table {
    pub fn new(name: impl ToString) -> Table {
        Table {
            name: Rc::new(name.to_string()),
            ..Default::default()
        }
    }

    pub fn name(&self) -> Rc<String> {
        self.name.clone()
    }

    pub fn set_column(&mut self, column: Column) -> &mut Table {
        self.columns.insert(column.name(), Rc::new(column));

        self
    }

    pub fn column(&self, key: &str) -> Option<Rc<Column>> {
        self.columns.get(&key.to_string()).cloned()
    }

    pub fn columns(&self) -> indexmap::map::Iter<'_, Rc<String>, Rc<Column>> {
        self.columns.iter()
    }

    pub fn set_constraint(&mut self, constraint: Constraint) -> &mut Table {
        self.constraints
            .insert(constraint.name(), Rc::new(constraint));

        self
    }

    pub fn constraint(&self, key: &str) -> Option<Rc<Constraint>> {
        self.constraints.get(&key.to_string()).cloned()
    }

    pub fn constraints(&self) -> std::collections::hash_map::Iter<'_, Rc<String>, Rc<Constraint>> {
        self.constraints.iter()
    }

    pub fn set_index(&mut self, index: Index) -> &mut Table {
        self.indexes.insert(index.name(), index);

        self
    }

    pub fn index(&self, key: &str) -> Option<&Index> {
        self.indexes.get(&key.to_string())
    }

    pub fn indexes(&self) -> indexmap::map::Iter<'_, Rc<String>, Index> {
        self.indexes.iter()
    }
}
