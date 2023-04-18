use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener: TcpListener =
        TcpListener::bind("127.0.0.1:7878").expect("Failed to bind to the address 127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream: TcpStream = stream.expect("Failed to unwrap a stream");

        handle_connection(stream);
    } // end for
} // end main()

fn handle_connection(mut stream: TcpStream) {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line: String = buf_reader.lines().next().unwrap().unwrap();

    // Check what type of request it is and act accordingly.

    let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
        // It is a GET request
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    }; // end if

    let content: String = fs::read_to_string(file_name).expect("Failed to read an HTML page");
    let length: usize = content.len();

    let response: String = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();
} // end handle_connection()
