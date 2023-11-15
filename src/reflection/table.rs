use crate::metadata::consts::METADATA_FLAG_PRIMARY;
use crate::metadata::WithMetadata;
use crate::reflection::column::Column;
use crate::reflection::constraint::Constraint;
use crate::reflection::index::Index;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Table {
    name: Rc<String>,
    primary_key: Vec<Rc<String>>,
    columns: IndexMap<Rc<String>, Rc<Column>>,
    constraints: HashMap<Rc<String>, Rc<Constraint>>,
    indexes: IndexMap<Rc<String>, Index>,
    metadata: HashMap<String, String>,
}

impl WithMetadata for Table {
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
        if column.meta_flag(METADATA_FLAG_PRIMARY) && !self.primary_key.contains(&column.name()) {
            self.primary_key.push(column.name());
        }

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
        if index.primary() && !self.primary_key.contains(&index.column().name()) {
            self.primary_key.push(index.column().name());
        }

        self.indexes.insert(index.name(), index);

        self
    }

    pub fn index(&self, key: &str) -> Option<&Index> {
        self.indexes.get(&key.to_string())
    }

    pub fn indexes(&self) -> indexmap::map::Iter<'_, Rc<String>, Index> {
        self.indexes.iter()
    }

    pub fn primary_key_count(&self) -> usize {
        self.primary_key.len()
    }

    pub fn primary_key(&self) -> Option<Rc<String>> {
        self.primary_key.first().cloned()
    }

    pub fn primary_key_column(&self) -> Option<Rc<Column>> {
        self.primary_key
            .first()
            .map(|k| self.columns.get(k).cloned())
            .unwrap()
    }

    pub fn primary_keys(&self) -> Iter<'_, Rc<String>> {
        self.primary_key.iter()
    }

    pub fn primary_key_columns(&self) -> Vec<Rc<Column>> {
        self.columns
            .iter()
            .filter(|kv| self.primary_key.contains(kv.0))
            .map(|kv| kv.1.clone())
            .collect::<Vec<Rc<Column>>>()
    }
}
