// u64 to u32

fn main() {
	let x_ok = 12345678u64;
	let result_ok = from_u64_to_u32(x_ok);
	println!("{} -> {:?}", x_ok, result_ok);

	let x_big = u32::MAX as u64 + 1;
	let result_big = from_u64_to_u32(x_big);
	println!("{} -> {:?}", x_big, result_big);
}

pub fn from_u64_to_u32(x: u64) -> Option<u32> {
    if x <= u32::MAX as u64 {
        Some(x as u32)
    } else {
        None
    }
}

// i32 to i8
fn main() {
    println!("100 -> {:?}", safe_i32_to_i8(100));
    println!("127 -> {:?}", safe_i32_to_i8(127));
    println!("128 -> {:?}", safe_i32_to_i8(128));
    println!("-128 -> {:?}", safe_i32_to_i8(-128));
    println!("-129 -> {:?}", safe_i32_to_i8(-129));
}

pub fn safe_i32_to_i8(x: i32) -> Option<i8> {
    if x <= i8::MAX as i32 && x >= i8::MIN as i32 {
        Some(x as i8)
    } else {
        None
    }
}