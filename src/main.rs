#![allow(unused_imports)]
use byteorder::{BigEndian, WriteBytesExt};
use bytes::{Bytes, BytesMut};
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
#[derive(Debug)]
struct Message {
    message_size: u32,
    request_api_key: u16,
    request_api_version: u16,
    correlation_id: u32,
    error: Option<ErrorCode>,
}
#[derive(Debug)]
enum ErrorCode {
    ApiVersion,
    Other,
}

impl Message {
    fn new(buf: &[u8; 1024]) -> Self {
        let message_size = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        let request_api_key = u16::from_be_bytes(buf[4..6].try_into().unwrap());
        let request_api_version = u16::from_be_bytes(buf[6..8].try_into().unwrap());
        let correlation_id = u32::from_be_bytes(buf[8..12].try_into().unwrap());

        let mut error: Option<ErrorCode> = None;
        if request_api_version > 4 || request_api_version < 0 {
            error = Some(ErrorCode::ApiVersion)
        }
        let message = Message {
            message_size,
            request_api_key,
            request_api_version,
            correlation_id,
            error,
        };

        let mut retbuf = Vec::new();
        retbuf
            .write_u32::<BigEndian>(0)
            .expect("Unable to write into buffer");
        retbuf
            .write_u32::<BigEndian>(correlation_id)
            .expect("Unable to write into buffer");

        message
    }

    fn response(&self) -> Vec<u8> {
        let mut retbuf = Vec::new();
        retbuf
            .write_u32::<BigEndian>(0)
            .expect("Unable to write into buffer");
        retbuf
            .write_u32::<BigEndian>(self.correlation_id)
            .expect("Unable to write into buffer");

        match self.error {
            Some(ErrorCode::ApiVersion) => {
                retbuf.write_u16::<BigEndian>(35).expect("error while writing ErrorCode");
            }
            _ => {}
        }
        retbuf
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 1024];

    match stream.read(&mut buf) {
        Ok(bytes) => {
            eprintln!("Amount of Recieved Bytes: {}", bytes);
            let message = Message::new(&buf);
            println!("Message : {:?}", &message);
            let retbuf = message.response();
            println!("returned buffer : {:?}", retbuf);
            stream.write(&retbuf).unwrap();
        }
        Err(e) => {
            println!("{:?}", e)
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection {:?}", &_stream);
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
