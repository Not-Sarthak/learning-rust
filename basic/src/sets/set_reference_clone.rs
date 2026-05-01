// you can clone a reference to a set, just like a vector.
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(3);
    
    let set_ref = &set; // create a reference
    let mut set2 = set_ref.clone();
    set2.insert(4);
    
    // set is unchanged because set2 is a clone
    println!("{:?}", set);
    println!("{:?}", set2);
} 