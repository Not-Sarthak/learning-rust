// to prevent a function from consuming a vector, we can pass a reference.
// a reference to a variable is that variable prepended with &.
// for example, we can pass a reference as follows:
/*
    let v = vec![1,2,3];
    foo(&v);
*/
// since `foo` is given a reference to `v`, rather than the actual variable `v`, `foo` does not consume `v`
// to make this work, `foo` must take a "reference to a vector" as the type, not a vector.

// for example, if `foo` had the signature `foo(v: Vec<i32>` we would need to change it to become `foo(v: &Vec<i32>`.

fn main() {
	let v = vec![1,2,3];
	let sum = vec_sum(&v);
	println!("{:?}", v);
	println!("{}", sum);
}

pub fn vec_sum(v: &Vec<i32>) -> i32 {
	let mut sum = 0;
	
	for i in 0..v.len() {
		sum = sum + v[i];
	}
	
	sum
} 

// sum of two vectors using references
fn main() {
	let v1 = vec![1,2,3];
	let v2 = vec![1,1,2];
	
	let result = elementwise_sum(&v1, &v2);
	
	// can print because v1 and v2 are not consumed
	println!("{:?}", v1);
	println!("{:?}", v2);
	println!("{:?}", result);
}

pub fn elementwise_sum(v1: &Vec<i32>, v2: &Vec<i32>) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];
    for element in 0..v1.len(){
        vec.push(v1[element] + v2[element]);
    }
    vec
} 

// reusing vectors:
// implement all_elements_less_than_k. It takes a reference to a vector and returns true if all elements are < k and false otherwise.
// implement sum_greater_than_s. It takes a reference to a vector and returns true if the elements sum to a value greater than s.
fn main() {
	let v = vec![1,1,5];
	let cond1 = all_elements_less_than_k(&v, 6);
	let cond2 = sum_greater_than_s(&v, 6);
	println!("{}", cond1 && cond2);
}

pub fn all_elements_less_than_k(v: &Vec<i32>, k: i32) -> bool {
    for i in 0..v.len() {
        if v[i] >= k {
            return false;
        }
    }
    true
}

pub fn sum_greater_than_s(v: &Vec<i32>, s: i32) -> bool {
    let mut sum: i32 = 0;
    for i in 0..v.len() {
        sum += v[i];
        if s < sum {
            return true;
        }
    }
    false
}