use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert("apple");
    set.insert("banana");
    set.insert("cherry");

    for fruit in &set {
        println!("{}", fruit);
    }
}
