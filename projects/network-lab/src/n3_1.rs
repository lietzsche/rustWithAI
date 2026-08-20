use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

pub fn tcp_server_demo() {
    println!("\nstart n3_1::tcp_server_demo");

    let listener = TcpListener::bind("127.0.0.1:7879").expect("failed to bind to address");
    let (mut stream, _client_addr) = listener.accept().expect("failed to accept connection");

    let mut read_buffer = [0u8; 8];

    loop {
        let read_count = stream.read(&mut read_buffer).expect("failed to read");
        if read_count == 0 {
            println!("EOF");
            break;
        }
        println!("read count: {read_count}");
        let bytes_slice = &read_buffer[..read_count];
        match std::str::from_utf8(bytes_slice) {
            Ok(s) => println!("bytes slice: {}", s),
            Err(error) => println!("failed to convert bytes slice to string, error: {}", error),
        }
    }
}

pub fn tcp_client_demo() {
    println!("\nstart n3_1::tcp_client_demo");

    let mut stream = TcpStream::connect("127.0.0.1:7879").expect("failed to connect");

    println!("write PING");
    stream.write_all(b"PING").expect("failed to write PING");
    println!("write PONG");
    stream.write_all(b"PONG").expect("failed to write PONG");
    stream
        .shutdown(Shutdown::Write)
        .expect("failed to shutdown write");
}
