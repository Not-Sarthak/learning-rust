// swap element at indices i and j
fn main() {
	let v = vec![1,2,3,4];
	let i = 1;
	let j = 2;
	
	let result = swap_ij(v, i, j);
	println!("{:?}", result);
}

pub fn swap_ij(mut v: Vec<i32>, i: usize, j: usize) -> Vec<i32> {
    if i < v.len() && j < v.len() {
        let tmp = v[i];
        v[i] = v[j];
        v[j] = tmp;
    }
    v
}