#![allow(unused_imports)]
use byteorder::{BigEndian, WriteBytesExt};
use bytes::{Bytes, BytesMut};
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
struct Header {
    // request_api_key : i16,
    // request_api_version:i16,
    correlation_id: i32,
    // client_id: String,
}

struct Message {
    message_size: u32,
    body: String,
    header: Header,
}

fn handle_client(mut stream: TcpStream) {
    let mut buf = [0u8; 1024];

    match stream.read(&mut buf) {
        Ok(bytes) => {
            println!("Bytes: {}", bytes);
            let message_size = u32::from_be_bytes(buf[0..4].try_into().unwrap());
            let request_api_key = u16::from_be_bytes(buf[4..6].try_into().unwrap());
            let request_api_version = u16::from_be_bytes(buf[6..8].try_into().unwrap());
            let correlation_id = u32::from_be_bytes(buf[8..12].try_into().unwrap());

            println!("message_size: {}", message_size);
            println!("request_api_key: {}", request_api_key);
            println!("request_api_version: {}", request_api_version);
            println!("correlation_id: {}", correlation_id);
            let mut retbuf = Vec::new();
            retbuf.write_u32::<BigEndian>(0).expect("Unable to write into buffer");
            retbuf.write_u32::<BigEndian>(correlation_id).expect("Unable to write into buffer");
            println!("returned buffer : {:?}",retbuf);
            stream.write(&retbuf).unwrap();
        }
        Err(e) => {println!("{:?}",e)}
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
