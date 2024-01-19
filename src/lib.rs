use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;

pub struct Server { }

impl Server {
    pub fn handle_connection(mut stream: TcpStream) {
        // process the request
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();
        let get = b"GET / HTTP/1.1\r\n";

        let (status_line, file_name) = if buffer.starts_with(get) {
            ("HTTP/1.1 200 OK\r\n\r\n", "home.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
        };

        let contents = fs::read_to_string(file_name).unwrap();
        let response = format!("{}{}" ,status_line, contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}