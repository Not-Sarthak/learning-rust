use std::collections::HashMap;

pub fn count_occurrences(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut hm: HashMap<i32, i32> = HashMap::new();
    for num in v {
        if hm.get(&num).is_none() {
            hm.insert(num, 1);
        } else {
            let current_count = hm.get(&num).unwrap();
            hm.insert(num, current_count + 1);
        }
    }
    hm
}

fn main() {
    let data1 = vec![10, 15, 10, 20, 15, 25, 10];
    let map1 = count_occurrences(data1.clone());
    println!(
        "Input: vec![10, 15, 10, 20, 15, 25, 10], Output: {:?}",
        map1
    );
    // Expected: {10: 3, 15: 2, 20: 1, 25: 1}

    let data2 = vec![1, 1, 1, 1, 1];
    let map2 = count_occurrences(data2.clone());
    println!("Input: vec![1, 1, 1, 1, 1], Output: {:?}", map2);
    // Expected: {1: 5}

    let data3: Vec<i32> = vec![];
    let map3 = count_occurrences(data3.clone());
    println!("Input: vec![], Output: {:?}", map3);
    // Expected: {}
}
