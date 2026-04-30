// copy manually

fn main() {
	let v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result);
}

pub fn modify(v: Vec<i32>) -> Vec<i32> {
    let mut my_vec = make_copy(v);
    
    my_vec.push(6);
    my_vec
}

pub fn make_copy(v: Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec![];
    for i in 0..v.len() {
        new_vec.push(v[i]);
    }
    new_vec
} 

// cloning using `clone()`
fn main() {
	let v = vec![3,4,5];
	let result = modify(v);
	println!("{:?}", result);
}

pub fn modify(v: Vec<i32>) -> Vec<i32> {
    let mut my_vec = v.clone();
    
    my_vec.push(6);
    my_vec
} 

// add one to each element in vector
fn main() {
	let v = vec![4,2,6];
	let result = add_1_to_each(v);
	println!("{:?}", result);
}

pub fn add_1_to_each(v: Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec![];
    for i in 0..v.len() {
        new_vec.push(v[i]+1);
    }
    new_vec
}

// append sum of elements
fn main() {
	let v = vec![4,2,6];
	let result = append_sum(v);
	println!("{:?}", result);
}

pub fn append_sum(v: Vec<i32>) -> Vec<i32> {
    let mut new_vec: Vec<i32> = v.clone();
    let mut sum = 0;

    for i in 0..new_vec.len() {
        sum += new_vec[i];
    }
    new_vec.push(sum);
    new_vec
} 

// remove odd numbers from vector
fn main() {
	let v = vec![1,2,3,4];
	let result = remove_odd(v);
	println!("{:?}", result);
}

pub fn remove_odd(v: Vec<i32>) -> Vec<i32> {
    let mut even_vec: Vec<i32> = vec![];
    for i in 0..v.len() {
        if v[i] % 2 == 0 {
            even_vec.push(v[i]);
        }
    }
    even_vec
}

// remove elements less than k
fn main() {
	let v = vec![1,2,3,4];
	let k = 3;
	let result = remove_less_than_k(v, k);
	println!("{:?}", result);
}

pub fn remove_less_than_k(v: Vec<i32>, k: i32) -> Vec<i32> {
    let mut vec: Vec<i32> = vec![];

    for i in 0..v.len() {
        if v[i] >= k {
            vec.push(v[i]);
        }
    }
    vec
}

// .zip — pairs elements from two iterators into tuples, stopping at the shorter one.
// .map — transforms each element of an iterator by applying a function to it.

// reverse a vector
fn main() {
	let v = vec![1,2,3,4];
	
	let result = reverse(v);
	println!("{:?}", result);
}

pub fn reverse(v: Vec<i32>) -> Vec<i32> {
    let mut reverse_vec: Vec<i32> = vec![];

    for i in 0..v.len() {
        reverse_vec.push(v[v.len()-1-i])
    }

    reverse_vec
}