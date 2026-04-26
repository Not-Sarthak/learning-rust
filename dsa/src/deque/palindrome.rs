fn palindrome_checker(pal: &str) -> bool {
    let mut d = Deque::new(pal.len());

    for c in pal.chars(){
        let _r = d.add_rear(c);
    }

    let mut is_pal = true;

    while d.size() > 1 && is_pal {
        let head = d.remove_front();

        let tail = d.remove_rear();

        if head != tail {
            is_pal = false;
        }
    }
    is_pal
}

// fn main() {
//     let pal = "rustsur";
//     let is_pal = palindrome_checker(pal);
//     println!("{pal} is palindrome string: {is_pal}");
//     // rustsur is palindrome string: true

//     let pal = "panda";
//     let is_pal = palindrome_checker(pal);
//     println!("{pal} is palindrome string: {is_pal}");
//     // panda is palindrome string: false
// }