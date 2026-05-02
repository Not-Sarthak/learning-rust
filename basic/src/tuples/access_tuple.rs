// you can access tuple elements using dot notation followed by their position. positions start at 0.
fn main() {
    let pair: (bool, u32, i32) = (true, 2025, -5);
    println!("Status: {}, Year: {}, GPA: {}", pair.0, pair.1, pair.2);
}
