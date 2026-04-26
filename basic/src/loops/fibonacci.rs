fn main() {
    let n = 10;
    let result = fibonacci(n);
    println!("{}", result);
}

pub fn fibonacci(n: u32) -> u32 {
    if n == 0 || n == 1 {
        return 1;
    } else {
        let x = fibonacci(n - 1) + fibonacci(n - 2);
        return x;
    }
}

// fn main() {
//     let n = 10;
//     let result = fibonacci(n);
//     println!("{}", result);
// }

// pub fn fibonacci(n: u32) -> u32 {
//     if n == 0 || n == 1 {
//         return 1;
//     }

//     let mut a = 1;
//     let mut b = 1;

//     for _ in 2..=n {
//         let temp = a + b;
//         a = b;
//         b = temp;
//     }

//     b
// }