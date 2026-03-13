use::std::thread;
use std::{primitive, sync::mpsc};

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 0..4 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in 0..1000000 {
                ans = ans + (i * 1000000 + j);
            }
            producer.send(ans).unwrap();
        });
    }

    drop(tx);

    let mut ans: u64 = 0;
    for val in rx {
        ans = ans + val;
        println!("Found Value!");
    }

    println!("Answer is {}", ans);
}