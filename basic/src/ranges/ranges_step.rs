// if we want to step in increments of a certain number, we can call .step_by(n) on a range to skip values in steps of n
// this allows you to control the increment used when iterating.
use std::collections::HashSet;

pub fn range_step_to_30(step: i32) -> HashSet<i32> {
    (0..30).step_by(step as usize).collect()
}

fn main() {
    let v: Vec<i32> = (0..10).step_by(2).collect();
    println!("{:?}", v);

    let result = range_step_to_30(5);
    println!("{:?}", result);
}
