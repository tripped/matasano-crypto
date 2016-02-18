// Crypto, woo!

/// Return the base64 encoding of a byte slice.
fn b64encode(data: &[u8]) -> String {
    const CODES: &'static str =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut data = data.iter().peekable();
    let mut result = String::new();

    let mut accumulator: u16 = 0;   // Bit storage
    let mut n = 0;                  // Number of accumulated bits

    loop {
        // Consume another byte if we have fewer than 6 bits available
        if n < 6 {
            match data.next() {
                Some(byte) => {
                    accumulator &= !(0xffff >> n); // preserve only high n bits
                    accumulator |= (*byte as u16) << (8 - n);
                    n += 8;
                },
                None => if n < 1 {
                    break;
                }
            }
        }

        // Yoink six bits from the accumulator and emit a base64 codepoint
        n -= 6;
        accumulator = accumulator.rotate_left(6);
        result.push(CODES.as_bytes()[(accumulator & 0x3f) as usize] as char);
    }

    // Add padding
    while result.len() % 4 != 0 {
        result.push('=');
    }

    result
}

/// Iterator adapter version of b64encode
struct Base64<T: Iterator<Item=u8>> {
    source: T,

    // It's obnoxious to have to keep continuation state in a struct.
    // It would be nice if Rust had continuations or generators.
    bits: u16,
    n: usize,
    count: usize,
}

impl<T: Iterator<Item=u8>> Base64<T> {
    fn new(source: T) -> Base64<T> {
        Base64 {
            source: source,
            bits: 0,
            n: 0,
            count: 0,
        }
    }
}

impl<T: Iterator<Item=u8>> Iterator for Base64<T> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
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
                    if self.n < 1 && (self.count % 4) != 0 {
                        return Some('=');
                    } else {
                        return None;
                    }
                }
            }
        }

        // Otherwise, consume 6 available bits
        self.n -= 6;
        self.bits = self.bits.rotate_left(6);
        return Some(CODES.as_bytes()[(self.bits & 0x3f) as usize] as char);
    }
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
struct HexToBytes<T: Iterator<Item = char>> {
    source: T
}

impl<T: Iterator<Item = char>> Iterator for HexToBytes<T> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        fn hex_digit(c: char) -> Option<u8> {
            c.to_digit(16).map(|c| c as u8)
        }
        self.source.next()
            .and_then(hex_digit)
            .and_then(|h| self.source.next().and_then(hex_digit)
                 .map(|l| h * 16 + l))
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

fn main() {
    println!("Hello, world!");
}
