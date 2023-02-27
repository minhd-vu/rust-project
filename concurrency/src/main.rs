use std::{sync::mpsc, thread, time::Duration};

fn main() {
    // After calling, notice that all the threads will shutdown, regardless of
    // whether they completed or not.
    create_new_thread();
    message_passing();
}

fn create_new_thread() {
    let v = vec![1, 2, 3];
    // Save the JoinHandle in a new variable. We have to use the move variable
    // in the thread closure to ensure that the thread takes ownership of v.
    let handle = thread::spawn(move || {
        println!("here's a vector {:?}", v);

        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // We can't do this because the thread has taken ownership of v.
    // drop(v);

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Without this, the thread could terminate early. This ensures the thread
    // finishes. If we move this before the main thread for loop, the prints will
    // no longer be executing concurrently.
    handle.join().unwrap();
}

fn message_passing() {
    // mpsc stand for multiple producer, single consumer. A channel can have
    // multiple ends producing messages, but can only have one end consuming
    // messages. Think of it as a river where all producer's messages will end
    // up in the same river in the end.
    let (tx, rx) = mpsc::channel();
    // Here tx stands for transmitter (sender) and rx stand for receiver.

    let tx1 = tx.clone();
    thread::spawn(move || {
        // We used move so the new thread now owns the transmitter.
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            // The send method can error if the receiver has already been dropped
            // or the there's no where to send the value.
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
            // The below statement isn't allowed, because val could be modified or
            // dropped before we got to use it again.
            // println!("val is {}", val);
        }
    });

    // Because we're using multiple producer, we can have multiple threads
    // sending to the channel.
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // The recv() method blocks until there is a message that is received in the
    // channel. When the transmitter closes, it can return an error to signal
    // that there are no more values coming in the Result. There is also a
    // try_recv which won't block and immediately return.
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // Here rx is being treated as an iterator. Each value is printed and if the
    // channel is closed, then iteration will stop.
    for received in rx {
        println!("Got: {}", received);
    }
}
