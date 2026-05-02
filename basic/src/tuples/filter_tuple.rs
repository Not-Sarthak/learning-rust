pub fn clone_filter_update(input: &Vec<(i32, bool)>) -> Vec<(i32, bool)> {
    let cloned = input.clone();
    let mut result = Vec::new();

    for (num, status) in cloned {
        if num % 2 == 0 {
            result.push((num, true))
        }
    }

    result
}

fn main() {
    let input = vec![(2, false), (3, false), (4, false)];
    println!("Original: {:?}, Filtered: {:?}", &input, clone_filter_update(&input) );
}