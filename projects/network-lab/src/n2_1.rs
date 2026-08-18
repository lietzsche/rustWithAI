use std::io::{Cursor, Read, Write};

pub fn n2_1() {
    let mut output: Vec<u8> = Vec::new();
    output.write_all(b"PONG").expect("failed to write bytes");
    println!("written bytes: {:?}", output);
    println!("written length: {}", output.len());

    let mut cursor: Cursor<Vec<u8>> = Cursor::new(b"PING".to_vec());
    let mut read_buffer: [u8; 2] = [0u8; 2];
    loop {
        let read_count = cursor.read(&mut read_buffer).expect("fail to read");
        let read_bytes = &read_buffer[..read_count];
        println!("read count: {}", read_count);
        println!("read bytes: {:?}", read_bytes);
        if read_count == 0 {
            println!("EOF reached");
            break;
        }
    }
}

pub fn blocking_stdin_demo() {
    let stdin = std::io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut std_buffer = [0u8; 8];
    println!("before blocking read");
    let std_read_count = stdin_lock.read(&mut std_buffer).expect("fail to read");
    println!("after blocking read");
    println!("stdin read count: {}", std_read_count);
    println!("stdin bytes: {:?}", &std_buffer[..std_read_count]);
}
