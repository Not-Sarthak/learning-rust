fn main() {
    let x = 10;
    let y = 2;
    
    let result = div(x,y);
    
    if !result.is_none() {
        println!("{}", result.unwrap());
    } else {
        println!("{}", "divide by zero");
    }

    let y_zero = 0;
    let result_zero = div(x, y_zero);
    if !result_zero.is_none() {
        println!("{}", result_zero.unwrap());
    } else {
        println!("{}", "divide by zero");
    }
}

pub fn div(n: i32, d: i32) -> Option<i32> {
    if d == 0 {
        None
    } else {
        Some(n / d)
    }
} 