mod challenge2;

use cryptopals::hex;
fn main() {
    let good_hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let err = hex::to_base64(good_hex).expect("should work");
    assert_eq!(
        err,
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string()
    );
}
