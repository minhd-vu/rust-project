use std::net::TcpListener;

fn main() {
    // This will listen to incoming tcp connections.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // The incoming method gives us a stream of incoming connections. These are
    // actually connection attempts.
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
