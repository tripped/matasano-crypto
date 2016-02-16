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

fn main() {
    println!("Hello, world!");
}
