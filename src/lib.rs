//! Iterator zippers
//!
//! This crate provides "zippers" as tuple structs: `Zip2`, `Zip3`, etc. These structs accept
//! iterators as fields, and provides iteration over the "zipped" elements.
//!
//! The functionality is similar to the `Iterator::zip` method. The difference is that the `zip`
//! method always produces an iterator that yields *two-element* tuples. If you want to zip 3
//! iterators, you'll write `xs.zip(ys).zip(zs)` which returns an iterator that yields elements of
//! type `((X, Y), Z)`. On the other hand, with this library you can write `Zip3(xs, ys, zs)` which
//! is an iterator that yields elements of type `(X, Y, Z)`.
//!
//! ```
//! use zip::Zip3;
//!
//! let chars = &['a', 'b', 'c'];
//! let mut v = vec![0u, 1, 2];
//! for (&c, i, &mut j) in Zip3(chars.iter(), range(0u, 5), v.iter_mut()) {
//!     assert!(i < 3);
//!     assert_eq!(i, j);
//! }
//! ```

#![deny(missing_docs, warnings)]
#![feature(macro_rules)]

macro_rules! min {
    ($x:expr) => { $x };
    ($x:expr, $($y:expr),+) => { ::std::cmp::min($x, min!($($y),+)) };
    ($($x:expr),+,) => { min!($($x),+) };
}

/// Two-iterator zipper
pub struct Zip2<A, B>(pub A, pub B);

impl<A, B, AI, BI> Iterator<(A, B)> for Zip2<AI, BI> where
    AI: Iterator<A>,
    BI: Iterator<B>,
{
    fn next(&mut self) -> Option<(A, B)> {
        if let Some(a) = self.0.next() {
            if let Some(b) = self.1.next() {
                return Some((a, b));
            }
        }

        None
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (min!{
            self.0.size_hint().0,
            self.1.size_hint().0,
        }, None)
    }
}

/// Three-iterator zipper
pub struct Zip3<A, B, C>(pub A, pub B, pub C);

impl<A, B, C, AI, BI, CI> Iterator<(A, B, C)> for Zip3<AI, BI, CI> where
    AI: Iterator<A>,
    BI: Iterator<B>,
    CI: Iterator<C>,
{
    fn next(&mut self) -> Option<(A, B, C)> {
        if let Some(a) = self.0.next() {
            if let Some(b) = self.1.next() {
                if let Some(c) = self.2.next() {
                    return Some((a, b, c));
                }
            }
        }

        None
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (min!{
            self.0.size_hint().0,
            self.1.size_hint().0,
            self.2.size_hint().0,
        }, None)
    }
}

/// Four-iterator zipper
pub struct Zip4<A, B, C, D>(pub A, pub B, pub C, pub D);

impl<A, B, C, D, AI, BI, CI, DI> Iterator<(A, B, C, D)> for Zip4<AI, BI, CI, DI> where
    AI: Iterator<A>,
    BI: Iterator<B>,
    CI: Iterator<C>,
    DI: Iterator<D>,
{
    fn next(&mut self) -> Option<(A, B, C, D)> {
        if let Some(a) = self.0.next() {
            if let Some(b) = self.1.next() {
                if let Some(c) = self.2.next() {
                    if let Some(d) = self.3.next() {
                        return Some((a, b, c, d))
                    }
                }
            }
        }

        None
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (min!{
            self.0.size_hint().0,
            self.1.size_hint().0,
            self.2.size_hint().0,
            self.3.size_hint().0,
        }, None)
    }
}

/// Five-iterator zipper
pub struct Zip5<A, B, C, D, E>(pub A, pub B, pub C, pub D, pub E);

impl<A, B, C, D, E, AI, BI, CI, DI, EI> Iterator<(A, B, C, D, E)>
for Zip5<AI, BI, CI, DI, EI> where
    AI: Iterator<A>,
    BI: Iterator<B>,
    CI: Iterator<C>,
    DI: Iterator<D>,
    EI: Iterator<E>,
{
    fn next(&mut self) -> Option<(A, B, C, D, E)> {
        if let Some(a) = self.0.next() {
            if let Some(b) = self.1.next() {
                if let Some(c) = self.2.next() {
                    if let Some(d) = self.3.next() {
                        if let Some(e) = self.4.next() {
                            return Some((a, b, c, d, e))
                        }
                    }
                }
            }
        }

        None
    }

    fn size_hint(&self) -> (uint, Option<uint>) {
        (min!{
            self.0.size_hint().0,
            self.1.size_hint().0,
            self.2.size_hint().0,
            self.3.size_hint().0,
            self.4.size_hint().0,
        }, None)
    }
}

#[cfg(test)]
mod bench {
    extern crate test;

    use self::test::Bencher;

    const SIZE: uint = 1024;

    #[bench]
    fn collect2(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let b = a.clone();

        z.iter(|| {
            ::Zip2(a.iter(), b.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn collect2_(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let b = a.clone();

        z.iter(|| {
            a.iter().zip(b.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn count2(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let b = a.clone();

        z.iter(|| {
            ::Zip2(a.iter(), b.iter()).count()
        })
    }

    #[bench]
    fn count2_(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let b = a.clone();

        z.iter(|| {
            a.iter().zip(b.iter()).count()
        })
    }

    #[bench]
    fn collect3(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c) = (a.clone(), a.clone());

        z.iter(|| {
            ::Zip3(a.iter(), b.iter(), c.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn collect3_(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c) = (a.clone(), a.clone());

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn count3(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c) = (a.clone(), a.clone());

        z.iter(|| {
            ::Zip3(a.iter(), b.iter(), c.iter()).count()
        })
    }

    #[bench]
    fn count3_(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c) = (a.clone(), a.clone());

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).count()
        })
    }

    #[bench]
    fn collect4(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c, d) = (a.clone(), a.clone(), a.clone());

        z.iter(|| {
            ::Zip4(a.iter(), b.iter(), c.iter(), d.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn collect4_(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c, d) = (a.clone(), a.clone(), a.clone());

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn count4(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c, d) = (a.clone(), a.clone(), a.clone());

        z.iter(|| {
            ::Zip4(a.iter(), b.iter(), c.iter(), d.iter()).count()
        })
    }

    #[bench]
    fn count4_(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c, d) = (a.clone(), a.clone(), a.clone());

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter()).count()
        })
    }

    #[bench]
    fn collect5(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c, d, e) = (a.clone(), a.clone(), a.clone(), a.clone());

        z.iter(|| {
            ::Zip5(a.iter(), b.iter(), c.iter(), d.iter(), e.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn collect5_(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c, d, e) = (a.clone(), a.clone(), a.clone(), a.clone());

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter()).zip(e.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn count5(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c, d, e) = (a.clone(), a.clone(), a.clone(), a.clone());

        z.iter(|| {
            ::Zip5(a.iter(), b.iter(), c.iter(), d.iter(), e.iter()).count()
        })
    }

    #[bench]
    fn count5_(z: &mut Bencher) {
        let a = Vec::from_elem(SIZE, 0u8);
        let (b, c, d, e) = (a.clone(), a.clone(), a.clone(), a.clone());

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter()).zip(e.iter()).count()
        })
    }
}
