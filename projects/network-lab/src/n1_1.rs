pub fn n1_1() {
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
}
