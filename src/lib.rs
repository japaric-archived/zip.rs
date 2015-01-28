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
//! let mut v = vec![0, 1, 2];
//! for (&c, i, &mut j) in Zip3(chars.iter(), 0i32..5, v.iter_mut()) {
//!     assert!(i < 3);
//!     assert_eq!(i, j);
//! }
//! ```

#![allow(unstable)]
#![deny(missing_docs, warnings)]

/// This macro emulates an "any-arity" free function that zips iterators
///
/// **Note** This macro calls `into_iter()` on the inputs. For this reason, this macro won't work
/// until `IntoIterator` lands in stdlib, or unless you provide your own `IntoIterator` trait.
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate zip;
///
/// # use std::slice;
/// # trait IntoIterator {
/// #     type Iter: Iterator;
/// #     fn into_iter(self) -> Self::Iter;
/// # }
/// # impl<I> IntoIterator for I where I: Iterator {
/// #     type Iter = I;
/// #     fn into_iter(self) -> I { self }
/// # }
/// # impl<'a, T> IntoIterator for &'a [T] {
/// #     type Iter = slice::Iter<'a, T>;
/// #     fn into_iter(self) -> slice::Iter<'a, T> { self.iter() }
/// # }
/// # impl<'a, T> IntoIterator for &'a mut Vec<T> {
/// #     type Iter = slice::IterMut<'a, T>;
/// #     fn into_iter(self) -> slice::IterMut<'a, T> { self.iter_mut() }
/// # }
/// # fn main() {
/// let chars = &['a', 'b', 'c'];
/// let mut v = vec![0, 1, 2];
///
/// for (&c, i, &mut j) in zip!(chars, 0i32..5, &mut v) {
///     assert!(i < 3);
///     assert_eq!(i, j);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! zip {
    ($a:expr, $b:expr) => {
        $crate::Zip2(($a).into_iter(), ($b).into_iter())
    };
    ($a:expr, $b:expr, $c:expr) => {
        $crate::Zip3(($a).into_iter(), ($b).into_iter(), ($c).into_iter())
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::Zip4(($a).into_iter(), ($b).into_iter(), ($c).into_iter(), ($d).into_iter())
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        $crate::Zip5(
            ($a).into_iter(),
            ($b).into_iter(),
            ($c).into_iter(),
            ($d).into_iter(),
            ($e).into_iter(),
        )
    };
    ($($x:expr),+,) => { zip!($($x),+) }
}

macro_rules! min {
    ($x:expr) => { $x };
    ($x:expr, $($y:expr),+) => { ::std::cmp::min($x, min!($($y),+)) };
    ($($x:expr),+,) => { min!($($x),+) };
}

/// Two-iterator zipper
pub struct Zip2<A, B>(pub A, pub B);

impl<A, B, AI, BI> Iterator for Zip2<AI, BI> where
    AI: Iterator<Item=A>,
    BI: Iterator<Item=B>,
{
    type Item = (A, B);

    fn next(&mut self) -> Option<(A, B)> {
        if let Some(a) = self.0.next() {
            if let Some(b) = self.1.next() {
                return Some((a, b));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (min!{
            self.0.size_hint().0,
            self.1.size_hint().0,
        }, None)
    }
}

/// Three-iterator zipper
pub struct Zip3<A, B, C>(pub A, pub B, pub C);

impl<A, B, C, AI, BI, CI> Iterator for Zip3<AI, BI, CI> where
    AI: Iterator<Item=A>,
    BI: Iterator<Item=B>,
    CI: Iterator<Item=C>,
{
    type Item = (A, B, C);

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

    fn size_hint(&self) -> (usize, Option<usize>) {
        (min!{
            self.0.size_hint().0,
            self.1.size_hint().0,
            self.2.size_hint().0,
        }, None)
    }
}

/// Four-iterator zipper
pub struct Zip4<A, B, C, D>(pub A, pub B, pub C, pub D);

impl<A, B, C, D, AI, BI, CI, DI> Iterator for Zip4<AI, BI, CI, DI> where
    AI: Iterator<Item=A>,
    BI: Iterator<Item=B>,
    CI: Iterator<Item=C>,
    DI: Iterator<Item=D>,
{
    type Item = (A, B, C, D);

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

    fn size_hint(&self) -> (usize, Option<usize>) {
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

impl<A, B, C, D, E, AI, BI, CI, DI, EI> Iterator for Zip5<AI, BI, CI, DI, EI> where
    AI: Iterator<Item=A>,
    BI: Iterator<Item=B>,
    CI: Iterator<Item=C>,
    DI: Iterator<Item=D>,
    EI: Iterator<Item=E>,
{
    type Item = (A, B, C, D, E);

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

    fn size_hint(&self) -> (usize, Option<usize>) {
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

    const SIZE: usize = 1024;

    #[bench]
    fn collect2(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let b = a;

        z.iter(|| {
            ::Zip2(a.iter(), b.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn collect2_(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let b = a;

        z.iter(|| {
            a.iter().zip(b.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn count2(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let b = a;

        z.iter(|| {
            ::Zip2(a.iter(), b.iter()).count()
        })
    }

    #[bench]
    fn count2_(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let b = a;

        z.iter(|| {
            a.iter().zip(b.iter()).count()
        })
    }

    #[bench]
    fn collect3(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c) = (a, a);

        z.iter(|| {
            ::Zip3(a.iter(), b.iter(), c.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn collect3_(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c) = (a, a);

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn count3(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c) = (a, a);

        z.iter(|| {
            ::Zip3(a.iter(), b.iter(), c.iter()).count()
        })
    }

    #[bench]
    fn count3_(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c) = (a, a);

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).count()
        })
    }

    #[bench]
    fn collect4(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c, d) = (a, a, a);

        z.iter(|| {
            ::Zip4(a.iter(), b.iter(), c.iter(), d.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn collect4_(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c, d) = (a, a, a);

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn count4(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c, d) = (a, a, a);

        z.iter(|| {
            ::Zip4(a.iter(), b.iter(), c.iter(), d.iter()).count()
        })
    }

    #[bench]
    fn count4_(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c, d) = (a, a, a);

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter()).count()
        })
    }

    #[bench]
    fn collect5(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c, d, e) = (a, a, a, a);

        z.iter(|| {
            ::Zip5(a.iter(), b.iter(), c.iter(), d.iter(), e.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn collect5_(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c, d, e) = (a, a, a, a);

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter()).zip(e.iter()).collect::<Vec<_>>()
        })
    }

    #[bench]
    fn count5(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c, d, e) = (a, a, a, a);

        z.iter(|| {
            ::Zip5(a.iter(), b.iter(), c.iter(), d.iter(), e.iter()).count()
        })
    }

    #[bench]
    fn count5_(z: &mut Bencher) {
        let a = [0u8; SIZE];
        let (b, c, d, e) = (a, a, a, a);

        z.iter(|| {
            a.iter().zip(b.iter()).zip(c.iter()).zip(d.iter()).zip(e.iter()).count()
        })
    }
}
