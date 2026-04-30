// The Option enum lets you return either Some value or None value.

/*<
pub enum Option<T> {
    None,
    Some<T>
}
*/

/*
fn main() {
    let index = find_first_a(String::from("Sarthak"));

    match index {
        Some(value) => println!("Index is {}", value),
        None => println!("a not found!"),
    };
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}
*/

fn main() {
    let index = find_first_a(String::from("Sarthak"));

    match index {
        CustomOption::Some(value) => println!("Index is {}", value),
        CustomOption::None => println!("a not found!"),
    };
}

enum CustomOption {
    Some(i32),
    None,
}

fn find_first_a(s: String) -> CustomOption {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return CustomOption::Some(index as i32);
        }
    }

    return CustomOption::None;
}