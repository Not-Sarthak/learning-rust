// when you have a reference to a collection like a vector, you can’t just use * to get the full value out of it. That only works for simple types like i32, u32, or bool because they are straightforward to copy.
// in rust, we call these Copy types. 
// you can dereference them with * because Rust knows how to copy them quickly behind the scene. 
// but collections like Vec, HashSet, or HashMap are more complex. rust won’t let you copy the actual value out of a reference using * because it could result in copying a lot of data and slow down the program significantly if the data is large.
// when you have a reference to a non-copy type but need to pass the actual value to a function, you can use the .clone() method to create a new copy.

fn main() {
    let x = vec![1, 2, 3];
    let ref_x = &x;

    foo(ref_x.clone());
}

fn foo(v: Vec<i32>) -> usize {
    v.len()
}
