use std::io::Read;
use std::{io::Write, net::TcpStream};

pub enum TcpRequest {
    Upload(String, Vec<u8>),
    Download(String),
}

impl TcpRequest {
    pub fn encode_request(self, stream: &mut TcpStream) -> Result<(), std::io::Error> {
        match self {
            Self::Upload(file_name, file_data) => {
                stream.write_all(&[0u8])?;
                stream.write_all(&(file_name.len() as u8).to_be_bytes())?;
                stream.write_all(file_name.as_bytes())?;
                stream.write_all(&(file_data.len() as u32).to_be_bytes())?;
                stream.write_all(&file_data)
            }
            Self::Download(filename) => {
                stream.write_all(&[1u8])?;
                stream.write_all(&(filename.len() as u8).to_be_bytes())?;
                stream.write_all(filename.as_bytes())
            }
        }
    }

    pub fn decode_request(stream: &mut TcpStream) -> Result<TcpRequest, std::io::Error> {
        let mut kind = [0u8; 1];
        stream.read_exact(&mut kind)?;
        let kind = kind[0];

        let mut file_name_len = [0u8; 1];
        stream.read_exact(&mut file_name_len)?;
        let file_name_len = file_name_len[0];

        let mut file_name = vec![0u8; file_name_len.into()];
        stream.read_exact(&mut file_name)?;
        let file_name = std::str::from_utf8(&file_name)
            .expect("file name buffer not utf-8")
            .to_string();

        match kind {
            0 => {
                let mut file_data_len = [0u8; 4];
                stream.read_exact(&mut file_data_len)?;
                let file_data_len = u32::from_be_bytes(file_data_len);

                let mut file_data = vec![0u8; file_data_len as usize];
                stream.read_exact(&mut file_data)?;

                Ok(TcpRequest::Upload(file_name, file_data))
            }
            1 => Ok(TcpRequest::Download(file_name)),
            _ => todo!(),
        }
    }
}
