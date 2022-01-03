use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    stream.write_all(b"Hello").unwrap();

    let mut buffer = [0; 5];
    stream.read_exact(&mut buffer).unwrap();

    println!(
        "Response from server:{:?}",
        std::str::from_utf8(&buffer).unwrap()
    );
}
