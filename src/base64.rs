use std::collections::HashMap;
use thiserror::Error;

const BASE64_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Debug, Error)]
enum Base64Error {
    #[error("invalid character {0}")]
    InvalidCharacter(char),
}
/// encode input of bytes to base64 encoded string
pub fn encode(input: &[u8]) -> String {
    let base64_bytes: Vec<u8> = input
        .chunks(3)
        .flat_map(|chunk| {
            let mut n = u32::from(chunk[0]) << 16;
            if chunk.len() > 1 {
                n |= u32::from(chunk[1]) << 8;
            }
            if chunk.len() > 2 {
                n |= u32::from(chunk[2]);
            }

            let mut q = [b'='; 4];
            q[0] = BASE64_ALPHABET[(n >> 18 & 63) as usize];
            q[1] = BASE64_ALPHABET[(n >> 12 & 63) as usize];
            if chunk.len() > 1 {
                q[2] = BASE64_ALPHABET[(n >> 6 & 63) as usize];
            }
            if chunk.len() > 2 {
                q[3] = BASE64_ALPHABET[(n & 63) as usize];
            }
            q
        })
        .collect();
    String::from_utf8(base64_bytes).unwrap()
}

/// Decode from base64 input to vector of bytes
/// buggy implementation - doesn't support error handling very well (no padding errors, etc.)
pub fn decode(input: &str) -> Result<Vec<u8>, Base64Error> {
    let mapping: HashMap<char, u8> = BASE64_ALPHABET
        .iter()
        .enumerate()
        .map(|(idx, char)| (*char as char, idx as u8))
        .collect();

    let mut buff: u32 = 0;
    let mut bytes = Vec::new();
    let mut bits = 0;
    for c in input.chars() {
        if c == '=' {
            break;
        }
        match mapping.get(&c) {
            Some(c) => {
                buff = (buff << 6) | (*c as u32);
                bits += 6;

                if bits >= 8 {
                    bits -= 8;
                    bytes.push((buff >> bits) as u8);
                }
            }
            None => return Err(Base64Error::InvalidCharacter(c)),
        }
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn base64_encode_and_decode() {
        let text = "Hello World";
        let encoded = encode(text.as_bytes());
        let decoded = decode(&encoded).unwrap();
        assert_eq!(text, String::from_utf8(decoded).unwrap());
    }

    #[test]
    fn base64_single_byte() {
        let hex = &[255];
        let result = encode(hex);
        assert_eq!(result, "/w==");
    }

    #[test]
    fn base64_two_bytes() {
        let hex = &[255, 255];
        let result = encode(hex);
        assert_eq!(result, "//8=");
    }

    #[test]
    fn base64_three_bytes() {
        let hex = &[255, 255, 255];
        let result = encode(hex);
        assert_eq!(result, "////");
    }

    #[test]
    fn base64_empty_string() {
        let hex = "";
        let result = encode(hex.as_bytes());
        assert_eq!(result, "");
    }

    #[test]
    fn decode_invalid_character() {
        let input = "SGVsbG8sIFdvcmxkI!Q==";
        let res = decode(input);
        assert!(res.is_err());
    }
}
