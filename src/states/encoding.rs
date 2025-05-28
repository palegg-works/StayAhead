pub fn xor_encrypt(input: &str, key: &str) -> Vec<u8> {
    input
        .as_bytes()
        .iter()
        .zip(key.as_bytes().iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect()
}

pub fn xor_decrypt(encrypted: &[u8], key: &str) -> String {
    encrypted
        .iter()
        .zip(key.as_bytes().iter().cycle())
        .map(|(a, b)| a ^ b)
        .map(|b| b as char)
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn decode_hex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks(2)
        .map(|pair| {
            let hi = (pair[0] as char).to_digit(16).unwrap();
            let lo = (pair[1] as char).to_digit(16).unwrap();
            (hi * 16 + lo) as u8
        })
        .collect()
}
