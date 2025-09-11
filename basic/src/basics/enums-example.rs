// Enums let you enumerate over various types of a value

enum Shape {
    Rectangle(f64, f64),
    Circle(f64),   
}

fn main() {
    let rect = Shape::Rectangle(1.0,2.0);
    calculate_area(rect);
}

fn calculate_area(shape: Shape) -> f64 {
    let area = match shape {
        Shape::Rectangle(a, b) => a*b,
        Shape::Circle(r) => 3.14 * r * r,
    };
    return area;
}