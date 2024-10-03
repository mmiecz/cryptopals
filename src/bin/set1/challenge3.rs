use cryptopals::{hex, lang_detection, xor};
fn main() {
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
                    // f64 comparator sadness, proper thing to do here is to impl Ord PartialOrd Eq for (f64, text)...
                    most_probable.push(((likeness * 1000.0) as u32, msg));
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
