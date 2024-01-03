use database_reflection::metadata::consts::{METADATA_CASCADE, METADATA_ON_DELETE};
use database_reflection::metadata::WithMetadata;
use database_reflection::reflection::{Column, Constraint, SqlDatatype, SqlSigned};
use std::rc::Rc;

#[test]
fn test_constraint_integrity() {
    let column_local = Rc::new(Column::new(
        "local",
        "local_id",
        SqlDatatype::Int(10, SqlSigned::Unsigned),
    ));
    let column_foreign = Rc::new(Column::new(
        "foreign",
        "foreign_id",
        SqlDatatype::Int(10, SqlSigned::Unsigned),
    ));

    let mut constraint = Constraint::new("fk_local_1", column_local, column_foreign);
    constraint.set_meta(METADATA_ON_DELETE, METADATA_CASCADE);

    assert_eq!(constraint.name(), String::from("fk_local_1").into());
    assert_eq!(constraint.local().name(), String::from("local_id").into());
    assert_eq!(
        constraint.foreign().name(),
        String::from("foreign_id").into()
    );
    assert_eq!(
        constraint.meta(METADATA_ON_DELETE),
        Some(METADATA_CASCADE.to_string())
    );

    assert_eq!(constraint.key_pairs_count(), 1);

    let column_local = Rc::new(Column::new(
        "local",
        "another_local_id",
        SqlDatatype::Int(10, SqlSigned::Unsigned),
    ));
    let column_foreign = Rc::new(Column::new(
        "foreign",
        "another_foreign_id",
        SqlDatatype::Int(10, SqlSigned::Unsigned),
    ));

    constraint.add_key_pair(column_local, column_foreign);

    assert_eq!(constraint.key_pairs_count(), 2);
    assert_eq!(constraint.key_pairs().len(), 2);
}
