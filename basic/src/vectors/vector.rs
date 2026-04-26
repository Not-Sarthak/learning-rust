/*
a "vector" is what other langauges call a "list."
note that the print statement uses "{:?}" instead of "{}" like it did for integers.
to get the length of a vector, we use the .len() property. Since length is not a "complex" type, we can print it with "{}".
*/

fn main() {
	let my_vec = vec![1,2,3,4];
	println!("{:?}", my_vec);

	let length = my_vec.len();
	println!("{}", length);

    for i in 0..my_vec.len() {
		let element = my_vec[i];
		println!("{}", element);
	}
}