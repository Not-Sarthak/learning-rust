fn main() {
	let base = 2;
	let exponent = 4;
	let result = power(base, exponent);
	
	println!("{}", result);
}

pub fn power(base: u32, exponent: u32) -> u32 {
	let mut acc = 1;
	for _i in 0..exponent {
        acc = acc * base;
    }
	acc
} 