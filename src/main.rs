#![allow(unused_imports)]
use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};

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
    let buf = [0,0,0,0,0,0,0,7];
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
