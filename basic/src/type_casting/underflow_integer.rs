// if we carelessly cast from an i32 to a u32, we will get an underflow

fn main() {
	let result = foo(-1); // result = 4294967295
	println!("{}", result);
}

pub fn foo(x: i32) -> u32 {
    x as u32
} 