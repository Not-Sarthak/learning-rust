/* 
how to approach [pattern questions]:
1. number of times outer loop will run = number of rows
2. number of times inner loop will run = number of columns. identify for every row number, how many columns are there OR types of elements in the column
3. what do you need to print
*/

// try to find formula between rows and columns

/*
*****
*****
*****
*****
*****
*/
fn main() {
    for i in 0..5 {
        for j in 0..5{
            print!("*");
        }
        println!();
    }
}

/*
*****
****
***
**
*
*/
fn main() {
    for i in 0..5 {
        for j in 0..5-i{
            print!("*");
        }
        println!();
    }
}

/*
1
12
123
1234
12345
*/
fn main() {
    for i in 0..=5 {
        for j in 1..i+1{
            print!("{}", j);
        }
        println!();
    }
}

/*
*
***
*****
*******
*********
*******
*****
***
*
*/
fn main() {
    let n = 5;

    for i in 1..=n {
        for _ in 0..2 * i - 1 {
            print!("*");
        }
        println!();
    }

    for i in (1..n).rev() {
        for _ in 0..2 * i - 1 {
            print!("*");
        }
        println!();
    }
}
