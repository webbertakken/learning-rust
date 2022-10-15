// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_imports, unused_variables)]
use crossbeam::channel;
use dotenvy::dotenv;
use log::{debug, error, info, trace, warn};
use std::thread;
use std::time::Duration;

fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn expensive_sum(v: Vec<i32>) -> i32 {
    // Pretend our fancy little filter-map-sum is expensive and takes 500ms
    sleep_ms(1);
    println!("Child thread: just about finished");
    v.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum()
}

fn main() {
    dotenv().expect("Failed to load .env file");
    env_logger::init();

    let my_vector = vec![2, 5, 1, 0, 4, 3];

    // 1. Spawn a child thread and have it call `expensive_sum(my_vector)`. Store the returned
    // join handle in a variable called `handle`. Once you've done this you should be able to run
    // the code and see the output from the child thread's expensive sum in the middle of the main
    // thread's processing of letters.
    //
    let handle = thread::spawn(move || expensive_sum(my_vector));

    // While the child thread is running, the main thread will also do some work
    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Processing the letter '{}'", letter);
        sleep_ms(2);
    }

    // 2. Let's retrieve the value returned by the child thread once it has exited.
    // - Uncomment and complete the code below.
    // - Call the .join() method on `handle` from #1 and assign the `Result<i32, Err>` it returns
    // to a variable named `result`
    // - Get the i32 out of `result` and store it in a `sum` variable.

    let result = handle.join();
    let sum = result.unwrap_or(0);
    println!("The child thread's expensive sum is {}", sum);

    // 3. Time for some fun with channels!
    // - Uncomment the block comment below (Find and remove the `/*` and `*/`).
    // - Create variables `tx` and `rx` and assign them to the sending and receiving ends of an
    // unbounded channel. Hint: An unbounded channel can be created with `channel::unbounded()`

    let (tx, rx) = channel::unbounded();

    // Cloning a channel makes another variable connected to that end of the channel so that you can
    // send it to another thread. We want another variable that can be used for sending...
    let tx2 = tx.clone();

    // 4. Examine the flow of execution of "Thread A" and "Thread B" below. Do you see how their
    // output will mix with each other?
    // - Run this code. Notice the order of output from Thread A and Thread B.
    // - Increase the value passed to the first `sleep_ms()` call in Thread A so that both the
    // Thread B outputs occur *before* Thread A outputs anything.
    // - Run the code again and make sure the output comes in a different order.

    // Thread A
    let handle_a = thread::spawn(move || {
        sleep_ms(35);
        tx2.send("Thread A: 1").unwrap();
        sleep_ms(20);
        tx2.send("Thread A: 2").unwrap();
    });

    sleep_ms(10); // Make sure Thread A has time to get going before we spawn Thread B

    // Thread B
    let handle_b = thread::spawn(move || {
        sleep_ms(0);
        tx.send("Thread B: 1").unwrap();
        sleep_ms(20);
        tx.send("Thread B: 2").unwrap();
    });

    // Using a Receiver channel as an iterator is a convenient way to get values until the channel
    // gets closed. A Receiver channel is automatically closed once all Sender channels have been
    // closed. Both our threads automatically close their Sender channels when they exit and the
    // destructors for the channels get automatically called.
    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    // 5. Oops, we forgot to join "Thread A" and "Thread B". That's bad hygiene!
    // - Use the thread handles to join both threads without getting any compiler warnings.
    handle_a.join().unwrap();
    handle_b.join().unwrap();

    // Challenge: Make two child threads and give them each a receiving end to a channel. From the
    // main thread loop through several values and print each out and then send it to the channel.
    // On the child threads print out the values you receive. Close the sending side in the main
    // thread by calling `drop(tx)` (assuming you named your sender channel variable `tx`). Join
    // the child threads.

    // Send stuff
    let (command_tx, command_rx) = channel::unbounded();
    let command_rx2 = command_rx.clone();

    // Receive stuff
    let (result_tx, result_rx) = channel::unbounded();
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

    println!("Main thread: Exiting.")
}
