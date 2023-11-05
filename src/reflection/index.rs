use crate::reflection::column::Column;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Index<'n> {
    pub(super) name: &'n str,
    column: Rc<Column<'n>>, //&'n str,
    primary: bool,
    unique: bool,
}

impl<'n> Index<'n> {
    pub fn new(name: &'n str, column: Rc<Column<'n>>, primary: bool, unique: bool) -> Self {
        Index {
            name,
            column,
            primary,
            unique,
        }
    }

    pub fn name(&self) -> &str {
        self.name
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
