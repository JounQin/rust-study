use std::collections::HashMap;
use crate::garden::vegetables::Asparagus;

pub mod garden;

fn _hash_map() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}", plant)
}
