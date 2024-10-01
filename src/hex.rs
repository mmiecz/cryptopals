use std::fmt::Write;
use thiserror::Error;

const BASE64_ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

#[derive(Debug, Error, PartialOrd, PartialEq)]
pub enum ConversionError {
    #[error("malformed hex string {0}")]
    MalformedHex(String),
    #[error("invalid hex char {0}")]
    InvalidHexChar(char),
    #[error("invalid hex char {0} at position {1}")]
    InvalidHexCharInString(char, usize),
    #[error("invalid hex string len {0}")]
    InvalidHexStringLen(usize),
}
fn to_nibble(ch: char) -> Result<u8, ConversionError> {
    match ch {
        '0'..='9' => Ok(ch as u8 - b'0'),
        'a'..='f' => Ok(ch as u8 - b'a' + 10),
        'A'..='F' => Ok(ch as u8 - b'A' + 10),
        _ => Err(ConversionError::InvalidHexChar(ch)),
    }
}

/// Decodes hex string to raw bytes. It doesn't expect prefix in form of 0x or 0X
pub fn decode(s: &str) -> Result<Vec<u8>, ConversionError> {
    if s.len() % 2 != 0 {
        return Err(ConversionError::InvalidHexStringLen(s.len()));
    }

    let bytes = s
        .chars()
        .try_fold((0, 0, Vec::new()), |(pos, mut byte, mut acc), ch| {
            // is pos % 2 == 0 we begin new byte
            let nibble =
                to_nibble(ch).map_err(|_| ConversionError::InvalidHexCharInString(ch, pos))?;
            if pos % 2 == 0 {
                byte |= nibble << 4;
                Ok((pos + 1, byte, acc))
            } else {
                byte |= nibble;
                acc.push(byte);
                Ok((pos + 1, 0, acc))
            }
        });
    Ok(bytes?.2)
}

pub fn encode(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x}");
        output
    })
}

/// Converts string hex number to base64 string, returns Conversion error when input is malformed
pub fn to_base64(input: &str) -> Result<String, ConversionError> {
    if input.len() % 2 != 0 {
        return Err(ConversionError::MalformedHex(input.to_string()));
    }

    let bytes = decode(input)?;
    let base64_bytes: Vec<u8> = bytes
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

    Ok(String::from_utf8(base64_bytes).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn str_conversion_error() {
        let odd_hex = "FFA";
        let err = to_base64(odd_hex);
        assert!(err.is_err());
    }
    #[test]
    fn str_to_bytes_simple() {
        let hex = "FFA0";
        let bytes = decode(hex).expect("good parse");
        assert_eq!(bytes[0], 255u8);
        assert_eq!(bytes[1], 160u8);
    }
    #[test]
    fn str_to_bytes_lowercase() {
        let hex = "deadbeef";
        let bytes = decode(hex).expect("Failed to parse lowercase hex");
        assert_eq!(bytes, vec![222, 173, 190, 239]);
    }
    #[test]
    fn str_to_bytes_uppercase() {
        let hex = "DEADBEEF";
        let bytes = decode(hex).expect("Failed to parse lowercase hex");
        assert_eq!(bytes, vec![222, 173, 190, 239]);
    }
    #[test]
    fn str_to_bytes_empty() {
        let hex = "";
        let bytes = decode(hex).expect("Failed to parse lowercase hex");
        assert_eq!(bytes, vec![]);
    }
    #[test]
    fn invalid_first_char() {
        let hex = "GHIJ";
        let result = decode(hex);
        assert_eq!(result, Err(ConversionError::InvalidHexCharInString('G', 0)));
    }
    #[test]
    fn invalid_next_char() {
        let hex = "FFAFOG";
        let result = decode(hex);
        assert_eq!(result, Err(ConversionError::InvalidHexCharInString('O', 4)));
    }

    #[test]
    fn empty_buff_to_hex() {
        let buf = vec![];
        let result = encode(&buf);
        assert_eq!(result, "".to_string());
    }
    #[test]
    fn single_max_byte_to_hex() {
        let buf = vec![255];
        let result = encode(&buf);
        assert_eq!(result, "ff".to_string());
    }
    #[test]
    fn single_byte_to_hex() {
        let buf = vec![15];
        let result = encode(&buf);
        assert_eq!(result, "0f".to_string());
    }

    #[test]
    fn many_bytes_to_hex() {
        let buf = vec![15, 255, 09];
        let result = encode(&buf);
        assert_eq!(result, "0fff09".to_string());
    }

    #[test]
    fn base64_empty_string() {
        let hex = "";
        let result = to_base64(hex).expect("Failed to convert empty string");
        assert_eq!(result, "");
    }

    #[test]
    fn base64_single_byte() {
        let hex = "FF";
        let result = to_base64(hex).expect("Failed to convert single byte string");
        assert_eq!(result, "/w==");
    }

    #[test]
    fn base64_two_bytes() {
        let hex = "FFFF";
        let result = to_base64(hex).expect("Failed to convert single byte string");
        assert_eq!(result, "//8=");
    }

    #[test]
    fn base64_three_bytes() {
        let hex = "FFFFFF";
        let result = to_base64(hex).expect("Failed to convert single byte string");
        assert_eq!(result, "////");
    }
}
