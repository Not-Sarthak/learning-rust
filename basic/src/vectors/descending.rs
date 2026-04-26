fn main() {
	let v = vec![1,2,1];
	
	let result = is_sorted(v);
	println!("{}", result);
}

pub fn is_sorted(v: Vec<i32>) -> bool {
    for i in 0..v.len()-1 {
        if v[i] > v[i+1] {
            return false;
        }
    }
    true
}