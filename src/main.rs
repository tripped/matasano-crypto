// Crypto, woo!
#![feature(iter_arith)]

extern crate itertools;
mod hex;
mod base64;
mod xor;

use std::char;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
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

///---------------------------------------------------------------------------
/// Set 1, Challenge 5
///---------------------------------------------------------------------------

/// Consume an iterator of u8, turning it into a string of hex pairs.
/// XXX: make this an iterator adapter u8->char
fn bytes_to_hex<I: Iterator<Item=u8>>(iter: I) -> String {
    itertools::Unfold::new((iter, None), |state| {
        let (ref mut iter, ref mut leftover) = *state;
        match *leftover {
            Some(nybble) => {
                *leftover = None;
                char::from_digit(nybble, 16)
            },
            None => match iter.next() {
                Some(byte) => {
                    let h = (byte / 16) as u32;
                    let l = (byte % 16) as u32;
                    *leftover = Some(l);
                    char::from_digit(h, 16)
                },
                None => None
            }
        }
    }).collect()
}

#[test]
fn bytes_to_hex_works() {
    let bytes = [0, 1, 67, 127, 255];
    assert_eq!(bytes_to_hex(bytes.iter().cloned()), "0001437fff");
}

fn repeating_key_xor(text: &str, key: &str) -> String {
    // XXX: assumes ASCII inputs
    let text = text.bytes();
    let key = key.bytes().cycle();
    bytes_to_hex(text.xor(key))
}

#[test]
fn repeating_key_xor_works() {
    let input = "Burning 'em, if you ain't quick and nimble\n\
                 I go crazy when I hear a cymbal";
    assert_eq!(
        repeating_key_xor(input, "ICE"),
        "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d\
         63343c2a26226324272765272a282b2f20430a652e2c652a31\
         24333a653e2b2027630c692b20283165286326302e27282f");
}

///---------------------------------------------------------------------------
/// Set 1, Challenge 6
///---------------------------------------------------------------------------

/// Consume two u8 iterators and return the hamming distance between their
/// contents.
fn hamming_distance<A, B>(a: A, b: B) -> usize
        where A: Iterator<Item=u8>,
              B: Iterator<Item=u8> {
    /// Hamming distance between two u8s
    fn dist((a, b): (u8, u8)) -> usize {
        let mut x = a ^ b;
        let mut bits = 0;
        while x != 0 {
            bits += 1;
            x &= x - 1;
        }
        bits
    }

    a.zip(b).map(dist).sum()
}

/// Convenience: compute the hamming distance between two strings.
fn hamming_distance_str(a: &str, b: &str) -> usize {
    assert_eq!(a.len(), b.len());
    hamming_distance(a.bytes(), b.bytes())
}

#[test]
fn hamming_distance_works() {
    assert_eq!(37, hamming_distance_str("this is a test", "wokka wokka!!!"));
}

/// Return the five "best" key-length candidates (between 2 and 42) for the
/// given file.
fn find_key_lengths(filename: &str) -> std::io::Result<Vec<usize>> {
    let f = try!(File::open(filename));
    let mut file = BufReader::new(&f);

    // XXX: the test file is small, so just read the whole thing into memory.
    // Ideally we'd use BufReader's bytes() implementation, but that is an
    // iterator over std::io::Result<u8> and not u8, and adapting is a pain.
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let mut keys: Vec<_> = (2..42).map(|keysize| {
        let mut chunks = data.chunks(keysize);
        let first = chunks.nth(0).unwrap().iter().cloned();
        let second = chunks.nth(1).unwrap().iter().cloned();
        let n = (hamming_distance(first, second) * 1000) / keysize;

        println!("{} -> {}", keysize, n);
        (keysize, n)
    }).collect();

    // Sort according to the normalized hamming distance
    keys.sort_by_key(|&(_, score)| score);

    Ok(keys.iter().cloned().take(5).map(|(k, _)| k).collect())
}

#[test]
fn find_key_length_works() {
    assert_eq!(
        vec![2, 10, 12, 14, 3],
        find_key_lengths("6.txt").unwrap());
}

fn main() {
    println!("Hello, world!");
}
