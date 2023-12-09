use std::collections::HashSet;

fn main() {
    let mut key_val_pair_hashset = HashSet::new();

    // insert
    key_val_pair_hashset.insert("Gourab");
    key_val_pair_hashset.insert("Gourab");
    key_val_pair_hashset.insert("Gourab");

    //.get()

    //.contains

    //.iter()

    //.remove

    println!("{:?}", key_val_pair_hashset.len());

}