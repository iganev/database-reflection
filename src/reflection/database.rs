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
    pub(crate) name: String,
    tables: IndexMap<Rc<String>, Rc<Table>>,
    constraints: HashMap<Rc<String>, Rc<Constraint>>,
    metadata: HashMap<String, String>,
}

impl<'n> WithMetadata for Database {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl Database {
    pub fn new(name: impl ToString) -> Database {
        Database {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_table(&mut self, table: Table) -> &mut Database {
        for (constraint_name, constraint) in table.constraints() {
            self.constraints.insert(constraint_name.clone(), constraint.clone());
        }

        self.tables.insert(table.name(), Rc::new(table));

        self
    }

    pub fn table(&self, key: &str) -> Option<Rc<Table>> {
        self.tables.get(&key.to_string()).cloned()
    }

    pub fn tables(&self) -> indexmap::map::Iter<'_, Rc<String>, Rc<Table>> {
        self.tables.iter()
    }

    pub fn constraints(&self) -> std::collections::hash_map::Iter<'_, Rc<String>, Rc<Constraint>> {
        self.constraints.iter()
    }

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
