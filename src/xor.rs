/// The xor iterator of two u8 iterators
pub struct Xor<X, Y> {
    a: X,
    b: Y,
}

impl<X, Y> Xor<X, Y> {
    fn new(a: X, b: Y) -> Xor<X, Y> {
        Xor { a: a, b: b }
    }
}

impl<X, Y> Iterator for Xor<X, Y> where X: Iterator<Item=u8>,
                                        Y: Iterator<Item=u8>, {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next()) {
            (Some(a), Some(b)) => Some(a ^ b),
            (None, _) | (_, None) => None,
        }
    }
}

pub trait XorExt<X, Y>: Sized {
    fn xor(self, other: Y) -> Xor<X, Y>;
}

impl<X, Y> XorExt<X, Y> for X where X: Iterator<Item=u8> {
    fn xor(self, other: Y) -> Xor<X, Y> {
        Xor::new(self, other)
    }
}

#[test]
fn xor_iterator_works() {
    let a = [0, 1, 0, 1, 7];
    let b = [0, 0, 1, 1, 5];

    let a = a.iter().cloned();
    let b = b.iter().cloned();

    let result: Vec<u8> = a.xor(b).collect();

    assert_eq!(result, vec![0, 1, 1, 0, 2]);
}
