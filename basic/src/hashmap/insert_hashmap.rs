// to store data in a HashMap we apply the in-built `.insert()` method on a mutable HashMap.
use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<i32, bool> = HashMap::new();
    hm.insert(2, true);
    hm.insert(3, false);
    hm.insert(4, true);
    hm.insert(5, false);

    println!("{:?}", hm);
}

// HashMap doesn’t hold duplicate keys. if you insert the same key twice, the old value is replaced.
use std::collections::HashMap;

fn main() {
	let mut hm: HashMap<i32, Option<i32>> = HashMap::new();
	
	hm.insert(2,Some(0));
	hm.insert(3,None);
	hm.insert(2,Some(4));
	
  println!("{:?}", hm); // {3: None, 2: Some(4)}
}

// another example:
use std::collections::HashMap;

fn main() {
    let mut hm: HashMap<i32, bool> = HashMap::new();
    hm.insert(1, false);
    hm.insert(2, false);
    hm.insert(1, true);

    println!("{:?}", hm);
}
