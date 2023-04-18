use hello::ThreadPool;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to the address 127.0.0.1:7878");

    let thread_pool: ThreadPool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.expect("Failed to unwrap a stream");

        thread_pool.execute(|| handle_connection(stream));
    } // end for
} // end main()

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    // Check what type of request it is and act accordingly.

    let (status_line, file_name) = match &request_line[..] {
        // It is a GET request
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    }; // end match

    let content: String = fs::read_to_string(file_name).expect("Failed to read an HTML page");
    let length: usize = content.len();

    let response: String = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
} // end handle_connection()
