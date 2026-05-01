use std::collections::HashSet;

fn main() {
    let v = vec![HashSet::from([1,2,3]), HashSet::from([4,5]), HashSet::from([7])];
    let result = largest_set(&v);
    println!("{:?} -> Largest set size: {}", v, result);

    let v_empty: Vec<HashSet<i32>> = vec![];
    let result_empty = largest_set(&v_empty);
    println!("{:?} -> Largest set size: {}", v_empty, result_empty);

    let v_single = vec![HashSet::from([10, 20])];
    let result_single = largest_set(&v_single);
    println!("{:?} -> Largest set size: {}", v_single, result_single);
}

pub fn largest_set(v: &Vec<HashSet<i32>>) -> usize {
    let mut max = 0;
    for element in 0..v.len() {
        if v[element].len() > max {
            max = v[element].len();
        }
    }
    return max;
} 