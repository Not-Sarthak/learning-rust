fn main() {
    let v = vec![1,2,3];
    let result_ok = double_at(&v, 1);
    println!("Double index 1: {:?}", result_ok);

    let result_none = double_at(&v, 5);
     println!("Double index 5: {:?}", result_none);

     let result_empty = double_at(&Vec::<i32>::new(), 0);
     println!("Double index 0 (empty): {:?}", result_empty);
}

pub fn double_at(v: &Vec<i32>, idx: usize) -> Option<Vec<i32>> {
    let mut vec = v.clone();
    if idx >= vec.len() {
        None
    } else {
        vec[idx] *= 2;
        Some(vec)
    }
}