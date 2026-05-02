fn main() {
    println!("1 -> {:?}", to_bool(1));
    println!("0 -> {:?}", to_bool(0));
    println!("5 -> {:?}", to_bool(5));
    println!("-1 -> {:?}", to_bool(-1));
}

pub fn to_bool(n: i32) -> Option<bool> {
    if n == 0 {
        Some(false)
    } else if n == 1 {
        Some(true)
    } else {
        None
    }
}