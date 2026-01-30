use learning_tcp::TcpRequest;
use std::fs::create_dir_all;
use std::fs::read;
use std::fs::write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let address = "127.0.0.1:7878";
    let listener = TcpListener::bind(address).expect("failed to bind port");

    create_dir_all("server_files").unwrap();

    println!("server started with address {address}");

    for connection in listener.incoming() {
        let mut connection = connection.expect("client request not accepted");
        println!("connection accepted");

        let client_request =
            TcpRequest::decode_request(&mut connection).expect("failed to decode request");
        handle_request(client_request, &mut connection).unwrap();
    }
}

fn handle_request(request: TcpRequest, stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let filepath = format!("server_files/{}", request.filename());

    match request {
        TcpRequest::Upload(_, data) => write(filepath, data),
        TcpRequest::Download(filename) => {
            let return_request = TcpRequest::Upload(filename, read(filepath).unwrap());
            return_request.encode_request(stream)
        }
    }
}
