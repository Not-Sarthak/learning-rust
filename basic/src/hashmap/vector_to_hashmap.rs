use std::collections::HashMap;

fn main() {
    let data = vec![
        (10, 5),
        (20, 5),
        (30, 4),
    ];

    let map: HashMap<i32, i64> = data.into_iter().collect();

    println!("{:?}", map);
}

// build a `HashMap` from a vector by turning the values into key-value pairs first. suppose you want to track the index of each item in a vector, you can use the item as the key and its index as the value.
use std::collections::HashMap;

pub fn vector_to_index_map(v: Vec<i32>) -> HashMap<i32, usize> {
    let mut hm = HashMap::new();

    for i in 0..v.len() {
        hm.insert(v[i], i);
    }

    hm
}

fn main() {
    let data1 = vec![5, 7, 5, 9];
    let result1 = vector_to_index_map(data1);
    println!("Input: vec![5, 7, 5, 9], Output: {:?}", result1); // expected: {5: 2, 7: 1, 9: 3}

    let data2 = vec![1, 2, 3, 4, 5];
    let result2 = vector_to_index_map(data2);
    println!("Input: vec![1, 2, 3, 4, 5], Output: {:?}", result2); // expected: {1:0, 2:1, 3:2, 4:3, 5:4}

    let data3 = vec![10, 10, 10];
    let result3 = vector_to_index_map(data3);
    println!("Input: vec![10, 10, 10], Output: {:?}", result3); // expected: {10:2}
}
