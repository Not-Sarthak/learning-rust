fn main() {
	let v_ok = vec![10, -20, 127, -128, 0];
	let v_bad_high = vec![10, 128, 50];
	let v_bad_low = vec![-129, 0, 100];

	println!("{:?} -> {}", v_ok, vector_can_be_downcasted(&v_ok));
	println!("{:?} -> {}", v_bad_high, vector_can_be_downcasted(&v_bad_high));
	println!("{:?} -> {}", v_bad_low, vector_can_be_downcasted(&v_bad_low));
	
	let empty_vec: Vec<i32> = vec![];
	println!("{:?} -> {}", empty_vec, vector_can_be_downcasted(&empty_vec));
}

pub fn vector_can_be_downcasted(v: &Vec<i32>) -> bool {
	for element in 0..v.len() {
        if v[element] > i8::MAX as i32 || v[element] < i8::MIN as i32{
            return false;
        }
    }
    true
} 