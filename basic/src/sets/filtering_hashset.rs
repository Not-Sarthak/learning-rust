// filter out even numbers from the given set
use std::collections::HashSet;

fn main() {
    let s: HashSet<i32> = HashSet::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let odds_s = remove_evens(s);
    println!("Original set had evens and odds. Odds only set: {:?}", odds_s);
}


pub fn remove_evens(s: HashSet<i32>) -> HashSet<i32> {
  let mut result = HashSet::new();

  for item in s.into_iter() {
    if item % 2 != 0 {
      result.insert(item);
    }
  }

  result
}

// increment all elements of set
use std::collections::HashSet;

fn main() {
    let s: HashSet<i32> = HashSet::from([1, 2, 3, 0, -5]);
    
    let result = inc_set(s);
    println!("{:?}", result);
}

pub fn inc_set(s: HashSet<i32>) -> HashSet<i32> {
    let mut new_set = HashSet::new();
    for item in s.into_iter() {
        new_set.insert(item+1);
    }
    new_set
} 