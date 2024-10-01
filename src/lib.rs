pub mod hex;
mod lang_detection;
pub mod xor;

/// Put challenges tests here
#[cfg(test)]
mod cryptochallenge_tests {
    use crate::hex;
    use std::cmp::Ordering;

    #[test]
    fn set1_challenge1() {
        use super::hex;
        let good_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let err = hex::to_base64(good_hex).expect("should work");
        assert_eq!(
            err,
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()
        );
    }

    #[test]
    fn set1_challenge2() {
        use super::{hex, xor};
        let input = "1c0111001f010100061a024b53535009181c";
        let key = "686974207468652062756c6c277320657965";

        let input_decoded = hex::decode(input).expect("input decode");
        let key_decoded = hex::decode(key).expect("key decode");

        let xored_bytes = xor::xor(&input_decoded, &key_decoded).expect("xor should succeed");

        let encoded = hex::encode(&xored_bytes);
        assert_eq!(encoded, "746865206b696420646f6e277420706c6179".to_string());
    }

    #[test]
    // run with nocapture
    fn set1_challenge3() {
        use super::{hex, lang_detection, xor};
        let cipher = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
        let cipher = hex::decode(cipher).expect("cipher decode error");

        let mut most_probable = std::collections::BinaryHeap::new();
        for key_byte in 0..255u8 {
            // expand key to same length as cipher bytes
            let key = vec![key_byte; cipher.len()];
            let deciphered = xor::xor(&cipher, &key).expect("xor success");
            match String::from_utf8(deciphered) {
                Ok(msg) => {
                    let likeness = lang_detection::english_likeness(&msg);
                    if likeness > 0.95 {
                        most_probable.push(((likeness * 1000.0) as u32, msg)); // f64 comparator sadness, proper thing to do here is to impl Ord PartialOrd Eq for (f64, text)...
                    }
                }
                _ => {}
            }
        }
        let (likeness, text) = most_probable.peek().expect("at least one element");
        println!("most probable msg with eng probability {likeness}");
        println!("{text}");

        assert_eq!("Cooking MC's like a pound of bacon", text);
    }
}
