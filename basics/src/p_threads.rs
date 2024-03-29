// Silence some warnings so they don't distract from the exercise.
#![allow(unused_variables, dead_code, unused_imports)]

use crossbeam::channel;
use std::thread;
use std::time::Duration;

pub fn main() {
    println!("\nthreads...");

    simple_delegation_to_thread();
    using_threads_with_channels();
}

fn simple_delegation_to_thread() {
    // Spawn child
    let handle = thread::spawn(move || {
        // do stuff
    });

    // wait for child to exit
    handle.join().unwrap();
}

fn using_threads_with_channels() {
    // Send stuff
    let (command_tx, command_rx) = channel::bounded(3);
    let command_rx2 = command_rx.clone();

    // Receive stuff
    let (result_tx, result_rx) = channel::bounded(3);
    let result_tx2 = result_tx.clone();

    // Workers
    let worker1 = thread::spawn(move || {
        for value in command_rx {
            result_tx.send(format!("Thread A: {}", value * 2)).unwrap();
        }
        drop(result_tx);
    });
    let worker2 = thread::spawn(move || {
        for value in command_rx2 {
            result_tx2.send(format!("Thread B: {}", value * 2)).unwrap();
        }
        drop(result_tx2);
    });

    // Get ready to receive stuff
    let value_receiver = thread::spawn(move || {
        for received_value in result_rx {
            println!("Receiver: {}", received_value);
        }
    });

    // Send stuff
    let value_sender = thread::spawn(move || {
        let multiple_values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        for value in multiple_values {
            command_tx.send(value).unwrap();
        }
        drop(command_tx);
    });

    worker1.join().unwrap();
    worker2.join().unwrap();
    value_sender.join().unwrap();
    value_receiver.join().unwrap();

    println!("Main thread function: Exiting.")
}
