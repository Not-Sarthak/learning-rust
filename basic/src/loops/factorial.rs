fn main() {
  let n = 5;
  let result = factorial(n);
  println!("{}", result);
}

pub fn factorial(n: u32) -> u32 {
  if n == 0 || n == 1 {
    return 1;
  } 
  n * factorial(n - 1)
} 


// fn main() {
//     let n = 5;
//     let result = factorial(n);
//     println!("{}", result);
// }

// pub fn factorial(n: u32) -> u32 {
//     let mut result = 1;

//     for i in 1..=n {
//         result *= i;
//     }

//     result
// }