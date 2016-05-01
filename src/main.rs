// Crypto, woo!
extern crate itertools;
mod hex;
mod base64;
mod xor;

use std::char;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::iter::repeat;

use base64::Base64Ext;
use hex::HexToBytesExt;
use xor::XorExt;

/// A handy utility iterator for converting a sequence of bytes to characters.
struct BytesToChars<I> {
    source: I
}

impl<I> Iterator for BytesToChars<I> where I: Iterator<Item=u8> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.source.next().and_then(|c| char::from_u32(c as u32))
    }
}

trait BytesToCharsExt: Sized {
    fn chars(self) -> BytesToChars<Self>;
}

impl<I> BytesToCharsExt for I where I: Iterator<Item=u8> {
    fn chars(self) -> BytesToChars<Self> {
        BytesToChars { source: self }
    }
}

#[test]
fn bytes_to_chars_works() {
    let bytes = [72, 101, 108, 108, 111];
    let string: String = bytes.iter().cloned().chars().collect();
    assert_eq!("Hello", string);
}


///---------------------------------------------------------------------------
/// Set 1, Challenge 1
///---------------------------------------------------------------------------
fn hex_to_base64(s: &str) -> String {
    s.chars().hexbytes().base64().collect()
}

#[test]
fn hex_to_base64_works() {
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c\
               696b65206120706f69736f6e6f7573206d757368726f6f6d";
    let expected = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9\
                    pc29ub3VzIG11c2hyb29t";
    assert_eq!(hex_to_base64(hex), expected);
}

///---------------------------------------------------------------------------
/// Set 1, Challenge 2
///---------------------------------------------------------------------------
fn xor(a: &str, b: &str) -> String {
    let mut result = String::new();
    for b in a.chars().hexbytes().xor(b.chars().hexbytes()) {
        let h = (b / 16) as u32;
        let l = (b % 16) as u32;
        result.push(char::from_digit(h, 16).unwrap());
        result.push(char::from_digit(l, 16).unwrap());
    }
    result
}

#[test]
fn xor_works() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    assert_eq!(xor(a, b), "746865206b696420646f6e277420706c6179");
}

///---------------------------------------------------------------------------
/// Set 1, Challenge 3
///---------------------------------------------------------------------------

/// Attempt to figure out a text's relative excellence
fn score(plaintext: &str) -> i32 {
    let mut score = 0;
    for c in plaintext.chars() {
        // Give rough score according to character frequency. I just
        // asspulled these numbers, but the idea is to reward lowercase
        // letters, slightly penalize capital letters and other printable
        // symbols, extremely penalize nonprintable symbols.
        match c {
            ' ' => { score += 1; },
            '!' ... '/' => { score -= 1; },
            '0' ... '9' => { score += 0; },
            ':' ... '@' => { score -= 2; },
            'A' ... 'Z' => { score -= 1; },
            '[' ... '`' => { score -= 2; },
            'a' ... 'z' => { score += 2; },
            '{' ... '~' => { score -= 1; },
            _ => { score -= 10; }
        }
    }
    score
}

fn decrypt_single_xor(ciphertext: &str) -> (i32, String) {
    let mut best = String::new();
    let mut best_score = i32::min_value();

    for i in 0u8..255u8 {
        let bytes = ciphertext.chars().hexbytes();
        let key = repeat(i);
        let plain = bytes.xor(key);
        let result: String = plain.chars().collect();
        let score = score(&result);

        if score >= best_score {
            best = result;
            best_score = score;
        }
    }

    println!("Best score: {} for \"{}\"", best_score, &best);
    (best_score, best)
}

#[test]
fn decrypt_single_xor_works() {
    let cipher = "1b37373331363f78151b7f2b783431333d7\
                  8397828372d363c78373e783a393b3736";

    let (_, result) = decrypt_single_xor(cipher);
    assert_eq!(result, "Cooking MC's like a pound of bacon");
}

///---------------------------------------------------------------------------
/// Set 1, Challenge 4
///---------------------------------------------------------------------------
fn find_single_char_xor(input_filename: &str)
        -> std::io::Result<(i32, String)> {
    let f = try!(File::open(input_filename));
    let file = BufReader::new(&f);

    let mut best_score = i32::min_value();
    let mut best = String::new();

    for line in file.lines() {
        let (score, result) = decrypt_single_xor(&line.unwrap());
        if score > best_score {
            best_score = score;
            best = result;
        }
    }
    Ok((best_score, best))
}

#[test]
fn find_single_char_xor_works() {
    let (score, result) = find_single_char_xor("4.txt").unwrap();
    println!("Found: {}, {}", score, result);
    assert_eq!(result, "Now that the party is jumping\n");
}

fn main() {
    println!("Hello, world!");
}
