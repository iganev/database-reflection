use database_reflection::reflection::{Column, SqlDatatype, Index};
use std::rc::Rc;

#[test]
fn test_index_integrity() {
    let column = Rc::new(Column::new("local", "local_id", SqlDatatype::Int(10)));

    let index = Index::new("ind_local_1", column, true, false);

    assert_eq!(index.name(), String::from("ind_local_1").into());
    assert_eq!(index.column().name(), String::from("local_id").into());
    assert!(index.primary());
    assert!(!index.unique());
}
