// find first common element index
//
// you are given two vectors `a` and `b`. find the first index `i` such that
// a[i] == b[i].
//
// rules:
// - if `a` is empty, return 0.
// - if `b` is empty, return the length of `a`.
// - if no such index exists, return the length of `a`.
// - if vectors have different lengths, only iterate up to the shorter length.
//
// examples:
// a = [1, 2, 3, 4], b = [2, 2, 3, 4] → output: 1
// a = [1, 2, 3, 4], b = [5, 6, 7, 8] → output: 4
// a = [], b = [1, 2, 3] → output: 0
// a = [1, 2, 3], b = [] → output: 3

fn main() {
	let v1 = vec![1,2,3,4];
	let v2 = vec![2,2,3,4];
	
	let result = first_common_index(&v1, &v2);
	println!("{}", result);
	println!("{:?}", &v1);
	println!("{:?}", &v2);
}

pub fn first_common_index(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
    if a.is_empty() {
        return 0;
    }
    if b.is_empty() {
        return a.len() as i32;
    }

    let len = std::cmp::min(a.len(), b.len());

    for i in 0..len {
        if a[i] == b[i] {
            return i as i32;
        }
    }

    a.len() as i32
}