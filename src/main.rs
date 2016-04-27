// Crypto, woo!

/// Iterator adapter version of b64encode
struct Base64<I> {
    source: I,

    // It's obnoxious to have to keep continuation state in a struct.
    // It would be nice if Rust had continuations or generators.
    bits: u16,
    n: i8,
    count: usize,
}

impl<I> Base64<I> {
    fn new(source: I) -> Base64<I> {
        Base64 {
            source: source,
            bits: 0,
            n: 0,
            count: 0,
        }
    }
}

impl<I> Iterator for Base64<I> where I: Iterator<Item=u8> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        const CODES: &'static str =
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        // Try to consume from source if there are fewer than 6 bits available
        if self.n < 6 {
            match self.source.next() {
                Some(byte) => {
                    self.bits &= !(0xffff >> self.n); // preserve high n bits
                    self.bits |= (byte as u16) << (8 - self.n);
                    self.n += 8;
                },
                None => {
                    if self.n < 1 {
                        if (self.count % 4) != 0 {
                            self.count += 1;
                            return Some('=');
                        } else {
                            return None;
                        }
                    }
                }
            }
        }

        // Otherwise, consume 6 available bits
        self.n -= 6;
        self.bits = self.bits.rotate_left(6);
        self.count += 1;
        return Some(CODES.as_bytes()[(self.bits & 0x3f) as usize] as char);
    }
}

trait Base64Ext: Sized {
    fn base64(self) -> Base64<Self>;
}

impl<I> Base64Ext for I where I: Iterator<Item=u8> {
    fn base64(self) -> Base64<Self> {
        Base64::new(self)
    }
}


/// Return the base64 encoding of a byte slice.
fn b64encode(data: &[u8]) -> String {
    data.iter().cloned().base64().collect()
}

#[test]
fn b64_empty() {
    assert_eq!(b64encode(&[]), "");
}

#[test]
fn b64_onebyte() {
    // 0000 00|00
    assert_eq!(b64encode(&[0x00]), "AA==");
    // 0000 00|01 (0000) = 0, 16
    assert_eq!(b64encode(&[0x01]), "AQ==");
    // 0001 00|01 (0000) = 4, 16
    assert_eq!(b64encode(&[0x11]), "EQ==");
    // 1111 11|11 (0000) = 63, 48
    assert_eq!(b64encode(&[0xff]), "/w==");
}

#[test]
fn b64_more_data() {
    assert_eq!(
        b64encode(
            &[0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c, 0x69, 0x6e,
              0x67, 0x20, 0x79, 0x6f, 0x75, 0x72, 0x20, 0x62, 0x72, 0x61,
              0x69, 0x6e, 0x20, 0x6c, 0x69, 0x6b, 0x65, 0x20, 0x61, 0x20,
              0x70, 0x6f, 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20,
              0x6d, 0x75, 0x73, 0x68, 0x72, 0x6f, 0x6f, 0x6d]),
        "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

/// An iterator adapter that consumes an iterator of hex digits and
/// produces a stream of bytes.
struct HexToBytes<I> {
    source: I
}

impl<I> Iterator for HexToBytes<I> where I: Iterator<Item=char> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        fn hex_digit(c: char) -> Option<u8> {
            c.to_digit(16).map(|c| c as u8)
        }
        self.source.next()
            .and_then(hex_digit)
            .and_then(|h| self.source.next().and_then(hex_digit)
                 .map(|l| h * 16 + l))
    }
}

trait HexToBytesExt: Sized {
    fn hexbytes(self) -> HexToBytes<Self>;
}

impl<I> HexToBytesExt for I where I: Iterator<Item=char> {
    fn hexbytes(self) -> HexToBytes<Self> {
        HexToBytes { source: self }
    }
}

#[test]
fn hex_to_bytes() {
    let mut i = HexToBytes { source: "deadbeef".chars() };
    assert_eq!(0xde, i.next().unwrap());
    assert_eq!(0xad, i.next().unwrap());
    assert_eq!(0xbe, i.next().unwrap());
    assert_eq!(0xef, i.next().unwrap());
    assert_eq!(None, i.next());
}

#[test]
fn hex_truncated_byte() {
    let mut i = HexToBytes { source: "c0ffe".chars() };
    assert_eq!(0xc0, i.next().unwrap());
    assert_eq!(0xff, i.next().unwrap());
    assert_eq!(None, i.next());
}

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

fn xor(a: &str, b: &str) -> String {
    String::new()
}

#[test]
fn xor_works() {
    let a = "1c0111001f010100061a024b53535009181c";
    let b = "686974207468652062756c6c277320657965";
    assert_eq!(xor(a, b), "746865206b696420646f6e277420706c6179");
}

fn main() {
    println!("Hello, world!");
}
