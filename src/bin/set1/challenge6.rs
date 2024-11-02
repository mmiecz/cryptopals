use cryptopals::base64;
use cryptopals::lang_detection::english_likeness;
use cryptopals::util::hamming;
use std::fs::File;
use std::io;
use std::io::BufRead;

fn find_key_size(input: &[u8]) -> Vec<usize> {
    let mut scores: Vec<_> = (2..=40)
        .map(|key_size| {
            let mut distances = Vec::new();
            let chunks: Vec<&[u8]> = input.chunks(key_size).take(4).collect();
            for i in 0..chunks.len() - 1 {
                for j in i + 1..chunks.len() {
                    if chunks[i].len() == key_size && chunks[j].len() == key_size {
                        let distance = hamming::distance(chunks[i], chunks[j]).unwrap();
                        distances.push(distance as f64 / key_size as f64);
                    }
                }
            }
            let avg_distance = distances.iter().sum::<f64>() / distances.len() as f64;
            (avg_distance, key_size)
        })
        .collect();

    scores.sort_by(|a, b| a.0.partial_cmp(&b.0).expect("float comparison :|"));
    scores.into_iter().map(|(_, k)| k).collect()
}

fn transpose(buff: &[u8], size: usize) -> Vec<Vec<u8>> {
    let mut blocks = vec![vec![]; size];
    for (i, b) in buff.iter().enumerate() {
        blocks[i % size].push(*b);
    }
    blocks
}

// single bye xor key is used to encrypt block, find it
fn decrypt_block(block: &[u8]) -> u8 {
    let mut scores = Vec::new();
    for key in 0..=255 {
        let decrypted: Vec<u8> = block.iter().map(|byte| byte ^ key).collect();
        let english_score = english_likeness(&String::from_utf8_lossy(&decrypted));
        scores.push((english_score, decrypted, key));
    }

    // sort by most likely key
    scores.sort_by(|a, b| b.0.partial_cmp(&a.0).expect("float comparison"));
    scores[0].2
}

// tries to find proper key with key_size.
fn find_key(encrypted: &[u8], key_size: usize) -> Vec<u8> {
    // transpose encrypted bytes with key_size
    let blocks = transpose(encrypted, key_size);
    let mut key = Vec::new();
    for block in blocks {
        let block_key = decrypt_block(&block);
        key.push(block_key);
    }
    key
}

fn main() {
    let file = File::open("inputs/challenge6.txt").expect("Input file not found.");
    let base64_input = io::BufReader::new(file)
        .lines()
        .fold(String::new(), |mut acc, line| {
            acc.push_str(line.unwrap().as_str());
            acc
        });
    let encrypted = base64::decode(&base64_input).unwrap();

    let key_sizes = find_key_size(&encrypted);
    let mut decrypted = vec![0u8; encrypted.len()];
    for key_size in key_sizes.iter().take(2) {
        println!("keysize: {key_size}");
        let key = find_key(&encrypted, *key_size);

        for (i, b) in encrypted.iter().enumerate() {
            decrypted[i] = b ^ key[i % key.len()];
        }

        println!("key: {}", String::from_utf8_lossy(&key));
        println!("message: {}", String::from_utf8_lossy(&decrypted));
    }
}
