use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    time::Duration,
    thread,
};
use webserver::ThreadPool;

fn main() {
    let endpoint = "127.0.0.1:4242";
    let listener = TcpListener::bind(endpoint).unwrap();
    let pool = ThreadPool::new(4);

    println!("The server is running at http://{endpoint}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let mut parts = request_line.split_whitespace();

    let (Some(method), Some(path), Some(version), None) = (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        bad_request_400(&mut stream);
        return;
    };
    
    if version != "HTTP/1.1" {
        http_version_not_supported_505(&mut stream);
        return;
    }


    match method {
        "GET" => serve_get(&mut stream, path),
        // "POST" => serve_post(&mut stream, path),
        // "DELETE" => serve_delete(&mut stream, path),
        _ => method_not_allowed_405(&mut stream),
    }
}

/*
 * METHODS
 */
fn serve_get(stream: &mut TcpStream, path: &str) {
    // success: 200 OK, fail: 404 Not Found
    let (status, file, content_type) = match path {
        "/" => ("200 OK", "web/index.html", "text/html"),
        "/style.css" => ("200 OK", "web/style.css", "text/css"),
        "/script.js" => ("200 OK", "web/script.js", "text/javascript"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("200 OK", "web/index.html", "text/html")
        },
        _ => ("404 Not Found", "web/status/404.html", "text/html"),
    };
    let body = fs::read_to_string(file).unwrap();
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );

    stream.write_all(response.as_bytes()).unwrap();
}

// fn serve_post() {
//     // success: 201 Created
// }

// fn serve_delete() {
//     // success: 204 No Content
// }

/*
 * HTTP Status Codes
 */
fn bad_request_400(stream: &mut TcpStream) {
    stream
        .write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n")
        .unwrap();
}

fn method_not_allowed_405(stream: &mut TcpStream) {
    stream
        .write_all(b"HTTP/1.1 405 Method Not Allowed\r\nContent-Length: 0\r\n\r\n")
        .unwrap();
}

fn http_version_not_supported_505(stream: &mut TcpStream) {
    stream
        .write_all(b"HTTP/1.1 505 HTTP Version Not Supported\r\nContent-Length: 0\r\n\r\n")
        .unwrap();
}

