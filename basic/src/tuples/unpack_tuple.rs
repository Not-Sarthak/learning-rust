// instead of using indexing like tuple.0 and tuple.1, you can unpack a tuple by breaking the tuple into separate variables that matches its structure.
// example: (x, y, z): (i32, i32, i32)
// as function parameters: 
// pub fn unpack((x, y, z): (i32, i32, i32)) -> i32 {
//     x+y+z
// }
// as for loop:
// fn main() {
// 	let data:Vec<(i32, i32)> = vec![(5, 6), (6, 8), (8, 9)];
// 	for (a, b) in data.into_iter() {
// 	    println!("First: {}, Second: {}", a, b);
// 	}	
// }
// this is cleaner and easier to read.
pub fn append_total(input: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = vec![];
    let mut sum0 = 0;
    let mut sum1 = 0;

    for (i, j) in input {
        output.push((i, j));
        sum0 += i;
        sum1 += j;
    }

    output.push((sum0, sum1));
    output
}

fn main() {
    let data = vec![(1, 2), (3, 4), (5, 6), (4,8), (6,9)];
    let result = append_total(data);
    println!("{:?}", result); // [(1, 2), (3, 4), (5, 6), (4, 8), (6, 9), (19, 29)]
}