use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;

fn main() {
    // This will listen to incoming tcp connections.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // The incoming method gives us a stream of incoming connections. These are
    // actually connection attempts.
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        println!("Connection established!");
        // This is single threaded.
        // handle_connection(stream);

        // This would create a new thread to handle each stream. This is problem because there is
        // no limit and could overload the system.
        // thread::spawn(|| {
        //     handle_connection(stream);
        // });

        // This is using a thread pool to handle requests.
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // This will collect the lines of request that the browser sends to the server.
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        // Take lines until there is an empty line.
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);

    let request_line = &http_request[0];

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // Read in the HTML file and return it as the response.
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Write an OK message back to the client.
    stream.write_all(response.as_bytes()).unwrap();
}
