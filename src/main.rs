use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;

const ADDRESS: &str = "127.0.0.1:8080";
const BUFF_SZ: usize = 1024;

fn main() {
    let listener = TcpListener::bind(ADDRESS).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handler(stream);
    }
}

fn handler(mut stream: TcpStream) {
    let request = read_request(&mut stream);
    println!("Request: {}", request);
    write_response(&mut stream);
}

fn read_request(stream: &mut TcpStream) -> String {
    let mut buffer = [0; BUFF_SZ];
    stream.read(&mut buffer).unwrap();
    return String::from_utf8_lossy(&buffer[..]).into();
}

fn write_response(stream: &mut TcpStream) {
    let contents = fs::read_to_string("./resources/hello.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}