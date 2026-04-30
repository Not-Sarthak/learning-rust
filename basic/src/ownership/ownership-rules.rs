/*
Ownership Rules:
1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped
*/

fn create_string() {
    let s: String = String::from("Hello");
    println!("{}", s);
}

fn main(){
    create_string();
}

// fn create_string() {
//     let s1: String = String::from("Hello");
//     let s2: String = s1;
//     println!("{}", s1);   //Error
// }

// fn main(){
//     create_string();
// }
