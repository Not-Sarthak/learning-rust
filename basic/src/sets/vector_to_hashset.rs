use std::collections::HashSet;

fn main() {
    let v1 = vec![10i64, -20i64, 500i64, i32::MAX as i64, i32::MIN as i64];
    println!("Original i64: {:?}", v1);
    println!("Casted i32 set: {:?}", safe_cast_vec_to_set(&v1));

    let v2 = vec![i32::MAX as i64 + 1, i32::MIN as i64 - 1];
    println!("Original i64: {:?}", v2);
    println!("Casted i32 set: {:?}", safe_cast_vec_to_set(&v2));

    let v3: Vec<i64> = vec![];
    println!("Original i64: {:?}", v3);
    println!("Casted i32 set: {:?}", safe_cast_vec_to_set(&v3));
}

pub fn safe_cast_vec_to_set(v: &Vec<i64>) -> HashSet<i32> {
    let mut set = HashSet::new();

    for element in 0..v.len() {
        if v[element] <= i32::MAX as i64 && v[element] >= i32::MIN as i64{
            set.insert(v[element] as i32);
        } 
    }
    set
} 

// remove vector element from hashset
use std::collections::HashSet;

fn main() {
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(4);
    set.insert(5);

    let v_to_remove = vec![2, 4, 6];

    println!("Original set: {:?}", set);
    println!("Vector to remove: {:?}", v_to_remove);
    
    let result_set = remove_vector_elements(&set, &v_to_remove);
    
    println!("Result set: {:?}", result_set);
    println!("Original set unchanged: {:?}", set);
}

pub fn remove_vector_elements(set: &HashSet<i32>, v: &Vec<i32>) -> HashSet<i32> {
    let mut new_set = set.clone();
    for element in 0..v.len() {
        new_set.remove(&v[element]);
    }
    new_set
} 

// a function that consumes a vector v and returns a set containing its items
use std::collections::HashSet;

fn main() {
    let v = vec![1, 2, 2, 3, 1, 4, 5, 4];
    println!("Original vector: {:?}", v);
    let set = vector_to_set(v);
    println!("Resulting set: {:?}", set);
}

pub fn vector_to_set(v: Vec<i32>) -> HashSet<i32> {
    let mut s = HashSet::new();
    for i in 0..v.len() {
        s.insert(v[i]);
    }
    s
} 