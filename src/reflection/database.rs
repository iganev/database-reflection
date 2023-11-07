use crate::metadata::WithMetadata;
use crate::reflection::column::Column;
use crate::reflection::constraint::{Constraint, ConstraintSide};
use crate::reflection::table::Table;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use std::collections::HashMap;
use std::rc::Rc;

#[serde_as]
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Database<'n> {
    pub(crate) name: String,
    #[serde(borrow = "'n")]
    tables: IndexMap<&'n str, Rc<Table<'n>>>,
    #[serde(borrow = "'n")]
    constraints: HashMap<&'n str, Rc<Constraint<'n>>>,
    metadata: HashMap<String, String>,
}

impl<'n> WithMetadata for Database<'n> {
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl<'n> Database<'n> {
    pub fn new(name: impl ToString) -> Database<'n> {
        Database {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn name(&'n self) -> &'n str {
        &self.name
    }

    pub fn set_table(&mut self, table: Table<'n>) -> &mut Database<'n> {
        for (constraint_name, constraint) in table.constraints() {
            self.constraints.insert(constraint_name, constraint.clone());
        }

        self.tables.insert(table.name, Rc::new(table));

        self
    }

    pub fn table(&self, key: &str) -> Option<Rc<Table<'n>>> {
        self.tables.get(key).cloned()
    }

    pub fn tables(&self) -> indexmap::map::Iter<'_, &'n str, Rc<Table<'n>>> {
        self.tables.iter()
    }

    pub fn constraints(&self) -> std::collections::hash_map::Iter<'_, &'n str, Rc<Constraint<'n>>> {
        self.constraints.iter()
    }

    pub fn constraints_by_table(
        &self,
        table: Rc<Table<'n>>,
        side: Option<ConstraintSide>,
    ) -> Vec<Rc<Constraint<'n>>> {
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
        column: Rc<Column<'n>>,
        side: Option<ConstraintSide>,
    ) -> Vec<Rc<Constraint<'n>>> {
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
