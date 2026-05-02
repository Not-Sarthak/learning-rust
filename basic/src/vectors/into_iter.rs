pub fn check_even(v: Vec<u32>) -> Vec<(u32, bool)> {
    let v_iter = v.into_iter();
    let mut new_vec = vec![];
    for item in v_iter {
        if item % 2 == 0 {
            new_vec.push((item, true));
        } else {
            new_vec.push((item, false));
        }
    }

    new_vec
}

fn main() {
    let numbers = vec![0, 1, 2, 3];
    let result = check_even(numbers);
    println!("{:?}", result); // [(0, true), (1, false), (2, true), (3, false)]
}
