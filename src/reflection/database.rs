use crate::metadata::consts::{METADATA_CHARSET, METADATA_COLLATION};
use crate::metadata::WithMetadata;
use crate::reflection::column::Column;
use crate::reflection::constraint::{Constraint, ConstraintSide};
use crate::reflection::table::Table;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Database {
    name: String,
    tables: IndexMap<Arc<String>, Arc<Table>>,
    constraints: HashMap<Arc<String>, Arc<Constraint>>,
    metadata: HashMap<String, String>,
}

impl WithMetadata for Database {
    /// Borrow metadata container for reading
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Borrow metadata container for writing
    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl Database {
    /// Create a new database with a name
    pub fn new(name: impl ToString) -> Database {
        Database {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Get database name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Add a table to the database
    pub fn set_table(&mut self, mut table: Table) -> &mut Database {
        if table.meta(METADATA_CHARSET).is_none()
            && table.meta(METADATA_COLLATION).is_none()
            && self.meta(METADATA_CHARSET).is_some()
            && self.meta(METADATA_COLLATION).is_some()
        {
            table
                .set_meta(
                    METADATA_CHARSET,
                    self.meta(METADATA_CHARSET).unwrap_or_default(),
                )
                .set_meta(
                    METADATA_COLLATION,
                    self.meta(METADATA_COLLATION).unwrap_or_default(),
                );
        }

        self.tables.insert(table.name(), Arc::new(table));

        self
    }

    /// Get a table by name
    pub fn table(&self, key: &str) -> Option<Arc<Table>> {
        self.tables.get(&key.to_string()).cloned()
    }

    /// Get tables iterator
    pub fn tables(&self) -> indexmap::map::Iter<'_, Arc<String>, Arc<Table>> {
        self.tables.iter()
    }

    /// Add a constraint to the database
    pub fn set_constraint(&mut self, constraint: Constraint) -> &mut Database {
        self.constraints
            .insert(constraint.name(), Arc::new(constraint));

        self
    }

    /// Find a constraint by name
    pub fn constraint(&self, key: &str) -> Option<Arc<Constraint>> {
        #[allow(clippy::unnecessary_to_owned)]
        self.constraints.get(&key.to_string()).cloned()
    }

    /// Get constraints iterator
    pub fn constraints(
        &self,
    ) -> std::collections::hash_map::Iter<'_, Arc<String>, Arc<Constraint>> {
        self.constraints.iter()
    }

    /// Search for constraints by local or foreign table name
    pub fn constraints_by_table(
        &self,
        table: Arc<Table>,
        side: Option<ConstraintSide>,
    ) -> Vec<Arc<Constraint>> {
        self.constraints
            .values()
            .filter(|c| {
                if c.local().table() == table.name() && (side != Some(ConstraintSide::Foreign)) {
                    return true;
                }

                if c.foreign().table() == table.name() && (side != Some(ConstraintSide::Local)) {
                    return true;
                }

                false
            })
            .cloned()
            .collect::<Vec<Arc<Constraint>>>()
    }

    /// Search for constraints by local or foreign column
    pub fn constraints_by_column(
        &self,
        column: Arc<Column>,
        side: Option<ConstraintSide>,
    ) -> Vec<Arc<Constraint>> {
        self.constraints
            .values()
            .filter(|c| {
                if c.local() == column.as_ref() && (side != Some(ConstraintSide::Foreign)) {
                    return true;
                }

                if c.foreign() == column.as_ref() && (side != Some(ConstraintSide::Local)) {
                    return true;
                }

                false
            })
            .cloned()
            .collect::<Vec<Arc<Constraint>>>()
    }

    /// Find constraints by column name
    pub fn constraints_by_column_name(
        &self,
        column_name: Arc<String>,
        side: Option<ConstraintSide>,
    ) -> Vec<Arc<Constraint>> {
        self.constraints
            .values()
            .filter(|c| {
                if c.local().name() == column_name && (side != Some(ConstraintSide::Foreign)) {
                    return true;
                }

                if c.foreign().name() == column_name && (side != Some(ConstraintSide::Local)) {
                    return true;
                }

                false
            })
            .cloned()
            .collect::<Vec<Arc<Constraint>>>()
    }
}
