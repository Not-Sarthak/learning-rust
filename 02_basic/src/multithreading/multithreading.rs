/*
In most current operating systems, an executed program's code is run in a process, and the operating system will manage multiple processes at once.
Within a program, you can also have independent parts that run simulataneously. The features that run these independent parts are called threads. 
For example, a web server could have multiple threads so that it could respond to more than one request at the same time.
*/

/*
Example 1:

use::std::thread;
use::std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi Number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi Number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
*/

// ------------- //

use::std::thread;
use::std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi Number {i} from spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();   // Wait for thread to complete

    for i in 1..5 {
        println!("Hi Number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}

// ------------- //

// Using move Closures with Threads
/*
We'll often use the move keyword with closures passed to thread::spawn because the closure will then take ownership of the values it uses from the environment thus transferring ownership of those values from one thread to another.
*/

/*
Error:

use::std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(|| {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
*/

/*
Solution

use::std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });

    handle.join().unwrap();
}
*/