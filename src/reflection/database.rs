use crate::metadata::consts::{METADATA_CHARSET, METADATA_COLLATION};
use crate::metadata::WithMetadata;
use crate::reflection::column::Column;
use crate::reflection::constraint::{Constraint, ConstraintSide};
use crate::reflection::table::Table;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Database {
    name: String,
    tables: IndexMap<Rc<String>, Rc<Table>>,
    constraints: HashMap<Rc<String>, Rc<Constraint>>,
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
        if table.meta(METADATA_CHARSET) == None
            && table.meta(METADATA_COLLATION) == None
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

        for (constraint_name, constraint) in table.constraints() {
            self.constraints
                .insert(constraint_name.clone(), constraint.clone());
        }

        self.tables.insert(table.name(), Rc::new(table));

        self
    }

    /// Get a table by name
    pub fn table(&self, key: &str) -> Option<Rc<Table>> {
        self.tables.get(&key.to_string()).cloned()
    }

    /// Get tables iterator
    pub fn tables(&self) -> indexmap::map::Iter<'_, Rc<String>, Rc<Table>> {
        self.tables.iter()
    }

    /// Get constraints iterator
    pub fn constraints(&self) -> std::collections::hash_map::Iter<'_, Rc<String>, Rc<Constraint>> {
        self.constraints.iter()
    }

    /// Search for constraints by local or foreign table name
    pub fn constraints_by_table(
        &self,
        table: Rc<Table>,
        side: Option<ConstraintSide>,
    ) -> Vec<Rc<Constraint>> {
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
            .collect::<Vec<Rc<Constraint>>>()
    }

    /// Search for constraints by local or foreign column
    pub fn constraints_by_column(
        &self,
        column: Rc<Column>,
        side: Option<ConstraintSide>,
    ) -> Vec<Rc<Constraint>> {
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
            .collect::<Vec<Rc<Constraint>>>()
    }
}
