use std::collections::HashSet;

pub fn collect_range_to_set(start: i32, end: i32) -> HashSet<i32> {
    let s:HashSet<i32> = (start..end).collect();
    s
}

fn main() {
    let v: HashSet<u32> = (0..10).collect();
    println!("{:?}", v);

    let result = collect_range_to_set(0, 10);
    println!("{:?}", result);
}
