// Write a function fib that finds the fibonnaci of a number it takes as input

fn get_string_length(s: &str) -> usize {
    return s.chars().count();
    //s.chars().count()
}

fn main() {
    let my_string = String::from("Hello, world");
    let length = get_string_length(&my_string);
    println!("Characters: {}", length);
}