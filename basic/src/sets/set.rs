// a Set is a collection of unique elements. Unlike a List or Array:
//      a. Uniqueness: It cannot contain duplicate values. If you insert 10 twice, the set will still only contain one 10.
//      b. Unordered: The elements do not have a specific index or guaranteed order.

use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(10);
    println!("{:?}", set);
} 

// to check if an element is in the set, we use the .contains() method. note that .contains() method takes a reference as an argument. 

use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(10);
    set.insert(8);
    
    println!("{}", set.contains(&10));
    println!("{}", set.contains(&11));
} 

// the `.contains()` method still works if used on a reference to a set.
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(10);
    set.insert(8);
    
    println!("{}", (&set).contains(&10));
    println!("{}", (&set).contains(&11));
} 

// a function that returns true if the set contains any of the values from 0 to n - 1.
use std::collections::HashSet;

fn main() {
	let mut set = HashSet::new();
	set.insert(3);
	set.insert(5);
	let n = 6;
	
	let result = has_zero_to_n(&set, n);
	println!("Set {:?} has 0..{}? {}", set, n, result);

	let n2 = 3;
	let result2 = has_zero_to_n(&set, n2);
	println!("Set {:?} has 0..{}? {}", set, n2, result2);
}

pub fn has_zero_to_n(set: &HashSet<i32>, n: i32) -> bool {
	for element in 0..n {
        if set.contains(&element) {
            return true;
        }
    }
    false
} 

// check if any vector element exists in set
use std::collections::HashSet;

fn main() {
	let mut set = HashSet::new();
	set.insert(5);
	set.insert(7);
	let v1 = vec![3,5,9];
	let v2 = vec![1,2,3];
	
	let result1 = exists_in_set(&set, &v1);
	println!("Set {:?} has element from {:?}? {}", set, v1, result1);
	println!("{:?}", v1);
	println!("{:?}", set);

	let result2 = exists_in_set(&set, &v2);
	println!("Set {:?} has element from {:?}? {}", set, v2, result2);
}

pub fn exists_in_set(set: &HashSet<i32>, v: &Vec<i32>) -> bool {
	for element in 0..v.len() {
        if set.contains(&v[element]) {
            return true;
        }
    }
    false
} 

// you can use `.remove()` to remove an item from a set.
// it does not result in an error if you remove a non-existent item.
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(10);
    set.insert(8);
    println!("Before remove: {:?}", set);
    set.remove(&10);
    set.remove(&9);
    set.remove(&100);

    println!("After remove: {:?}", set);
    
    println!("Contains 10? {}", (&set).contains(&10));
    println!("Contains 8? {}", (&set).contains(&8));
} 

// you can clone a set just like you can clone a vector.
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert(3);
    
    let mut set2 = set.clone();
    set2.insert(4);
    
    println!("{:?}", set);
    println!("{:?}", set2);
} 