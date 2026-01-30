use learning_tcp::TcpRequest;
use std::fs::{create_dir_all, write};
use std::path::Path;
use std::{fs::read, net::TcpStream};

fn main() {
    create_dir_all("client_files").unwrap();

    println!("client started");

    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("client failed to connect");

    println!("client connected");

    upload(Path::new("test"), &mut stream);

    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("client failed to connect");

    download("server-to-client", &mut stream);
}

fn upload(filepath: &Path, stream: &mut TcpStream) {
    let request = TcpRequest::Upload(
        filepath.file_name().unwrap().to_str().unwrap().to_string(),
        read(filepath).unwrap(),
    );
    request.encode_request(stream).unwrap();
}

fn download(filename: &str, stream: &mut TcpStream) {
    let request = TcpRequest::Download(filename.to_string());
    request.encode_request(stream).unwrap();

    if let TcpRequest::Upload(_, data) = TcpRequest::decode_request(stream).unwrap() {
        write(format!("client_files/{filename}"), data).unwrap();
    }
}
