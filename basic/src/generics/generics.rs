// Generics are a concept that exist in many languages

/*
Without Generic
*/
fn main() {
    let bigger = largest_i32(1, 2);
    let bigger_char = largest_char('a', 'b');
    println!("{}", bigger);
    println!("{}", bigger_char);
}

fn largest_i32(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn largest_char(a: char, b: char) -> char {
    if a > b {
        a
    } else {
        b
    }
}

/*
With Generic
*/

fn main() {
    let bigger = largest(1, 2);
    let bigger_char = largest('a', 'b');
    println!("{}", bigger);
    println!("{}", bigger_char);
}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}


/*
TypeScript:

function larger<T>(a: T, b: T): T {
    if (a > b) {
        return a;
    } else {
        return b; 
    }
}

larger(1, 2);

---------------------

function larger<T, U>(a: T, b: U): T | U {
    if (a > b) {
        return a;
    } else {
        return b; 
    }
}

larger(1, "sarthak");
*/