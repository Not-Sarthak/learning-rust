// remove maximum value
// write a function that takes a vector of unsigned integers and returns a new vector
// with the maximum value removed. if there are multiple occurrences of the maximum
// value, remove only the first occurrence. if the input vector has a single element
// or is empty, return an empty vector.

fn main() {
	let v = vec![1,2,3];
	
	let result = remove_max(&v);
	
	println!("{:?}", &v);
	println!("{:?}", &result);
}

pub fn remove_max(v: &Vec<u32>) -> Vec<u32> {
    if v.len() <= 1 {
        return vec![];
    }

    let mut max_idx = 0;
    for i in 1..v.len() {
        if v[i] > v[max_idx] {
            max_idx = i;
        }
    }

    let mut result = Vec::with_capacity(v.len() - 1);
    for i in 0..v.len() {
        if i != max_idx {
            result.push(v[i]);
        }
    }

    result
}