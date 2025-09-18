use shortid_rs::{unique_short_code};
use uuid::Uuid;
use std::collections::HashSet;

fn main() {
    let mut existing = HashSet::new();
    for _ in 0..10 {
        let uuid = Uuid::new_v4();
        let code = unique_short_code(&uuid, &mut existing);
        println!("{} -> {}", uuid, code);
    }
}
