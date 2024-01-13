use crate::metadata::consts::{METADATA_CHARSET, METADATA_COLLATION, METADATA_FLAG_PRIMARY};
use crate::metadata::WithMetadata;
use crate::reflection::column::Column;
use crate::reflection::index::Index;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::slice::Iter;
use std::sync::Arc;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Table {
    name: Arc<String>,
    primary_key: Vec<Arc<String>>,
    columns: IndexMap<Arc<String>, Arc<Column>>,
    indexes: IndexMap<Arc<String>, Index>,
    metadata: HashMap<String, String>,
}

impl WithMetadata for Table {
    /// Borrow metadata container for reading
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Borrow metadata container for writing
    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl Table {
    /// Create a new empty table
    pub fn new(name: impl ToString) -> Table {
        Table {
            name: Arc::new(name.to_string()),
            ..Default::default()
        }
    }

    /// Get table name
    pub fn name(&self) -> Arc<String> {
        self.name.clone()
    }

    /// Add a new column to the table
    pub fn set_column(&mut self, mut column: Column) -> &mut Table {
        if column.datatype().is_text()
            && column.meta(METADATA_CHARSET).is_none()
            && column.meta(METADATA_COLLATION).is_none()
            && self.meta(METADATA_CHARSET).is_some()
            && self.meta(METADATA_COLLATION).is_some()
        {
            column
                .set_meta(
                    METADATA_CHARSET,
                    self.meta(METADATA_CHARSET).unwrap_or_default(),
                )
                .set_meta(
                    METADATA_COLLATION,
                    self.meta(METADATA_COLLATION).unwrap_or_default(),
                );
        }

        if column.meta_flag(METADATA_FLAG_PRIMARY) && !self.primary_key.contains(&column.name()) {
            self.primary_key.push(column.name());
        }

        self.columns.insert(column.name(), Arc::new(column));

        self
    }

    /// Find a column by name
    pub fn column(&self, key: &str) -> Option<Arc<Column>> {
        self.columns.get(&key.to_string()).cloned()
    }

    /// Get columns iterator
    pub fn columns(&self) -> indexmap::map::Iter<'_, Arc<String>, Arc<Column>> {
        self.columns.iter()
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

    /// Find an index by column name
    pub fn index_by_column_name(&self, column_name: Arc<String>) -> Option<Index> {
        self.indexes
            .iter()
            .find(|(_, c)| c.column().name() == column_name)
            .map(|(_, c)| c.clone())
    }

    /// Find an index by column
    pub fn index_by_column(&self, column: &Column) -> Option<Index> {
        self.indexes
            .iter()
            .find(|(_, c)| c.column() == column)
            .map(|(_, c)| c.clone())
    }

    /// Get indexes iterator
    pub fn indexes(&self) -> indexmap::map::Iter<'_, Arc<String>, Index> {
        self.indexes.iter()
    }

    /// Get number of primary keys
    pub fn primary_key_count(&self) -> usize {
        self.primary_key.len()
    }

    /// Get primary key, or first primary key column name
    pub fn primary_key(&self) -> Option<Arc<String>> {
        self.primary_key.first().cloned()
    }

    /// Get primary key column or first primary key column
    pub fn primary_key_column(&self) -> Option<Arc<Column>> {
        self.primary_key
            .first()
            .map(|k| self.columns.get(k).cloned())
            .unwrap()
    }

    /// Get primary keys iterator
    pub fn primary_keys(&self) -> Iter<'_, Arc<String>> {
        self.primary_key.iter()
    }

    /// Get primary key columns
    pub fn primary_key_columns(&self) -> Vec<Arc<Column>> {
        self.columns
            .iter()
            .filter(|kv| self.primary_key.contains(kv.0))
            .map(|kv| kv.1.clone())
            .collect::<Vec<Arc<Column>>>()
    }
}
