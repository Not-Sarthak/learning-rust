// You can return the value from a function, back to function 

fn create_string() {
    let s1: String = String::from("Hello");
    println!("{}", s1);
    // let s2: String = s1;     //Moving
    let s2: String = s1.clone();  // People use this to fix errors, but it is a bad practise
    println!("{}", s1);
}

fn main(){
    create_string();
}

// fn main() {
//     let mut s1 = String::from("Sarthak");
//     let s2 = s1;
//     s1 = s2;

//     println!("{}", s1);
// }