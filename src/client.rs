use std::io::Write;
use std::net::TcpStream;

fn main() {
    println!("client started");

    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("client failed to connect");

    println!("client connected");

    stream
        .write_all(b"hello server")
        .expect("failed to write to tcp stream");
}
