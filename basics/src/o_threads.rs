// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables, dead_code, unused_imports)]

use crossbeam::channel;
use std::thread;
use std::time::Duration;

pub fn main() {
    println!("\nthreads...");

    simple_delegation_to_thread();
    exercise();
}

fn simple_delegation_to_thread() {
    // Spawn child
    let handle = thread::spawn(move || {
        // do stuff
    });

    // wait for child to exit
    handle.join().unwrap();
}

fn exercise() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];
    let handle = thread::spawn(move || expensive_sum(my_vector));

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }

    // wait for child to exit
    let sum = handle.join().unwrap();
    println!("The child thread's expensive sum is {}", sum);

    // Though there is a primitive type of channel in the std::sync::mpsc module,
    // It is recommended to always using channels from the crossbeam crate.
    // Unbounded means the buffer is not bound to a certain size/memory
    let (tx, rx) = channel::unbounded();

    // Cloning the transmit end of the channel, for use in a separate thread.
    let tx2 = tx.clone();
    let handle_a = thread::spawn(move || {
        pause_ms(300);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(400);
        tx2.send("Thread A: 2").unwrap();
    });

    // Make sure Thread A has time to get going before we spawn Thread B
    pause_ms(100);
    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    // Using a Receiver end of a channel as an iterator to get values until the channel gets closed.
    // Receiver end gets closed automatically when transmitting ends are all closed.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    // Join the child threads for hygiene.
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    let (sender, receiver) = channel::unbounded();
    let receiver2 = receiver.clone();

    let thread = thread::spawn(move || {
        for message in receiver.iter() {
            println!("thread 1 received {}", message)
        }
    });

    let thread2 = thread::spawn(move || {
        for message in receiver2.iter() {
            println!("thread 2 received {}", message)
        }
    });

    // Send some messages.
    for attempt in 1..8 {
        sender
            .send(format!("Ground Control to Major Tom ({})...", attempt))
            .unwrap();

        pause_ms(100);
    }

    // Close the channel
    drop(sender);

    // Cleanup
    thread.join().unwrap();
    thread2.join().unwrap();

    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`).  Join
    // the child threads.
    println!("Main thread: Exiting.")
}

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    // 1a. Between the .iter() and the .sum() add a .filter() with a closure to keep any even
    // number (`x % 2` will be 0 for even numbers).
    // 1b. Between the .filter() and the .sum() add a .map() with a closure to square the values
    // (multiply them by themselves)
    //
    // In the closures for both the .filter() and .map() the argument will be a reference, so you'll
    // either need to dereference the argument once in the parameter list like this: `|&x|` or you
    // will need to dereference it each time you use it in the expression like this: `*x`
    v.iter()
        // .filter() goes here
        // .map() goes here
        .sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
