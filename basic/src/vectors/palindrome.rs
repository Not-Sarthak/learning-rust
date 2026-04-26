fn main() {
    let v = vec![1, 2, 3, 2, 1];

    let result = is_palindrome(v);
    println!("{}", result);
}

pub fn is_palindrome(v: Vec<i32>) -> bool {
    for i in 0..v.len() / 2 {
        if v[i] != v[v.len() - 1 - i] {
            return false;
        }
    }
    true
}