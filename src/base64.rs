/// Base64 encoder implemented as a handy iterator adapter
pub struct Base64<I> {
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

pub trait Base64Ext: Sized {
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
