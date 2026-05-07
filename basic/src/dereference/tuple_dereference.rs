// a tuple behaves slightly differently. if a tuple only contains Copy types, you can use * to dereference the whole collection.
// fn take_copy(pair: &(i32, bool)) -> (i32, bool) {
//     *pair
// }

fn main() {
    let p = (2, 5);
    println!("{:?}", swap_and_double(&p));
}

pub fn swap_and_double(pair: &(i32, i32)) -> (i32, i32) {
    let (a, b) = pair;
    (b * 2, a * 2)
}

// but if the items in a tuple are themselves references (&i32, &bool), you can’t dereference the entire collection with * and expect the internal items to be dereferenced.
// fn take_copy(pair: (&i32, &bool)) -> (i32, bool) {
//     *pair // this will not compile
// }
fn main() {
    let a = 100;
    let b = false;
    let p = (&a, &b);
    println!("{:?}", take_copy(p));
}

pub fn take_copy(pair: (&i32, &bool)) -> (i32, bool) {
    (*pair.0, *pair.1)
}

// if a tuple contains a non-Copy type, like a Vec, you can’t dereference the entire tuple. you’ll have to clone it if you want a non-referenced (owned) version of it.
fn get_num(pair: &(Vec<i32>, i32)) -> (Vec<i32>, i32) {
	pair.clone()
}

fn main() {
    let p = (vec![4, 5], 9);
    println!("{:?}", get_num(&p));
}
