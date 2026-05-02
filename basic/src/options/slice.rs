// a function from_index that takes a vector of i32 and a starting index. 
// if the index is within the bounds of the vector, return a new Vec containing all values from that index to the end, wrapped in Some. 
// if the index is out of bounds, return None.

pub fn from_index(v: Vec<i32>, start: usize) -> Option<Vec<i32>> {
    if start > v.len() {
        None
    }
    let mut new_vec: Vec<i32> = vec![];
    for i in start..v.len() {
        new_vec.push(v[i]);
    }
    Some(result)
}

fn main() {
    let vector = vec![-2, 1,2,3,4,5,6,7,8];
    let result = from_index(vector, 7);

    println!("{:?}", result);

    let vector = vec![7,8];
    let result = from_index(vector, 2);

    println!("{:?}", result);
}