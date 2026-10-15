# Webserver

## What is it?
This is a small multithreaded HTTP server Rust learning project based on [Chapter 21 of The Rust Programming Language](https://doc.rust-lang.org/stable/book/ch21-00-final-project-a-web-server.html) It runs an HTTP/1.1 server at 127.0.0.1:4242 and handles connection with pools of four threads.

### Implemented Methods
- [x] GET
- [ ] POST 
- [ ] PUT
- [ ] PATCH
- [ ] DELETE

### Response Content-Types
- text/html
- text/css
- text/javascript

### Routes
|Route|Behavior|
|---|---|
|/|web/index.html|
|/style.css|web/style.css|
|/script.js|web/script.js|
|/sleep|waits five seconds, then serves web/index.html|

## Usage
The project should be run from root because the server reads files from the web/ directory.

Terminal 1:
```bash
# run
cargo run
```

Terminal 2:
```bash
# test
curl -i http://127.0.0.1:4242/
```
