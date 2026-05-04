// you can pass a HashMap to a function just like every other type.
// to do this, define the function’s parameter with the expected type of HashMap.
// eg:
// pub fn contains4(hm: HashMap<i32, i32>) -> bool {
//     hm.get(&4).is_some()
// }
use std::collections::HashMap;

pub fn does_k_exist(hm: HashMap<i32, i64>, k: &i32) -> i32 {
    if hm.get(&k).is_some() {
        return 1;
    }
    0
}

fn main() {
    let mut hm = HashMap::new();
    hm.insert(1, 1);
    hm.insert(2, 4);
    hm.insert(3, 9);

    let result_exists = does_k_exist(hm.clone(), &3); // clone `hm` as it's consumed
    println!("Key 3 exists: {:?}", result_exists);

    let result_not_exists = does_k_exist(hm.clone(), &5);
    println!("Key 5 exists: {:?}", result_not_exists);
}
