use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;

fn handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                println!("{}", from_utf8(&read).unwrap());
                stream.write(&read[0..n]).unwrap();
            }
            Err(_err) => {
                // probs just reset, nothing to fret about
                // println!("Error: {}", err);
                // panic!(err);
            }
        }
    }
}

pub(crate) fn start() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    println!("Server starting, streaming log statements...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}