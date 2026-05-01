fn main() {
	let x = 3;
	let y = 5;
	let z = 4;
	
	let result = max_of_three(x, y, z);
	println!("{}", result);
}

pub fn max_of_three(x: i32, y: i32, z: i32) -> i32 {
    if x >= y && x >= z {
        x
    } else if y >= x && y >= z {
        y
    } else {
        z
    }
} 