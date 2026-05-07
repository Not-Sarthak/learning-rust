// since ranges can silently convert to iterators, we can simply apply .collect() to convert it to a vector, but we’ll have to specify the vector type.
pub fn collect_range_to_vector(start: usize, end: usize) -> Vec<usize> {
    let v: Vec<usize> = (start..end).collect();
    v
}

fn main() {
    let v: Vec<usize> = (0..10).collect();
    println!("{:?}", v);

    let result = collect_range_to_vector(3, 7);
    println!("{:?}", result);
}
