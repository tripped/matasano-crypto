/// An iterator adapter that consumes an iterator of hex digits and
/// produces a stream of bytes.
pub struct HexToBytes<I> {
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

pub trait HexToBytesExt: Sized {
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
