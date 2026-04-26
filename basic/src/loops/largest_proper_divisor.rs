fn main() {
  let x = 12;
  let result = largest_proper_divisor(x);
  println!("{}", result);
}

pub fn largest_proper_divisor(x: u32) -> u32 {
  for i in 1..x {
    let backward_index = x - i;
    if x % backward_index == 0 {
        return backward_index;
    }
  }
  return 1;
}