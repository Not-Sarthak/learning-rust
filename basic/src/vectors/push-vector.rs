// push(value) -> used to append a value
fn main() {
  let mut v = vec![4, 5, 6];
  v.push(99);
  println!("{:?}", v);
}


/*
generate a sequence of integers
*/

// fn main() {
//   let result = simple_count(6);
//   println!("{:?}", result);
// }

// pub fn simple_count(n: u32) -> Vec<u32> {
//     let mut v: Vec<u32> = Vec::new();
//     for i in 1..n+1 {
//         v.push(i);
//     }
//     return v;
// } 

/*
generate a countdown sequence
*/
// fn main() {
//   let result = countdown(6);
//   println!("{:?}", result);
// }

// pub fn countdown(n: u32) -> Vec<u32> {
//     let mut vec:Vec<u32> = Vec::new();
//     for i in 1..(n+1) {
//         vec.push(n-i)
//     }
//     return vec;
// }