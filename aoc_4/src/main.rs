use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn part_1() -> i32 {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut valid_passphrases = 0;
    for line in reader.lines() {
        if is_valid_1(&line.unwrap()) { valid_passphrases += 1; }
    }

    valid_passphrases
}

fn part_2() -> i32 {
    let input_file = File::open("src/input.txt").unwrap();
    let reader = BufReader::new(input_file);

    let mut valid_passphrases = 0;
    for line in reader.lines() {
        if is_valid_2(&line.unwrap()) { valid_passphrases += 1; }
    }

    valid_passphrases
}


fn main() {
    println!("Valid passphrases {}", part_1());
    println!("Valid passphrases {}", part_2());
}

fn is_valid_1(passphrase : &str) -> bool {
    let mut words = HashMap::new();
    for word in passphrase.split_whitespace() {
        *words.entry(word).or_insert(0) += 1;
    }

    if words.iter().any(|(_, &count)| count > 1) { return false; } else { return true; }
}

fn is_valid_2(passphrase : &str) -> bool {
    let mut words = HashMap::new();
    for word in passphrase.split_whitespace() {
        let mut sorted_chars = word.chars().collect::<Vec<char>>();
        sorted_chars.sort();
        let sorted_string :String = sorted_chars.iter().collect();
        *words.entry(sorted_string).or_insert(0) += 1;
    }

    if words.iter().any(|(_, &count)| count > 1) { return false; } else { return true; }
}

