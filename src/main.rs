use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let endpoint = "127.0.0.1:7878";
    let listener = TcpListener::bind(endpoint).unwrap();

    println!("The server is running at http://{endpoint}");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let mut parts = request_line.split_whitespace();

    let (Some(method), Some(path), Some(version), None) = (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        // TODO: 400 Bad Request: malformed request_line
        bad_request_400(&mut stream);
        return;
    };
    
    if version != "HTTP/1.1" {
        // TODO: 505 HTTP Version Not Supported
        http_version_not_supported_505();
        return;
    }

    /*
    match method {
        "GET" => match path { // success: 200
            "/" => ,
            "/style.css" => ,
            "/script.js" => ,
            _ => // fail: 404
        },
        "POST" => match path { // success: 201
            
        },
        "DELETE" => match path { // success: 204
        
        },
        _ => // 405 Method Not Allowed
    }
    */ 

    /*
    let response = match method {
        "GET" => serve_get(path),
        "POST" => serve_post(path),
        "DELETE" => serve_delete(path),
        _ => {
            method_not_allowed_405();
            return;
        }
    };

    stream.write_all(response.as_bytes()).unwrap();
}

fn serve_get() {
    match path {

    }
}

fn serve_post() {}

fn serve_delete() {}
    */
}

fn bad_request_400(stream: &mut TcpStream) {
    stream
        .write_all(b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n")
        .unwrap();
}

fn method_not_allowed_405() {}

fn http_version_not_supported_505() {}
