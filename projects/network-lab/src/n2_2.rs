use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

pub fn tcp_server_demo() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind to address");
    println!("before accept");
    let (mut stream, client_address) = listener.accept().expect("failed to accept connection");
    println!("after accept, client address: {}", client_address);

    let local = stream.local_addr().expect("fail to get local address");
    println!("local: {local}");
    let peer = stream.peer_addr().expect("fail to get peer address");
    println!("peer: {peer}");

    let mut buffer = [0u8; 1];
    let read_count = stream.read(&mut buffer).expect("failed to read");
    let received = &buffer[..read_count];
    println!("read count: {read_count}");
    println!("received: {received:?}");
    stream.write_all(received).expect("failed to write");

    println!("before second read");
    let second_read_count = stream.read(&mut buffer).expect("failed to second read");
    println!("second read count: {second_read_count}");
    if second_read_count == 0 {
        println!("EOF from client");
    }
}

pub fn tcp_client_demo() {
    println!("before connect");
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("fail to connect");
    println!("after connect");
    let local = stream.local_addr().expect("fail to get local address");
    println!("local: {local}");
    let peer = stream.peer_addr().expect("fail to get peer address");
    println!("peer: {peer}");

    stream.write_all(b"X").expect("failed to write message");
    let mut buffer = [0u8; 1];
    let read_count = stream.read(&mut buffer).expect("failed to read");
    let received = &buffer[..read_count];
    println!("read count: {read_count}");
    println!("received: {received:?}");

    println!("before client shutdown");
    stream
        .shutdown(Shutdown::Both)
        .expect("failed to shutdown both");
    println!("after client shutdown");
}
