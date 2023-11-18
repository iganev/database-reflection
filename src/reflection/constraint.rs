use crate::metadata::WithMetadata;
use crate::reflection::column::Column;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq)]
pub enum ConstraintSide {
    Local,
    Foreign,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ConstraintKeyPair {
    pub local: Rc<Column>,
    pub foreign: Rc<Column>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Constraint {
    name: Rc<String>,
    key_pairs: Vec<ConstraintKeyPair>,
    metadata: HashMap<String, String>,
}

impl WithMetadata for Constraint {
    /// Borrow meadata container for reading
    fn get_metadata(&self) -> &HashMap<String, String> {
        &self.metadata
    }

    /// Borrow meadata container for writing
    fn get_metadata_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.metadata
    }
}

impl Constraint {
    /// Create a new constraint with at least one local and foreign column pair
    pub fn new(name: impl ToString, local: Rc<Column>, foreign: Rc<Column>) -> Self {
        Constraint {
            name: Rc::new(name.to_string()),
            key_pairs: vec![ConstraintKeyPair { local, foreign }],
            ..Default::default()
        }
    }

    /// Get constraint name
    pub fn name(&self) -> Rc<String> {
        self.name.clone()
    }

    /// Get local column, or local column from first pair
    pub fn local(&self) -> &Column {
        &self.key_pairs.first().unwrap().local
    }

    /// Get foreign column, or foreign column from first pair
    pub fn foreign(&self) -> &Column {
        &self.key_pairs.first().unwrap().foreign
    }

    /// Add a local/foreign column pair
    pub fn add_key_pair(&mut self, local: Rc<Column>, foreign: Rc<Column>) -> &mut Constraint {
        self.key_pairs.push(ConstraintKeyPair { local, foreign });

        self
    }

    /// Get column pairs iterator
    pub fn key_pairs(&self) -> Iter<'_, ConstraintKeyPair> {
        self.key_pairs.iter()
    }

    /// Get number of pairs
    pub fn key_pairs_count(&self) -> usize {
        self.key_pairs.len()
    }
}
