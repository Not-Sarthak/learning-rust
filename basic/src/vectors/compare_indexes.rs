/*
if the value in the vector at i is greater than or equal to the value at index j, return true.
*/

fn main() {
	let i = 0;
	let j = 1;
	let v = vec![5,5,15,21];
	
	let result = index_greater(v, i, j);
	println!("{}", result);
}

pub fn index_greater(v: Vec<u32>, i: usize, j: usize) -> bool {
    for _ in 0..v.len() {
        if v[i] >= v[j] {
            return true;
        }
    }
    false
}