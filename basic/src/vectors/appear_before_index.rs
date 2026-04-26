// a function that determines whether a value k appears in a vector before a given index idx. The function should return true if the value k is found at any position before the specified index, and false otherwise.
fn main() {
	let v = vec![1,2,3,4,5];
	let k = 3;
	let idx = 4;
	
	let result = k_appears_before_idx(v, k, idx);
	println!("{}", result);
}

pub fn k_appears_before_idx(v: Vec<i32>, k: i32, idx: usize) -> bool {
    for element in 0..idx {
        if v[element] == k {
            return true;
        }
    }
    false
} 