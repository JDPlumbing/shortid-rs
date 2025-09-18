use shortid_rs::{short_code_from_uuid, unique_short_code};
use uuid::Uuid;
use std::collections::HashSet;

#[test]
fn can_generate_short_code() {
    let uuid = Uuid::new_v4();
    let code = short_code_from_uuid(&uuid);
    assert_eq!(code.len(), 6);
}

#[test]
fn collision_resolves_with_dash_or_underscore() {
    let uuid = Uuid::nil();
    let mut existing = HashSet::new();

    let first = unique_short_code(&uuid, &mut existing);
    let second = unique_short_code(&uuid, &mut existing);

    assert!(first != second);
    assert!(second.contains('-') || second.contains('_'));
}
