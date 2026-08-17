fn main() {
    println!("network lab");

    let ascii_text = "GET";
    println!("ascii text: {}", ascii_text);
    println!("ascii byte length: {}", ascii_text.len());

    let ascii_bytes: &[u8] = ascii_text.as_bytes();
    println!("ascii bytes: {:?}", ascii_bytes);

    let utf8_text = "가";
    println!("utf8 text: {}", utf8_text);
    println!("utf8 byte length: {}", utf8_text.len());
    println!("utf8 char count: {}", utf8_text.chars().count());

    let utf8_bytes: &[u8] = utf8_text.as_bytes();
    println!("utf8 bytes: {:?}", utf8_bytes);

    let mut buffer: Vec<u8> = ascii_text.as_bytes().to_vec();
    buffer.push(b'!');
    println!("buffer: {:?}", buffer);
    println!("buffer length: {}", buffer.len());

    let n1_2_byte: u32 = 0x12345678;
    let be: [u8; 4] = n1_2_byte.to_be_bytes();
    let le: [u8; 4] = n1_2_byte.to_le_bytes();
    println!("number: {:#010x}", n1_2_byte);
    println!("big endian bytes: {:?}", be);
    println!("little endian bytes: {:?}", le);
    println!("decoded big endian: {}", u32::from_be_bytes(be));
    println!("decoded little endian: {}", u32::from_le_bytes(le));
    println!("wrong endian decoded: {}", u32::from_le_bytes(be));

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
