use cryptopals::{hex, xor};

fn main() {
    let input = "1c0111001f010100061a024b53535009181c";
    let key = "686974207468652062756c6c277320657965";

    let input_decoded = hex::decode(input).expect("input decode");
    let key_decoded = hex::decode(key).expect("key decode");

    let xored_bytes = xor::xor(&input_decoded, &key_decoded).expect("xor should succeed");

    let encoded = hex::encode(&xored_bytes);
    assert_eq!(encoded, "746865206b696420646f6e277420706c6179".to_string());
}
