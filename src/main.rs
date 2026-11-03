use std::env;
use std::io::Write;
use std::net::TcpListener;

fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 22\r\n\r\nframeworkless rust ok\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
