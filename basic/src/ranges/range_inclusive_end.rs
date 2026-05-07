// so far we’ve seen ranges that do not include their end value.
// we can specify that the range is inclusive of its end value by using the start..=endsyntax.

pub fn collect_and_sum_range(start: i32, end: i32) -> Vec<i32> {
    let mut v: Vec<i32> = (start..=end).collect();
    let mut sum = 0;
    
    for i in 0..v.len() {
        sum += v[i];
    }

    v.push(sum);

    v
}

fn main() {
    for i in 1..=5 {
        println!("{}", i);
    }

    let result = collect_and_sum_range(4, 8);
    println!("{:?}", result);
}
