pub fn count_even_numbers(v: &Vec<i32>) -> (usize, usize) {
    let mut count = 0;
    for i in v.into_iter() {
        if i%2==0 {
            count+=1;
        }
    }
    (count as usize, v.len())
}

fn main() {
    let numbers = vec![2, 5, 8, 9, 12, 15];
    let result = count_even_numbers(&numbers);
    println!("Even numbers: {}, Length of vector: {}", result.0, result.1);
}

// Output: Even numbers: 3, Length of vector: 6