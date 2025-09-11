/*
Traits: Defining Share Behavior
A trait defines the functionality a particular type has and can share with other types. 
We can use traits to define shared behaviour in an abstract way.
We can use trait bounds to specify that a generic type can be any type that has certain behaviour.

Note: Traits are similar to a feature often called interfaces in other languages, although with some differences
*/

pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("User {} is {} years old", self.name, self.age);
    }
}

fn main() {
    let user = User {
        name: String::from("Sarthak"),
        age: 21,
    };
    println!("{}", user.summarize());
}

// You might say one class implements another abstracted class as in Java. Similarly, the User struct implements the Summary Trait

// We can have just the function signature or the default implementation

// Traits as parameters

/*
pub trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!("{} is {} years old", self.name, self.age)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

fn main() {
    let user = User{
        name: String::from("Sarthak"),
        age: 21,
    };
    notify(&user);
}
*/


// Trait Bound Syntax:
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound.
/*
pub fn notify<T: Summary>(item: T) {
    println!("Breaking News! {}", item.summarize());
}
*/