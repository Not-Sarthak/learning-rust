// a function that takes a vector of integers and a value, and returns a new vector where each element is incremented by the given value.
fn main() {
	let v = vec![1,2,3];
	let a = 2;
	
	let result = increment_by(&v, a);
	
	println!("{}", a);
	println!("{:?}", &v);
	println!("{:?}", &result);
}

pub fn increment_by(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec![];
    for element in 0..v.len() {
        new_vec.push(v[element] + k);
    }
    new_vec
}