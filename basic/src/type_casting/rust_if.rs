// rusts if statements are a little different from other languages because they can return values. 
// you can think of this as a more powerful version of ternary operator in other languages.
// if an `if` statement is used in this manner, don’t forget to put the semicolon at the end of the if statement, or it won’t compile.

fn main() {
	let result = max(50,22);
	println!("{}", result);
}

pub fn max(x: i32, y: i32) -> i32 {
	let m = if x > y {
		x // no semicolon, because the if statement returns x
	} else {
		y // no semicolon, because the if statement returns y
	}; // semicolon here
	m
} 


// absolute value
fn main() {
	let result = absolute_value(-50);
	println!("{}", result);
}

pub fn absolute_value(x: i32) -> i32 {
	let ans = if x < 0 {
		-x
    } else {
		x
	};
	ans
} 

// absolute value with type conversion to `u32`
fn main() {
	let result = absolute_value(-50);
	println!("{}", result);
}

pub fn absolute_value(x: i32) -> u32 {
	let ans = if x < 0 {
		x * -1
	} else {
		x
	};
	ans as u32
} 