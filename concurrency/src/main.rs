use std::{thread, time::Duration};

fn main() {
    // After calling, notice that all the threads will shutdown, regardless of
    // whether they completed or not.
    create_new_thread();
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
