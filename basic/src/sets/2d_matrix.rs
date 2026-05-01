// implement the function mul_table that takes a non-negative integer n and returns a 2D vector representing an n×n multiplication table. if n is 0, return an empty vector

fn main() {
    let n = 4;
    let result = mul_table(n);
    println!("{:?}", result);
}

pub fn mul_table(n: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![];
    for i in 1..n+1 {
        let mut row = vec![];
        for j in 1..n+1 {
            row.push(i*j);
        }
        matrix.push(row);
    }
    matrix
} 

// filter sets by size
use std::collections::HashSet;

fn main() {
    let v1 = vec![HashSet::from([1,2,3,4]), HashSet::from([1,2]), HashSet::from([5,6,7])]; 
    let k1 = 3;
    let result1 = remove_smaller_than_k(&v1, k1);
    println!("Original: {:?}, k={}, Result: {:?}", v1, k1, result1);

    let v2 = vec![HashSet::from([1]), HashSet::from([2]), HashSet::from([3])];
    let k2 = 2;
    let result2 = remove_smaller_than_k(&v2, k2);
    println!("Original: {:?}, k={}, Result: {:?}", v2, k2, result2);
}

pub fn remove_smaller_than_k(v: &Vec<HashSet<i32>>, k: usize) -> Vec<HashSet<i32>> {
    let mut new_vec: Vec<HashSet<i32>> = vec![];
    for i in 0..v.len() {
        if v[i].len() >= k {
            new_vec.push(v[i].clone());
        }
    }
    new_vec
} 