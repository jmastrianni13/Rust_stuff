use std::fs;
use std::io;
use std::io::prelude::*;
use std::net;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: net::TcpStream) {
    let buf_reader = io::BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let status_line;
    let contents;

    if request_line == "GET / HTTP/1.1" {
        status_line = "HTTP/1.1 200 OK";
        contents = fs::read_to_string("src/hello.html").unwrap();
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
        contents = fs::read_to_string("src/404.html").unwrap();
    }

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
