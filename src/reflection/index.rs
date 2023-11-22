use crate::reflection::column::Column;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Index {
    name: Rc<String>,
    column: Rc<Column>,
    primary: bool,
    unique: bool,
}

impl Index {
    /// Create an index
    pub fn new(name: impl ToString, column: Rc<Column>, primary: bool, unique: bool) -> Self {
        Index {
            name: Rc::new(name.to_string()),
            column,
            primary,
            unique,
        }
    }

    /// Get index name
    pub fn name(&self) -> Rc<String> {
        self.name.clone()
    }

    /// Get column
    pub fn column(&self) -> &Column {
        &self.column
    }

    /// Get flag indicating whether the index is a primary key
    pub fn primary(&self) -> bool {
        self.primary
    }

    /// Get a flag indicating whether the index is unique
    pub fn unique(&self) -> bool {
        self.unique
    }
}
