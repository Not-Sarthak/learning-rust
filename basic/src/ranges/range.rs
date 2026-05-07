// a range in Rust is a value that represents a sequence of numbers, starting from one number and ending at another.
// eg: the range 0..10 represents the numbers 0 through 9
// we can’t start a range from the upper bound to the lower bound, like 10..0. it must be in an increasing sequence from the lower bound to the upper.
fn main() {
    let my_range = 0..10;
    println!("{:?}", my_range);
}

// ranges get automatically converted to iterators
fn main() {
    println!("Looping with implicit into_iter():");
    for i in 0..10 {
        println!("{}", i);
    }

    println!("Looping with explicit into_iter():");
    for i in (0..10).into_iter() {
        println!("{}", i);
    }
}
