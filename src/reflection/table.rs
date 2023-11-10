use crate::metadata::WithMetadata;
use crate::reflection::column::Column;
use crate::reflection::constraint::Constraint;
use crate::reflection::index::Index;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Table<'n> {
    pub(super) name: &'n str,
    columns: IndexMap<&'n str, Rc<Column<'n>>>,
    constraints: HashMap<&'n str, Rc<Constraint<'n>>>,
    indexes: IndexMap<&'n str, Index<'n>>,
    metadata: HashMap<String, String>,
}

impl<'n> WithMetadata for Table<'n> {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl<'n> Table<'n> {
    pub fn new(name: &'n str) -> Table<'n> {
        Table {
            name,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn set_column(&mut self, column: Column<'n>) -> &mut Table<'n> {
        self.columns.insert(column.name, Rc::new(column));

        self
    }

    pub fn column(&self, key: &str) -> Option<Rc<Column<'n>>> {
        self.columns.get(key).cloned()
    }

    pub fn columns(&self) -> indexmap::map::Iter<'_, &'n str, Rc<Column<'n>>> {
        self.columns.iter()
    }

    pub fn set_constraint(&mut self, constraint: Constraint<'n>) -> &mut Table<'n> {
        self.constraints
            .insert(constraint.name, Rc::new(constraint));

        self
    }

    pub fn constraint(&self, key: &str) -> Option<Rc<Constraint<'n>>> {
        self.constraints.get(key).cloned()
    }

    pub fn constraints(&self) -> std::collections::hash_map::Iter<'_, &'n str, Rc<Constraint<'n>>> {
        self.constraints.iter()
    }

    pub fn set_index(&mut self, index: Index<'n>) -> &mut Table<'n> {
        self.indexes.insert(index.name, index);

        self
    }

    pub fn index(&self, key: &str) -> Option<&Index> {
        self.indexes.get(key)
    }

    pub fn indexes(&self) -> indexmap::map::Iter<'_, &'n str, Index<'n>> {
        self.indexes.iter()
    }
}
