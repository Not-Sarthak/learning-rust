pub fn append_total(input: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = vec![];
    let mut sum0 = 0;
    let mut sum1 = 0;
		
    for i in 0..input.len() {
        output.push(input[i]);
        sum0 += input[i].0;
        sum1 += input[i].1;
    }
    output.push((sum0, sum1));
    output
}

fn main() {
    let data = vec![(1, 2), (3, 4), (5, 6), (4,8), (6,9)];
    let result = append_total(data);
    println!("{:?}", result); // [(1, 2), (3, 4), (5, 6), (4, 8), (6, 9), (19, 29)]
}