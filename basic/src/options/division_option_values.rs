fn main() {
    let a = Some(10);
    let b = Some(2);
    let b_zero = Some(0);
    let c: Option<i32> = None;

    println!("{:?} / {:?} = {:?}", a, b, div_options(a, b));
    println!("{:?} / {:?} = {:?}", a, b_zero, div_options(a, b_zero));
    println!("{:?} / {:?} = {:?}", a, c, div_options(a, c));
    println!("{:?} / {:?} = {:?}", c, b, div_options(c, b));
    println!("{:?} / {:?} = {:?}", c, c, div_options(c, c));

}

pub fn div_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    // check for None
    if a.is_none() || b.is_none() {
        return None;
    }

    // unwrap to get the i32 values
    let val_a = a.unwrap();
    let val_b = b.unwrap();

    // check for division by zero
    if val_b == 0 {
        return None;
    }

    // perform division
    Some(val_a / val_b)
}