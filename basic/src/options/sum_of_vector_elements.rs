pub fn sum_present(values: Vec<Option<i32>>) -> i32 {
    let mut sum = 0;
    for i in 0..values.len() {
        if values[i].is_some() {
            sum += values[i].unwrap();
        }
    }
    sum
}

fn main() {
    let values1 = vec![Some(2), None, Some(5), Some(3), None];
    let result1 = sum_present(values1);
    println!("Result 1: {:?}", result1); // Output: 10

    let values2 = vec![None, None, None];
    let result2 = sum_present(values2);
    println!("Result 2: {:?}", result2); // Output: 0

    let values3 = vec![Some(10), Some(20), Some(-5)];
    let result3 = sum_present(values3);
    println!("Result 3: {:?}", result3); // Output: 25

    let values4: Vec<Option<i32>> = vec![];
    let result4 = sum_present(values4);
    println!("Result 4: {:?}", result4); // Output: 0
}
