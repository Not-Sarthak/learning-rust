use std::collections::HashMap;

pub fn first_indices(v: Vec<i32>) -> HashMap<i32, usize> {
    let mut hm = HashMap::new();
    for i in 0..v.len() {
        if hm.get(&v[i]).is_none() {
            hm.insert(v[i], i);
        }
    }
    hm
}

fn main() {
    let nums1 = vec![10, 20, 10, 30, 20];
    println!(
        "Input: {:?}, Output: {:?}",
        nums1,
        first_indices(nums1.clone())
    );

    let nums2 = vec![7, 7, 7];
    println!(
        "Input: {:?}, Output: {:?}",
        nums2,
        first_indices(nums2.clone())
    );

    let nums3 = vec![9, 8, 7, 6];
    println!(
        "Input: {:?}, Output: {:?}",
        nums3,
        first_indices(nums3.clone())
    );

    let nums4 = vec![5, 3, 5, 10, 3, 7];
    println!(
        "Input: {:?}, Output: {:?}",
        nums4,
        first_indices(nums4.clone())
    );
}
