// The Result enum lets you return either Ok value or Err value
// The Result enum is how you can do error handling in Rust

//Write a function that reads the contents of a file

use std::fs::read_to_string;

fn main() {
    let greeting_file_result = read_to_string(String::from("hello.txt"));

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        }
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }
}