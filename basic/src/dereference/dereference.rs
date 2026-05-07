// if we pass a reference to a function that expects the actual value, the code will not compile.
// to turn a reference into an actual value, we need to dereference. This is done with the * operator.

// & - Address of Operator
// * - Dereference Operator

// eg:
// let x = 2;
// let y = &x; // reference to x
// let z = *y; // dereference y

fn main() {
    let vector = vec![1, 2, 3];
    let result = sum_doubles(&vector);
    println!("{}", result);
}

pub fn sum_doubles(v: &Vec<i32>) -> i32 {
    let mut s = 0;
    for e in v {
        s += double(*e); // dereference e
    }
    s
}

pub fn double(e: i32) -> i32 {
    e * 2
}

// auto dereference
fn main() {
    let x = 10;
    let y = 5;
    let x_ref = &x;
    let y_ref = &y;
    let sum = x_ref + y_ref; // rust auto-dereferences
    println!("Sum: {}", sum);
}

// OR
fn main() {
    let v = vec![1, 2, 3];
    let result = sum(&v);
    println!("{}", result);
}

pub fn sum(v: &Vec<i32>) -> i32 {
    let mut s = 0;
    for e in v {
        s += e; // rust auto-dereferences here. change to *e. both work.
    }
    s
}

// index dereference => does not dereference automatically
fn main() {
    let numbers = vec![10, 20, 30];
    let index_ref = &1;
    println!("{}", numbers[*index_ref]);
}

// when rust dereference?
// rust sometimes dereference automatically and sometimes not
// will study the reason later in depth
pub fn increment<T>(x: T) -> i32
where
    T: std::borrow::Borrow<i32>,
{
    *x.borrow() + 1
}

fn main() {
    let x = 1;
    println!("{}", x); // 2
    println!("{}", &x); // 2
}


// `.get()` - only accepts values
fn main() {
    let v = vec![1, 2, 3];
    let index = &0;
    let result = v.get(*index);
    println!("{:?}", result);
}

// on the other side, `.contains()` only accepts reference
use std::collections::HashSet;

fn main() {
    let s = HashSet::from([1, 2, 3]);
    let e = 2;

    let result = s.contains(&e);

    println!("{}", result);
}

// the .insert method works for both references and owned values, but the types must be consistent. in other words, the values of the hashmap must all be references or all be owned values.
use std::collections::HashMap;

fn main() {
    let mut hm = HashMap::new();

    let k1 = 1;
    let k2 = 2;
    let v1 = 10;
    let v2 = &20;

    hm.insert(k1, v1);
    hm.insert(k2, *v2);
    // OR
    // hm.insert(k1, &v1); 
    // hm.insert(k2, v2);
    println!("{:?}", hm);
}

// in general, a function will only accept either a value or a reference, but not both.
fn main() {
    let v = vec![1, 2, 3];
    let except = &1;

    let result = sum_except(&v, *except);

    println!("{}", result);
}

pub fn sum_except(v: &Vec<i32>, except: usize) -> i32 {
    let mut sum = 0;
    for i in 0..v.len() {
        if i != except {
            sum += v[i];
        }
    }
    sum
}
