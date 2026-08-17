pub fn n1_3() {
    let mut n1_3_vec: Vec<u8> = Vec::with_capacity(4);
    println!("initial buffer length: {}", n1_3_vec.len());
    println!("initial buffer capacity: {}", n1_3_vec.capacity());
    n1_3_vec.extend_from_slice(b"GET");
    println!("buffer length: {}", n1_3_vec.len());
    println!("buffer capacity: {}", n1_3_vec.capacity());
    n1_3_vec.extend_from_slice(b" /hp");
    println!("buffer length: {}", n1_3_vec.len());
    println!("buffer capacity: {}", n1_3_vec.capacity());
    println!("request buffer: {:?}", n1_3_vec);
    let mut read_position: usize = 0;
    let command_bytes: &[u8] = &n1_3_vec[read_position..3];
    read_position = 3;
    let remaining_bytes: &[u8] = &n1_3_vec[read_position..];
    println!("command bytes: {:?}", command_bytes);
    println!("remaining bytes: {:?}", remaining_bytes);
    println!(
        "command text: {}",
        std::str::from_utf8(command_bytes).unwrap()
    );
    println!(
        "remaining text: {}",
        std::str::from_utf8(remaining_bytes).unwrap()
    );
    println!("read position: {}", read_position);
    println!("buffer length after read: {}", n1_3_vec.len());
}
