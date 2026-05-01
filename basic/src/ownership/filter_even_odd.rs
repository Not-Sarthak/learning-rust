fn main() {
	let v = vec![1,2,3];
	let filter_even = true;
	
	let result = filter_even_odd(&v, filter_even);
	println!("{:?}", &v);
	println!("{}", filter_even);
	println!("{:?}", &result);
}

pub fn filter_even_odd(v: &Vec<i32>, b: bool) -> Vec<i32> {
    let mut new_vec: Vec<i32> = vec![];
    if b == false {
        for element in 0..v.len() {
            if v[element] % 2 == 0 {
                new_vec.push(v[element]);
            }
        }
    } else {
        for element in 0..v.len() {
            if v[element] % 2 != 0 {
                new_vec.push(v[element]);
            }
        }        
    }
    return new_vec;
}