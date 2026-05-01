use std::collections::HashSet;

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = vec![3, 7, 8];

    println!("{:?} and {:?} disjoint? {}", v1, v2, is_disjoint(&v1, &v2));
    println!("{:?} and {:?} disjoint? {}", v1, v3, is_disjoint(&v1, &v3));
    println!("{:?} and {:?} disjoint? {}", v2, v3, is_disjoint(&v2, &v3));
    println!("{:?} and {:?} disjoint? {}", v1, v1, is_disjoint(&v1, &v1));
}

pub fn is_disjoint(v1: &Vec<i32>, v2: &Vec<i32>) -> bool {
    let mut set = HashSet::new();

    for i in 0..v1.len() {
        set.insert(v1[i]);
    }

    for j in 0..v2.len() {
        if set.contains(&v2[j]) {
            return false;
        }
    }

    true
} 