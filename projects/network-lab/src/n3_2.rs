use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

pub fn partial_write_demo() {
    println!();
    println!("enter partial_write_demo");

    let mut destination = [0u8; 3];
    let source = b"HELLO";

    let written;

    {
        let mut writer = &mut destination[..];
        written = writer.write(source).expect("failed to write");
    }

    println!("requested: {}", source.len());
    println!("written: {}", written);
    println!("remaining: {:?}", &source[written..]);

    destination = [0u8; 3];

    {
        let mut writer = &mut destination[..];
        match writer.write_all(b"HELLO") {
            Ok(()) => println!("write_all succeeded"),
            Err(error) => println!("write_all error kind: {}", error.kind()),
        }
    }

    println!("destination after error: {:?}", destination);
}

pub fn partial_read_demo() {
    println!();
    println!("enter partial_read_demo");

    let mut reader: &[u8] = b"HELLO";
    let mut buffer = [0u8; 3];

    let read_count = reader.read(&mut buffer).expect("failed to read");

    println!("requested: {}", buffer.len());
    println!("read count: {}", read_count);
    println!("received: {:?}", &buffer[..read_count]);
    println!("reader remaining: {:?}", reader);
}

pub fn read_exact_demo() {
    println!();
    println!("enter read_exact_demo");

    let mut reader: &[u8] = b"HELLO";
    let mut buffer = [0u8; 5];

    reader
        .read_exact(&mut buffer)
        .expect("failed to read exact");
    println!("complete read: {:?}", buffer);

    reader = b"HEL";
    buffer = [0u8; 5];

    match reader.read_exact(&mut buffer) {
        Ok(()) => println!("complete read: {:?}", buffer),
        Err(error) => println!("short read error kind: {:?}", error.kind()),
    }
}

pub fn exact_server_demo() {
    println!();
    println!("enter exact_server_demo");

    let listener = TcpListener::bind("127.0.0.1:7880").expect("failed to bind to address");
    let (mut stream, _addr) = listener.accept().expect("failed to accept");
    let mut buffer = [0u8; 8];

    println!("before read_exact");
    stream
        .read_exact(&mut buffer)
        .expect("failed to read exact");
    println!("after read_exact");
    println!(
        "buffer: {}",
        std::str::from_utf8(&buffer).expect("failed to convert to utf8")
    );
}
pub fn exact_client_demo() {
    println!();
    println!("enter exact_client_demo");

    let mut stream = TcpStream::connect("127.0.0.1:7880").expect("failed to connect");

    println!("write PING");
    stream.write_all(b"PING").expect("failed to write PING");
    println!("write PONG");
    stream.write_all(b"PONG").expect("failed to write PONG");

    stream
        .shutdown(Shutdown::Write)
        .expect("failed to shutdown write");
}
