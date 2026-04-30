fn main() {
	let v = vec![4,8,14];
	let k = 8;
	let result = filter_lt_k(&v, k);
	println!("{:?}", result);
}

pub fn filter_lt_k(v: &Vec<i32>, k: i32) -> Vec<i32> {
    let result: Vec<i32> = vec![];

    for i in 0..v.len() {
        if v[i] >= k {
            result.push(v[i])
        }
    }
    result
}