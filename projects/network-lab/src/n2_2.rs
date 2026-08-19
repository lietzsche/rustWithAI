use std::net::{TcpListener, TcpStream};

pub fn tcp_server_demo() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind to address");
    println!("before accept");
    let (stream, client_address) = listener.accept().expect("failed to accept connection");
    println!("after accept, client address: {}", client_address);

    let local = stream.local_addr().expect("fail to get local address");
    println!("local: {local}");
    let peer = stream.peer_addr().expect("fail to get peer address");
    println!("peer: {peer}");
}

pub fn tcp_client_demo() {
    println!("before connect");
    let stream = TcpStream::connect("127.0.0.1:7878").expect("fail to connect");
    println!("after connect");
    let local = stream.local_addr().expect("fail to get local address");
    println!("local: {local}");
    let peer = stream.peer_addr().expect("fail to get peer address");
    println!("peer: {peer}");
}
