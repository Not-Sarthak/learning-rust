// vector deduplication using hashset
use std::collections::HashSet;

fn main() {
  let v = vec![1, 2, 2, 3, 4, 4, 4, 5, 1];
  let deduped_v = dedup(v);
  println!("{:?}", deduped_v);
}

pub fn dedup(v: Vec<i32>) -> Vec<i32> {
  let dedup_hash: HashSet<i32> = v.into_iter().collect();
    let dedup_vec: Vec<i32> = dedup_hash.into_iter().collect();
    dedup_vec
}
