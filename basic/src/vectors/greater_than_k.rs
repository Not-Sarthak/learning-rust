/*
a function that returns true if all elements are > k. 
you can do this by looping through the array and returning false if any element is ≤ k. 
if the loop completes without returning, then return true.
*/

fn main() {
	let my_vec = vec![1,2,3];
	let k = 0;
	let result = greater_than_k(my_vec, k);
	
	println!("{}", result);
}

pub fn greater_than_k(v: Vec<u32>, k: u32) -> bool {
    for num in v {
        if num <= k {
            return false;
        }
    }
    true
}