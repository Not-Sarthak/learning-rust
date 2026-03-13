// The String type, which is provided by Rust's standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type. When Rustaceans refer to "strings" in Rust, they might be referring to either the String or the string slice &str types, not just one of those types.

// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

//Strings
// 1. Create a String
fn main() {
    let name = String::from("Sarthak");
    println!("Name is {}", name);
}

// 2. Mutating a String 
fn main() {
    let mut name = String::from("Shah");
    name.push_str(" Sarthak");
    println!("Name is {}", name);
}

// 3. Deleting from a String
fn main() {
    let mut name = String::from("Sarthak");
    name.push_str(" Shah");
    println!("{}", name);

    name.replace_range(8..name.len(), "");
    println!("{}", name);
}