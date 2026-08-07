use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let mut parts = request_line.split_whitespace();

    let method = match parts.next() {
        Some(a) => a,
        None => return
    };

    match method {
        "GET" => {
            let url = match parts.next() {
                Some(val) => val,
                None => return
            };

            match url {
                "/" => serve_index(stream),
                "/style.css" => serve_style(stream),
                _ => not_found(stream)
            }
        },
        "POST" => {},
        "PUT" => {},
        "DELETE" => {},
        _ => {}
    }
}

fn serve_index(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("web/index.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn serve_style(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("web/style.css").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    
    stream.write_all(response.as_bytes()).unwrap();
}

fn not_found(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 404 Not Found";
    let contents = fs::read_to_string("web/status/404.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
