// rust does allow integers and boolean values (roughly “fixed-size types”) to be used after being passed to a function.
// vectors can potentially take a lot of memory. therefore, they need to be kept on the "heap" (a large region of memory allocated for the program). if a vector is no longer used, it is optimal to erase it. to track if is "no longer used" Rust needs to track who "owns" the vector.
// a "fixed-size type" takes up very little space, so when another variable or function uses a "fixed-size type" rust silently makes a copy of it and passes the copy.

// integer
fn main() {
	let x = 2;
	let y = inc(x);
	println!("{}", y);
    println!("{}", x);
}

pub fn inc(x: i32) -> i32 {
    x + 1
} 

// boolean
fn main() {
	let x = true;
	let y = x;

	println!("{}", y);
    println!("{}", x);
}