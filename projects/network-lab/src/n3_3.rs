use std::io::{ErrorKind, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

pub fn early_close_server_demo() {
    println!();
    println!("early close server enter");

    let listener = TcpListener::bind("127.0.0.1:7881").expect("failed to bind");
    let (mut stream, _client_addr) = listener.accept().expect("failed to accept client");
    let mut buffer = [0u8; 8];

    match stream.read_exact(&mut buffer) {
        Ok(()) => println!("read complete: {:?}", buffer),
        Err(error) if error.kind() == ErrorKind::UnexpectedEof => {
            println!("client closed before 8 bytes")
        }
        Err(error) => println!("failed to read from client, error kind: {}", error.kind()),
    }
}
pub fn early_close_client_demo() {
    println!();
    println!("early close client enter");

    let mut stream = TcpStream::connect("127.0.0.1:7881").expect("failed to connect to server");
    stream
        .write_all(b"PING")
        .expect("failed to write to server");

    println!("sent 4 bytes");

    stream
        .shutdown(Shutdown::Write)
        .expect("failed to shutdown write");
}

pub fn timeout_server_demo() {
    println!();
    println!("timeout server enter");

    let listener = TcpListener::bind("127.0.0.1:7882").expect("failed to bind");
    let (mut stream, _client_addr) = listener.accept().expect("failed to accept client");
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("failed to set read timeout");

    let mut buffer = [0u8; 1];

    println!("before timeout read");
    match stream.read(&mut buffer) {
        Ok(0) => println!("EOF"),
        Ok(read_count) => {
            println!("read count: {read_count}");
            println!("bytes: {:?}", &buffer[..read_count]);
        }
        Err(error) if matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
            println!("read time out: {:?}", error.kind())
        }
        Err(error) => println!("error: {}, error kind: {:?}", error, error.kind()),
    }
}
pub fn silent_client_demo() {
    println!();
    println!("silent client enter");

    let _stream = TcpStream::connect("127.0.0.1:7882").expect("failed to connect server");
    thread::sleep(Duration::from_secs(2));
}
