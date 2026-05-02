// something that can be a None or Some is called an Option. if it is Some then it has some underlying value. 
// options must be printed with the "{:?}" formatter.
// to check if an Option is None, we can use the built-in is_none() method shown on the right.
fn main() {
    let v = vec![1,2,3];
    
    let result = v.get(7); // out of bounds
    
    if result.is_none() {
        println!("{}", "No value");
    }

    if !result.is_none() {
        println!("Value: {:?}", result);
    }
}