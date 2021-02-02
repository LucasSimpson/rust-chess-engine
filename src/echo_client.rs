use std::io::Write;

/**
The hackiest shit LOL
*/
pub fn log(message: &str) {
    match std::net::TcpStream::connect("localhost:8080") {
        Ok(mut stream) => {
            stream.write(message.as_ref()).unwrap();
        },
        Err(_e) => {
            println!("Trace: {}", message);
        }
    }
}
