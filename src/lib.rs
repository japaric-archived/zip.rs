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
//! let chars = ['a', 'b', 'c'];
//! let mut v = vec![0, 1, 2];
//! for (&c, i, &mut j) in Zip3(chars.iter(), 0i32..5, v.iter_mut()) {
//!     assert!(i < 3);
//!     assert_eq!(i, j);
//! }
//! ```

#![cfg_attr(test, feature(test))]
#![deny(missing_docs, warnings)]

/// This macro emulates an "any-arity" free function that zips iterators
///
/// # Examples
///
/// ```
/// #[macro_use]
/// extern crate zip;
///
/// # fn main() {
/// let chars = ['a', 'b', 'c'];
/// let mut v = vec![0, 1, 2];
///
/// for (&c, i, &mut j) in zip!(&chars, 0..5, &mut v) {
///     assert!(i < 3);
///     assert_eq!(i, j);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! zip {
    ($a:expr, $b:expr) => {
        $crate::Zip2(
            ::std::iter::IntoIterator::into_iter($a),
            ::std::iter::IntoIterator::into_iter($b),
        )
    };
    ($a:expr, $b:expr, $c:expr) => {
        $crate::Zip3(
            ::std::iter::IntoIterator::into_iter($a),
            ::std::iter::IntoIterator::into_iter($b),
            ::std::iter::IntoIterator::into_iter($c),
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        $crate::Zip4(
            ::std::iter::IntoIterator::into_iter($a),
            ::std::iter::IntoIterator::into_iter($b),
            ::std::iter::IntoIterator::into_iter($c),
            ::std::iter::IntoIterator::into_iter($d),
        )
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        $crate::Zip5(
            ::std::iter::IntoIterator::into_iter($a),
            ::std::iter::IntoIterator::into_iter($b),
            ::std::iter::IntoIterator::into_iter($c),
            ::std::iter::IntoIterator::into_iter($d),
            ::std::iter::IntoIterator::into_iter($e),
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
pub struct Zip2<A, B>(pub A, pub B) where
    A: Iterator,
    B: Iterator;

impl<A, B> Iterator for Zip2<A, B> where
    A: Iterator,
    B: Iterator,
{
    type Item = (A::Item, B::Item);

    fn next(&mut self) -> Option<(A::Item, B::Item)> {
        if let Some(a) = self.0.next() {
            if let Some(b) = self.1.next() {
                return Some((a, b));
            }
        }

        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (min! {
            self.0.size_hint().0,
            self.1.size_hint().0,
        }, None)
    }
}

/// Three-iterator zipper
pub struct Zip3<A, B, C>(pub A, pub B, pub C) where
    A: Iterator,
    B: Iterator,
    C: Iterator;

impl<A, B, C> Iterator for Zip3<A, B, C> where
    A: Iterator,
    B: Iterator,
    C: Iterator,
{
    type Item = (A::Item, B::Item, C::Item);

    fn next(&mut self) -> Option<(A::Item, B::Item, C::Item)> {
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
pub struct Zip4<A, B, C, D>(pub A, pub B, pub C, pub D) where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator;

impl<A, B, C, D> Iterator for Zip4<A, B, C, D> where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator,
{
    type Item = (A::Item, B::Item, C::Item, D::Item);

    fn next(&mut self) -> Option<(A::Item, B::Item, C::Item, D::Item)> {
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
pub struct Zip5<A, B, C, D, E>(pub A, pub B, pub C, pub D, pub E) where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator,
    E: Iterator;

impl<A, B, C, D, E> Iterator for Zip5<A, B, C, D, E> where
    A: Iterator,
    B: Iterator,
    C: Iterator,
    D: Iterator,
    E: Iterator,
{
    type Item = (A::Item, B::Item, C::Item, D::Item, E::Item);

    fn next(&mut self) -> Option<(A::Item, B::Item, C::Item, D::Item, E::Item)> {
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
