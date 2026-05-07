// a function odd_descending that takes a number end: i32, and returns a Vec of all odd numbers less than end starting from 1, listed in descending order.

pub fn odd_descending(end: i32) -> Vec<i32> {
    let mut v: Vec<i32> = vec![];
    for i in (0..end).rev() {
        if i % 2 != 0 {
            v.push(i);
        }
    }
    v
}

fn main() {
    let result = odd_descending(20);
    println!("{:?}", result);
}
