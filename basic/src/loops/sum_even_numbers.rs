fn main() {
    let start = 0;
    let end = 5;

    let result = sum_evens(start, end);
    println!("{}", result);
}

pub fn sum_evens(start: u32, end: u32) -> u32 {
    let mut sum = 0;
    for i in start..end {
        if i%2 == 0 {
            sum += i
        }
    }
    sum
} 