#![allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write,Cursor};
use bytes::{Bytes, BytesMut};
use byteorder::{BigEndian, WriteBytesExt};
struct Header{
    // request_api_key : i16,
    // request_api_version:i16,
    correlation_id:i32,
    // client_id: String,
}

struct Message{
    message_size : u32,
    body:String,
    header: Header,
}

fn handle_client(mut stream: TcpStream)
{
    // let buf = [0,0,0,0,0,0,0,7];
    let mut buf = Vec::new();
    buf.write_u32::<BigEndian>(0).expect("Unable to write into buffer");
    let request_api_key :i16 = 0;
    let request_api_version : i16 = 0;
    let correlation_id :i16 = 7;
    buf.write_i16::<BigEndian>(request_api_key).expect("Errror Writing the api key");
    buf.write_i16::<BigEndian>(request_api_version).expect("Errror Writing the api version");
    buf.write_i16::<BigEndian>(correlation_id).expect("Errror Writing the correlation_id");
    eprintln!("{:?}",&buf);
    stream.write(&buf).unwrap();
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
                println!("accepted new connection");
                handle_client(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
