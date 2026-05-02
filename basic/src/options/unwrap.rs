// to access the value inside the Some we use the .unwrap() method. In other words, .unwrap() gets the value inside the option.
fn main() {
    let v = vec![1,2,3];
    
    let result = v.get(0);
    
    if !result.is_none() {
	    let sum = result.unwrap() + 1; // replace result with result.unwrap()
        println!("{}", sum);
    }
} 