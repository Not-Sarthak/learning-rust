// sometimes you want to count from the end of a range back to the beginning.
// you can reverse a range by calling .rev() on it.
// this flips the order of values in the range.

pub fn reverse_range(start: i32, end: i32) -> Vec<i32> {
    (start..=end).rev().collect()
}

fn main() {
    let v1: Vec<i32> = (8..=4).rev().collect(); // ❌ Wrong
    let v2: Vec<i32> = (4..=8).rev().collect();
    println!("V1: {:?}, V2: {:?} ", v1, v2);

    let result = reverse_range(2, 7);
    println!("{:?}", result);
}


// Output:
// V1: [], V2: [8, 7, 6, 5, 4] 
// [7, 6, 5, 4, 3, 2]

// .rev() & .step_by()
// the order in which we call .step_by() and .rev() can change the result.
// that’s because .step_by(n) always starts from the first value of the iterator, and .rev() flips the start and end of the range.
pub fn reverse_range_with_step(step_by: usize) -> Vec<i32> {
    let range = (0..=10).rev().step_by(step_by);
    range.collect()
}

fn main() {
    let a: Vec<i32> = (1..9).step_by(3).rev().collect();
    println!("{:?}", a);

    let b: Vec<i32> = (1..9).rev().step_by(3).collect();
    println!("{:?}", b);

    println!("{:?}", reverse_range_with_step(2));
}

// Output:
// [7, 4, 1]
// [8, 5, 2]
// [10, 8, 6, 4, 2, 0]