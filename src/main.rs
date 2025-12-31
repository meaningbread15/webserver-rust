use webserve::Threadpool;
use std::{fs,
        io::{BufRead, BufReader, Write},
        net::{TcpListener, TcpStream},
        thread, time::Duration
    };

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = Threadpool::new(4);

    for stream in listener.incoming(){
        let stream = stream.unwrap();
            // handle_connection(stream);

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream){
    let bufread = BufReader::new(&stream);
    let req_line = bufread.lines().next().unwrap().unwrap();

    let (statusline, filename) = match &req_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "static/hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "static/sleep.html")
        },
        _ => ("HTTP/1.1 404 ERROR", "static/404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{statusline}\r\nContent-Length:{length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    println!("Request: {req_line:#?}");
}
