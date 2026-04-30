// In Rust, all variables are immutable by default. You have to explicityly set them as mut.

fn main() {
    let mut name: String = String::from("Hello");
    name.push_str("World");
    print!("{}", name);

    let mut a: u32 = 10;
    a = 20;
    print!("{}", a);
}