use std::fs::File;
use std::io;
use std::io::BufRead;
use std::ops::Add;

fn main() {
    let file = File::open("inputs/challenge6.txt").expect("Input file not found.");
    let base64_input = io::BufReader::new(file)
        .lines()
        .fold(String::new(), |mut acc, line| {
            acc.push_str(line.unwrap().as_str());
            acc
        });
    println!("{base64_input}");
}
