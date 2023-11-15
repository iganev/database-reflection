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
    pub fn new(name: impl ToString, column: Rc<Column>, primary: bool, unique: bool) -> Self {
        Index {
            name: Rc::new(name.to_string()),
            column,
            primary,
            unique,
        }
    }

    pub fn name(&self) -> Rc<String> {
        self.name.clone()
    }

    pub fn column(&self) -> &Column {
        &self.column
    }

    pub fn primary(&self) -> bool {
        self.primary
    }

    pub fn unique(&self) -> bool {
        self.unique
    }
}
