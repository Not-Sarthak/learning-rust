// if a non-copy type like a vector contain a copy type (a number i32), it is possible to deference a number in the collection with * but not the entire vector.
fn main() {
    let a = 1;
    let b = 2;
    let c = 3;

    let numbers: Vec<&i32> = vec![&a, &b, &c];

   let values = collect_values(numbers);
    println!("{:?}", values);
}

pub fn collect_values(input: Vec<&i32>) -> Vec<i32> {
    let mut values: Vec<i32> = Vec::new();

    for value in input.into_iter() {
        // dereference each value and push it to values
        values.push(*value);
    }

    values
}

// practise question:
fn main() {
    let a = 10;
    let b = 20;
    let c = 30;

    let refs = vec![&a, &b, &c];

    let result = return_owned_vector(&refs);
    println!("{:?}",result);
}

pub fn return_owned_vector(input: &Vec<&i32>) -> Vec<i32> {
    let v = dereference_values_from_vector(input.clone());  // here
    v
}

pub fn dereference_values_from_vector(v: Vec<&i32>) -> Vec<i32> {
    let mut values = Vec::new();

    for val in v {
        values.push(*val); // here
    }
    values
}
