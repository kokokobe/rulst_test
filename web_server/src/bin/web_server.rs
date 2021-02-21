use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    println!("Shutting down.")
}

// fn handle_connection2(mut stream: TcpStream) {
//     println!("enter connection");
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//     println!("enter read");
// //        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
//     let get = b"GET / HTTP/1.1\r\n";
//     let (status_line, filename) = if buffer.starts_with(get) {
//         ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
//     } else {
//         ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
//     };
//     let contents = fs::read_to_string(filename).unwrap();
//     let response = format!("{}{}", status_line, contents);
//     stream.write_all(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }

fn handle_connection(stream: TcpStream) {
    let mut header = String::new();
    let mut buf_reader = BufReader::new(stream);
    buf_reader.read_line(&mut header).unwrap();
    println!("header is:{}", header);
//        println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = "GET / HTTP/1.1\r\n";
    let (status_line, filename) = if header.starts_with(get) {
        println!("enter hello");
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        println!("enter 404");
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "static/404.html")
    };
    let env = env!("CARGO_MANIFEST_DIR");
    let path = env.to_owned() + "/" + filename;
    println!("env is {}", env);
    println!("file path is:{}", path);
    let contents = fs::read_to_string(path).unwrap();
    let response = format!("{}{}", status_line, contents);
    let stream = buf_reader.get_mut();
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}