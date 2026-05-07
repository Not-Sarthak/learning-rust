// a function flatten_vector that takes a reference to a vector &Vec> and returns a set HashSet containing all the elements of the inner vectors.

use std::collections::HashSet;

pub fn flatten_vector(group: &Vec<Vec<i32>>) -> HashSet<i32> {
    let mut flat = HashSet::new();

    for inner_vec in group {
        for item in inner_vec {
            flat.insert(*item);
        }
    }
    flat
}

fn main() {
    let group = vec![vec![1], vec![2], vec![3], vec![4]];

    let flat = flatten_vector(&group);
    println!("{:?}", flat); // {1, 2, 3, 4}
}
