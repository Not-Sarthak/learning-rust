// a function find_set_of_size_k that takes a vector of HashSets of integers and a size k, and returns the index of the first HashSet that has exactly k elements. if no such HashSet exists, return -1.
use std::collections::HashSet;

fn main() {
    let v1 = vec![
        HashSet::from([1]), 
        HashSet::from([2, 3]), 
        HashSet::from([4, 5, 6])
    ];
    println!("{:?}, find k=2 -> {}", v1, find_set_of_size_k(&v1, 2));
    println!("{:?}, find k=3 -> {}", v1, find_set_of_size_k(&v1, 3));
    println!("{:?}, find k=4 -> {}", v1, find_set_of_size_k(&v1, 4));

    let v2: Vec<HashSet<i32>> = vec![];
    println!("{:?}, find k=1 -> {}", v2, find_set_of_size_k(&v2, 1));
}

pub fn find_set_of_size_k(v: &Vec<HashSet<i32>>, k: usize) -> i32 {
    for i in 0..v.len() {
        if v[i].len() == k {
            return i as i32;
        }
    }
    -1
} 