fn main() {
    let x = 48;
    let y = 18;
    let result = gcd(x, y);
    println!("{}", result);
}

pub fn gcd(x: u32, y: u32) -> u32 {
    // 1. determine the minimum of the two numbers
    let mut min = x;
    if y < x {
        min = y;
    }

    // 2. loop backwards from the minimum down to 1
    for i in 0..min {
        let current = min - i;
        
        // 3. return the first number that divides both
        if x % current == 0 && y % current == 0 {
            return current;
        }
    }
    
    1
}