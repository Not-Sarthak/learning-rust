// a unique feature of rust is notion of "consumption" or "ownership"
// when a function takes a vector as an argument, it "consumes" the vector, meaning no code after the function can use the vector
// saying a function "consumed" a vector means exactly the same thing as a function "took ownership" of the vector

// the reason ownership exists is so that Rust can run without a garbage collector.
// it needs to know how many variables are "pointing" to a vector so it can erase the vector automatically when all the references go out of scope.

fn main() {
	let v = vec![1,2,3];
	let w = v;
	println!("{:?}", w); // ok
	// println!("{:?}", v); // w consumed v, so v cannot be used here
}