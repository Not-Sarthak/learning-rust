mod stack;
mod queue;

use stack::{Stack, par_checker, divide_by_two};

fn main() {
    println!("=== Stack Examples ===\n");

    basic_stack();
    peek_example();
    iter_example();
    paranthesis_example();
    decimal_to_binary_example();
}

fn basic_stack() {
    println!("--- Basic Stack Operations ---");
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    println!("size: {}, {:?}", s.len(), s);
    println!("pop: {:?}, size {}", s.pop().unwrap(), s.len());
    println!("empty: {}, {:?}", s.is_empty(), s);

    s.clear();
    println!("after clear: {:?}\n", s);
}

fn peek_example() {
    println!("--- Peek Operations ---");
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    println!("{:?}", s);
    let peek_mut = s.peek_mut();
    if let Some(top) = peek_mut {
        *top = 4;
    }

    println!("top {:?}", s.peek().unwrap());
    println!("{:?}\n", s);
}

fn iter_example() {
    println!("--- Iterator Operations ---");
    let mut s = Stack::new();
    s.push(1);
    s.push(2);
    s.push(3);

    let sum1 = s.iter().sum::<i32>();
    let mut addend = 0;

    for item in s.iter_mut() {
        *item += 1;
        addend += 1;
    }

    let sum2 = s.iter().sum::<i32>();
    println!("{sum1} + {addend} = {sum2}");
    assert_eq!(9, s.into_iter().sum::<i32>());
    println!();
}

fn paranthesis_example() {
    println!("--- Matching Paranthesis ---");
    let a = "() {} []";
    let b = "() {) [}";

    let response1 = par_checker(a);
    let response2 = par_checker(b);
    println!("'{}' balanced: {}", a, response1);
    println!("'{}' balanced: {}\n", b, response2);
}

fn decimal_to_binary_example() {
    println!("--- Decimal to Binary ---");
    let num = 10;
    let bin_str = divide_by_two(num);
    println!("{num} = b{bin_str}");
}
