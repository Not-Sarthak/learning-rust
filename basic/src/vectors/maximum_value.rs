fn main() {
	let my_vec = vec![1,2,3];
	let result = max(my_vec);
	
	println!("{}", result);
}

pub fn max(v: Vec<u32>) -> u32 {
  let mut biggest = 0;
  for i in 0..v.len() {
    if v[i] > biggest {
      biggest = v[i];
    }
  }
  
  biggest
}