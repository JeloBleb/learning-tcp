use std::io::Read;
use std::net::TcpListener;
use std::str::from_utf8;

fn main() {
    let address = "127.0.0.1:7878";
    let listener = TcpListener::bind(address).expect("failed to bind port");

    println!("server started with address {address}");

    for connection in listener.incoming() {
        let mut connection = connection.expect("client request not accepted");
        println!("connection accepted");

        let mut byte_buffer = [0u8; 512];
        let bytes_read = connection
            .read(&mut byte_buffer)
            .expect("uh oh, failed to read message");
        let message = from_utf8(&byte_buffer[..bytes_read]).expect("message was not utf-8");

        println!("client sent message: {message} ");
    }
}
