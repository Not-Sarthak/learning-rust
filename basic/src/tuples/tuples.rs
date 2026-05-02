// tuples are fixed-size lists that may have heterogeneous types (i.e. the items don’t all have the same type).
// we define a tuple with parentheses and comma-separated types inside them.
fn main() {
    let x: (i32, bool) = (2, true);
    let y: (u32, i32, bool) = (2, -5, false);
    let z: (Vec<i32>, i32, bool) = (vec![-5, 2, 3], 8, true);
    let p: (i32, i32) = (2,3);
    let unit: () = ();

    println!("{:?}, {:?}, {:?}, {:?}, {:?}", x, y, z, p, unit);
}

// output:
// (2, true), (2, -5, false), ([-5, 2, 3], 8, true), (2, 3), ()

// tuples with types that don’t copy automatically. tuples that contain vectors have all the issues with ownership that we learned previously. 
fn main() {
    let x = (3, vec![1, 2, 3]);
    let y = x;
    println!("{:?}", y);
}

// when a tuple contains only fixed-sized types like bool and i32 Rust can copy them automatically into a function or to other variables so you can re-use it without giving up ownership.
fn main() {
    let x = (3, 1, true);
    let y = x;
    println!("{:?}", x);
}