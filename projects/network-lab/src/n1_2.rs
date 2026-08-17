pub fn n1_2() {
    let n1_2_byte: u32 = 0x12345678;
    let be: [u8; 4] = n1_2_byte.to_be_bytes();
    let le: [u8; 4] = n1_2_byte.to_le_bytes();
    println!("number: {:#010x}", n1_2_byte);
    println!("big endian bytes: {:?}", be);
    println!("little endian bytes: {:?}", le);
    println!("decoded big endian: {}", u32::from_be_bytes(be));
    println!("decoded little endian: {}", u32::from_le_bytes(le));
    println!("wrong endian decoded: {}", u32::from_le_bytes(be));
}
