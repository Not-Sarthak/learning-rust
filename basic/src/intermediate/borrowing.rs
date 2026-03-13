// fn main() {
//     let s1 = String::from("Sarthak");
//     let s2 = &s1;
//     print!("{} and {}", s1, s2);
// }

fn main() {
    let s1 = String::from("Sarthak");
    do_something(&s1);   // Borrowing
    println!("Name is: {}", s1);
}

fn do_something(s2: &String) {   // Borrowing
    println!("{}", s2);
}

// What if you want the borrow to mutate the value?
// fn main() {
//     let mut s1 = String::from("Sarthak");
//     do_something(&mut s1);   // Borrowing
//     println!("Name is: {}", s1);
// }

// fn do_something(s2: &mut String) {   // Borrowing
//     s2.push_str(" Shah");
//     println!("{}", s2);
// }


/* 
The Rules of References:
1. At any given time, you can have either one mutable reference or any number of immutable references.
2. References must always be valid.
*/
