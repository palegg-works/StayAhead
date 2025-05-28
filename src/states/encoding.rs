const CRYPT_KEY: &str = "OBFUSCATION_ONLY";

pub fn encode(input: &str) -> String {
    let encrypted_pat = xor_encrypt(input, CRYPT_KEY);
    encode_hex(&encrypted_pat)
}

pub fn decode(input: &str) -> String {
    let decoded_pat = decode_hex(input);
    xor_decrypt(&decoded_pat, CRYPT_KEY)
}

fn xor_encrypt(input: &str, key: &str) -> Vec<u8> {
    input
        .as_bytes()
        .iter()
        .zip(key.as_bytes().iter().cycle())
        .map(|(a, b)| a ^ b)
        .collect()
}

fn xor_decrypt(encrypted: &[u8], key: &str) -> String {
    encrypted
        .iter()
        .zip(key.as_bytes().iter().cycle())
        .map(|(a, b)| a ^ b)
        .map(|b| b as char)
        .collect()
}

fn encode_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

fn decode_hex(s: &str) -> Vec<u8> {
    s.as_bytes()
        .chunks(2)
        .map(|pair| {
            let hi = (pair[0] as char).to_digit(16).unwrap();
            let lo = (pair[1] as char).to_digit(16).unwrap();
            (hi * 16 + lo) as u8
        })
        .collect()
}
