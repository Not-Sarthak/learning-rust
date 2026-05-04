// we can get a specific value from a HashMap using the `.get()` method.
// it takes the key as a reference and returns an Option type. syntax: .get(&key);
// the Option type will contain Some(value) if the key exists or None if it doesn’t.
use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<i32, i32> = HashMap::new();

    hm.insert(2, 5);
    hm.insert(4, 6);
    println!("{:?}", hm.get(&2));
    println!("{:?}", hm.get(&3));
}
