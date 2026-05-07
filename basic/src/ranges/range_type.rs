// you can simplify this by passing the range itself as an argument using Rust’s built‐in `Range` type.

use std::collections::HashSet;
use std::ops::Range;

pub fn collect_range_values(r: Range<usize>) -> HashSet<usize> {
    r.collect()
}

fn main() {
    let my_range: Range<i32> = 0..10;
    println!("{:?}", my_range);

    let values = collect_range_values(5..10);
    println!("{:?}", values);
}

// a function sum_even_range that takes a range of i32 and returns the sum of all even numbers in that range.
use std::ops::Range;

pub fn sum_even_range(my_range: Range<i32>) -> i32 {
    let mut sum = 0;
    for i in my_range {
        if i % 2 == 0 {
            sum += i;
        }
    }
    sum
}

fn main() {
    let total = sum_even_range(0..5);
    println!("Sum: {}", total);
}
