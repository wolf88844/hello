use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();

    //println!("Request: {:#?}", http_request);

    let request_line = http_request.first().unwrap();

    let (status_line, filname) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        std::thread::sleep(std::time::Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filname).unwrap();
    let length = contents.len();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line, length, contents
    );

    stream.write_all(response.as_bytes()).unwrap();
}
