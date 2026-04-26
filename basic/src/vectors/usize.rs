fn main() {
    let k = 8;
    let v = vec![1, 2, 8, 9, 3];    
    let result = find_k(v, k);
    println!("{}", result);
}

pub fn find_k(v: Vec<i32>, k: i32) -> usize {
    for i:u32 in 0..v.len() {
        if v[i] == k {
            return i
        }
    }
    0
}

// fn main() {
//     let index_at = 2;
//     let v = vec![2, 4, 6, 8];    
//     let result = get_index_at(v, index_at);
//     println!("{}", result);
// }

// pub fn get_index_at(v: Vec<i32>, i: usize) -> i32 {
//     v[i]
// }