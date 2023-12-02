use std::rc::Rc;
use database_reflection::reflection::{Column, Constraint, Datatype};

#[test]
fn test_constraint_integrity() {
    let column_local = Rc::new(Column::new("local", "local_id", Datatype::Int(10)));
    let column_foreign = Rc::new(Column::new("foreign", "foreign_id", Datatype::Int(10)));

    let constraint = Constraint::new("fk_local_1", column_local, column_foreign);

    assert_eq!(constraint.name(), String::from("fk_local_1").into());
    assert_eq!(constraint.local().name(), String::from("local_id").into());
    assert_eq!(constraint.foreign().name(), String::from("foreign_id").into());
    assert_eq!(constraint.key_pairs_count(), 1);
}