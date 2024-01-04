use database_reflection::reflection::{Column, Index, SqlDatatype, SqlSigned};
use std::sync::Arc;

#[test]
fn test_index_integrity() {
    let column = Arc::new(Column::new(
        "local",
        "local_id",
        SqlDatatype::Int(10, SqlSigned::Unsigned),
    ));

    let index = Index::new("ind_local_1", column, true, false);

    assert_eq!(index.name(), String::from("ind_local_1").into());
    assert_eq!(index.column().name(), String::from("local_id").into());
    assert!(index.primary());
    assert!(!index.unique());
}
