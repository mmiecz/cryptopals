use cryptopals::{hex, xor};

fn main() {
    let input = r#"Burning 'em, if you ain't quick and nimble
I go crazy when I hear a cymbal"#;

    let key = "ICE".as_bytes();

    let encrypted = xor::encrypt(input.as_bytes(), key).expect("encryption should succeed");
    let hex_result = hex::encode(&encrypted);

    let expected = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";

    println!("{hex_result}");
    assert_eq!(expected, &hex_result);
}
