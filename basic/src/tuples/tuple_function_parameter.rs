// when you pass a tuple to a function, you define the parameter with the tuple's exact shape and types.
pub fn product_of_coordinates(coord: (i32, i32, i32)) -> i32 {
    coord.0 * coord.1 * coord.2
}

fn main() {
    let coord = (12, 6, 5);
    println!("Product of Cordinates: {}", product_of_coordinates(coord));
}

// OR

pub fn sum_points(points: (i32, i32)) -> i32 {
    points.0 + points.1
}

fn main() {
    let x = (42, 5);
    println!("{}", sum_points(x));
}