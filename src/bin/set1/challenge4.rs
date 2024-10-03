use cryptopals::hex;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::BufRead;

fn main() {
    let file = File::open("inputs/challenge4.txt").expect("Input file not found.");
    // max heap of (score, line)
    let mut probable_solutions: BinaryHeap<(u32, String)> = BinaryHeap::new();
    for line in std::io::BufReader::new(file).lines() {
        let bytes =
            hex::decode(&line.expect("line read error")).expect("proper line with hex values");
        for key_byte in 0..255u8 {
            // string lengths are fixed but I'm lazy
            let key = vec![key_byte; bytes.len()];
            let decoded = cryptopals::xor::xor(&bytes, &key).expect("xor decode should success");
            match String::from_utf8(decoded) {
                Ok(msg) => {
                    let likeness = cryptopals::lang_detection::english_likeness(&msg);
                    if likeness > 0.95 {
                        probable_solutions.push(((likeness * 1000.0) as u32, msg));
                    }
                }
                _ => {}
            }
        }
    }
    println!("top 3 best solutions");
    println!("1: {}", probable_solutions.pop().unwrap().1);
    println!("2: {}", probable_solutions.pop().unwrap().1);
    println!("3: {}", probable_solutions.pop().unwrap().1);
}
