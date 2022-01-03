use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Running on port 3000...");

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 5];

        stream.read_exact(&mut buffer).unwrap();
        stream.write_all(&buffer).unwrap();
        println!("{}", std::str::from_utf8(&buffer).unwrap());
    }
}
