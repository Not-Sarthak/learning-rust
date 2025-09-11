/*
Lifetimes:
Lifetimes are hard to digest.
Takes a lot of time to understand why they are needed
Lot of times the compiler will help you and guide you in the right direction.
*/

// Write a function that takes two strings as an input and returns the bigger amongst them
fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

// fn main() {
//     let longest_str;
//     let str1 = String::from("sarthak");
//     let str2 = String::from("shah");

//     longest_str = longest(str1, str2);
//     println!("{}", longest_str);
// }

fn main() {
    let longest_str;
    let str1 = String::from("shah");
    {
        let str2 = String::from("sarthak");
        longest_str = longest(str1, str2);
    }
    println!("{}", longest_str);
}

// Write a function that takes two string references as an input and returns the bigger amongst them

/*
Error

fn longest(a: &str, b: &str) -> &str {
    if a.len() > b.len() {
        return a;
    } else {
        return b;
    }
}

fn main() {
    let longest_str;
    let str1 = String::from("sarthak");
    let str2 = String::from("shah");

    longest_str = longest(str1, str2);
    println!("{}", longest_str);
}
*/

/* Solution: Using Lifetimes */
// Write a function that takes two string references as an input and returns the bigger amongst them

// <'a> is very similar to generic and is called a generic lifetime annotation
// 'a describe a relationship between the lifetimes of input args (parameters) and output args (return value). It says that the `return type` will be valid as long as both the arguments are valid or more technically the shorter lifetime is what the return type will have.
fn longest<'a>(str_one: &'a str, str_two: &'a str) -> &'a str {
    if str_one.len() > str_two.len() {
        str_one
    } else {
        str_two
    }
}

fn main() {
    let str1 = String::from("sarthak");
    let str2 = String::from("shah");

    let longest_str = longest(&str1, &str2);
    println!("{}", longest_str);
}


// Structs with lifetimes

/* Error */
// struct User {
//     name: &str,
// }

// fn main() {
//     let name = String::from("sarthak");
//     let user = User { name: &name };

//     println!("{}", user.name);
// }

/* Solution */
struct User<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("sarthak");
    let user = User { name: &name };

    println!("{}", user.name);
}
// Why do we need structs with references to have a lifetime parameter?
// So we know how long the `struct` can live


// Display Trait