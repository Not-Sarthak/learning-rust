// Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

// Write a function that takes a string as an input and returns the view to the first word from it.
fn main() {
    let word = String::from("Hello world");

    let word2 = &word[0..5];
    // word.clear();   // This won't be allowed

    println!("{}", word2);
}

///////////////

fn main() {
    let name = String::from("Hello World");
    let ans = first_word(&name);
    println!("Answer is {}", ans);
}

fn first_word(str: &String) -> &str {
    let mut space_index = 0;
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        space_index = space_index + 1;
    }
    return &str[0..space_index];
}


// Three types of commonly used strings (there are actually much more)
fn main() {
    let name = String::from("Hello World"); // String type
    let string_slice = &name; // Has a `view` into the original string/is a reference
    let string_literal = "Hello"; // literal is also an &str but it points directly to an address in the binary
}

// Slices can also be applied to other collections like vectors/arrays