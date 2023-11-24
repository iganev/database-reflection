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
    /// Borrow meadata container for reading
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Borrow meadata container for writing
    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl Table {
    /// Create a new empty table
    pub fn new(name: impl ToString) -> Table {
        Table {
            name: Rc::new(name.to_string()),
            ..Default::default()
        }
    }

    /// Get table name
    pub fn name(&self) -> Rc<String> {
        self.name.clone()
    }

    /// Add a new column to the table
    pub fn set_column(&mut self, column: Column) -> &mut Table {
        if column.meta_flag(METADATA_FLAG_PRIMARY) && !self.primary_key.contains(&column.name()) {
            self.primary_key.push(column.name());
        }

        self.columns.insert(column.name(), Rc::new(column));

        self
    }

    /// Find a column by name
    pub fn column(&self, key: &str) -> Option<Rc<Column>> {
        self.columns.get(&key.to_string()).cloned()
    }

    /// Get columns iterator
    pub fn columns(&self) -> indexmap::map::Iter<'_, Rc<String>, Rc<Column>> {
        self.columns.iter()
    }

    /// Add a new constraint
    pub fn set_constraint(&mut self, constraint: Constraint) -> &mut Table {
        self.constraints
            .insert(constraint.name(), Rc::new(constraint));

        self
    }

    /// Find a constraint by name
    pub fn constraint(&self, key: &str) -> Option<Rc<Constraint>> {
        self.constraints.get(&key.to_string()).cloned()
    }

    /// Find a constraint by local column name
    pub fn constraint_by_column_name(&self, column_name: Rc<String>) -> Option<Rc<Constraint>> {
        self.constraints
            .iter()
            .find(|(_, c)| c.local().name() == column_name)
            .map(|(_, c)| c.clone())
    }

    /// Get constraints iterator
    pub fn constraints(&self) -> std::collections::hash_map::Iter<'_, Rc<String>, Rc<Constraint>> {
        self.constraints.iter()
    }

    /// Add a new index
    pub fn set_index(&mut self, index: Index) -> &mut Table {
        if index.primary() && !self.primary_key.contains(&index.column().name()) {
            self.primary_key.push(index.column().name());
        }

        self.indexes.insert(index.name(), index);

        self
    }

    /// Find an index by name
    pub fn index(&self, key: &str) -> Option<&Index> {
        self.indexes.get(&key.to_string())
    }

    /// Get indexes iterator
    pub fn indexes(&self) -> indexmap::map::Iter<'_, Rc<String>, Index> {
        self.indexes.iter()
    }

    /// Get number of primary keys
    pub fn primary_key_count(&self) -> usize {
        self.primary_key.len()
    }

    /// Get primary key, or first primary key column name
    pub fn primary_key(&self) -> Option<Rc<String>> {
        self.primary_key.first().cloned()
    }

    /// Get primary key column or first primary key column
    pub fn primary_key_column(&self) -> Option<Rc<Column>> {
        self.primary_key
            .first()
            .map(|k| self.columns.get(k).cloned())
            .unwrap()
    }

    /// Get primary keys iterator
    pub fn primary_keys(&self) -> Iter<'_, Rc<String>> {
        self.primary_key.iter()
    }

    /// Get primary key columns
    pub fn primary_key_columns(&self) -> Vec<Rc<Column>> {
        self.columns
            .iter()
            .filter(|kv| self.primary_key.contains(kv.0))
            .map(|kv| kv.1.clone())
            .collect::<Vec<Rc<Column>>>()
    }
}
