// a HashMap lets you store key-value pairs, where each key maps to exactly one value.
use std::collections::HashMap;

fn main() {
    let hm: HashMap<usize, u32> = HashMap::new();
    println!("{:?}", hm); // {}
}
