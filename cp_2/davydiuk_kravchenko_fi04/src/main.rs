use std::fs;

fn main() {
    let text_content = fs::read_to_string("TEXT").expect("problem with reading of file");
    let key_content = fs::read_to_string("KEY").expect("problem with reading of file");
    let s = String::from("a я привет");

    // for i in 'a'..'я' {
    //     println!("{i}: {:?}", i.to_digit(10));
    // }
    println!("{}", vigenere_encode(text_content.as_str(), key_content.as_str()));
}

fn vigenere_encode(text: &str, key: &str) -> String {
    let mut result = String::new();
    let text_bytes = text.as_bytes();
    let key_bytes = key.as_bytes();
    let key_len = key.len();

    for (i, &byte) in text_bytes.iter().enumerate() {
        let key_byte = key_bytes[i % key_len];
        let encoded_byte = ((byte as i16 + key_byte as i16 - 2 * 0xC0_i16) % 32_i16 + 32_i16) % 32_i16 + 0xC0_i16;
        result.push(encoded_byte as u8 as char);
    }

    result
}
