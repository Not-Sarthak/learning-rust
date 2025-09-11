// It lets thread pass data from one thread to another

/*
One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data. 
Here's the idea in a slogan from the Go Language Documentation" "Do not communicate by sharing memory, instead, share memory by communicating."

To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels. 
A channel is a general programming concept by which data is sent from one thread to another.

A channel has two halves: a transmitter and a receiver.
The transmitter half is the upstream location where you can put rubber ducks into the river,
and the receiver half is where the rubber duck ends up downstream.
One part of your code calls methods on the transmitter with the data you want to send, and another part checks the receiving end for arriving messages.
A channel is said to be closed if either the transmitter or receiver half is dropped.
*/

use::std::thread;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Sarthak");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

// Can you write a code that finds the sum from 1 - 10^8?
// Use threads to make sure you use all cores of your machine
// Remember the name says `multiple producer single consumer`

/*
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
*/