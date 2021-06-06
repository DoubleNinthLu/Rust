use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use std::{fs, thread};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (content_path, status_line) = if buffer.starts_with(get) {
        ("hello.html", "HTTP/1.1 200 OK\r\n\r\n")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("hello.html", "HTTP/1.1 200 OK\r\n\r\n")
    } else {
        ("404.html", "HTTP/1.1 404 NOT FOUND\r\n\r\n")
    };
    let contents = fs::read_to_string(content_path).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
