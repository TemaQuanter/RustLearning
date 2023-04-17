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
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line: &str = "HTTP/1.1 200 OK";
    let content: String = fs::read_to_string("index.html").expect("Failed to read the HTML page");
    let length: usize = content.len();

    let response: String = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");

    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {:#?}", http_request);
} // end handle_connection()
