struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn main(){
    let user = User {
        active: true,
        username: String::from("0xsarthak"),
        email: String::from("notsarthakshah@gmail.com"),
        sign_in_count: 42
    };

    println!("{}", user.active);
    println!("{}", user.username);
    println!("{}", user.email);
    println!("{}", user.sign_in_count);
}