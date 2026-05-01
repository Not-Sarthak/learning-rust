// to iterate through a vector, we can loop through 0..vector.len().
// however, sets don’t support indexing like that since sets don’t “index” the values they hold. 
// to loop through the items in a set, we need to convert the set to an iterator.
// syntax:
//      // define the set
//      let s = HashSet::from([2,4,8,10]);
//      // convert it to an iterator
//      let s_iter = s.into_iter();

use std::collections::HashSet;

fn main() {
	let s = HashSet::from([2,4,8,10]);
	
    let s_iter = s.into_iter();
	
	for item in s_iter {
		println!("{}", item);
	}
} 

// vectors can also be converted into iterators. we can then loop through the iterator instead of indexing each value of the vector.
fn main() {
	let v = Vec::from([2,4,8,10]);
	
    let v_iter = v.into_iter();
	
	for item in v_iter {
		println!("{}", item);
	}
} 

// converting a variable into an iterator consumes it. therefore, we can’t use the original variable after turning it into an iterator. 
// as a temporary solution, we can clone the vector before we turn it into an iterator.
// this doubles the memory usage — we’ll learn more optimally solutions later.
fn main() {
	let v = Vec::from([2,4,8,10]);
	
    let v_clone = v.clone();
    let v_iter = v_clone.into_iter();
	
	for item in v_iter {
		println!("{}", item);
	}
	
	println!("{:?}", v);
} 

// an iterator can be turned into a set or to a vector using the `.collect()` method. we need to explicitly specify the type we want .collect() to produce.
// vector to hashset
use std::collections::HashSet;

fn main() {
	let v = Vec::from([2,4,8,10]);
	let v_iter = v.into_iter();
	let s: HashSet<i32> = v_iter.collect();
    println!("{:?}", s);
} 

// hashset to vector
use std::collections::HashSet;

fn main() {
	let mut s = HashSet::new();
	s.insert(1);
	s.insert(2);
	s.insert(3);
	
	let v: Vec<i32> = s.into_iter().collect();
    println!("{:?}", v);
}

// hashset to hashset
use std::collections::HashSet;

fn main() {
	let mut s = HashSet::new();
	s.insert(1);
	s.insert(2);
	s.insert(3);
	
	let v: HashSet<i32> = s.into_iter().collect();
    println!("{:?}", v);
} 

