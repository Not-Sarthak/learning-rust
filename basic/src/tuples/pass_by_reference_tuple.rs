// consider a situation where a function only needs to read from a tuple and not take ownership, you can pass a reference to it instead.
// this allows the original variable to remain usable after the function call.
// syntax: &(i32, bool, Vec<i32>)
// the code below won’t compile. the reason is that accept_tuple takes ownership of the tuple, making it invalid after the call, but we're trying to use it again in println!.
// pub fn accept_tuple(t: (i32, bool, Vec<i32>)) {

// }

// fn main() {
//     let data = (99, true, vec![1, 2, 3]);
//     accept_tuple(data);
//     println!("{:?}", data);
// }

// accept a reference instead
pub fn accept_tuple(_t: &(i32, bool, Vec<i32>)) {

}

fn main() {
    let data = (99, true, vec![1, 2, 3]);
    // pass a reference instead
    accept_tuple(&data);
    println!("{:?}", data);
}

// creating a new tuple from a reference
pub fn update_tuple_elements(t: &(Vec<i32>, bool, i32)) -> (Vec<i32>, bool, i32) {
    let mut new_t = t.clone();
    new_t.0.push(1);
    new_t.1 = !new_t.1;
    new_t.2 += 1;
    new_t
}

fn main() {
    let t = (vec![10, 20], true, 5);
    
    let updated_t = update_tuple_elements(&t);
    
    println!("Original: {:?}, Updated{:?}",t ,updated_t);
}
