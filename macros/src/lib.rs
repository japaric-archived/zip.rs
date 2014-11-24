#![deny(missing_docs, warnings)]
#![feature(macro_rules)]

//! The `zip!` macro

/// This macro emulates an "any-arity" free function that zips iterators
///
/// **Note** This macro calls `into_iter()` on the inputs. For this reason, this macro won't work
/// until `IntoIterator` lands in stdlib, or unless you provide your own `IntoIterator` trait.
///
/// # Examples
///
/// ```
/// # #![feature(phase)]
/// # extern crate zip;
/// # #[phase(plugin)]
/// # extern crate zip_macros;
/// # use std::slice;
/// # trait IntoIterator<T, I> where I: Iterator<T> {
/// #     fn into_iter(self) -> I;
/// # }
/// # impl<T, I> IntoIterator<T, I> for I where I: Iterator<T> {
/// #     fn into_iter(self) -> I { self }
/// # }
/// # impl<'a, T> IntoIterator<&'a T, slice::Items<'a, T>> for &'a [T] {
/// #     fn into_iter(self) -> slice::Items<'a, T> { self.iter() }
/// # }
/// # impl<'a, T> IntoIterator<&'a mut T, slice::MutItems<'a, T>> for &'a mut [T] {
/// #     fn into_iter(self) -> slice::MutItems<'a, T> { self.iter_mut() }
/// # }
/// # fn main() {
/// let chars = &['a', 'b', 'c'];
/// let mut v = vec![0u, 1, 2];
///
/// for (&c, i, &mut j) in zip!(chars, range(0u, 5), &mut *v) {
///     assert!(i < 3);
///     assert_eq!(i, j);
/// }
/// # }
/// ```
#[macro_export]
macro_rules! zip {
    ($a:expr, $b:expr) => {
        ::zip::Zip2(($a).into_iter(), ($b).into_iter())
    };
    ($a:expr, $b:expr, $c:expr) => {
        ::zip::Zip3(($a).into_iter(), ($b).into_iter(), ($c).into_iter())
    };
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ::zip::Zip4(($a).into_iter(), ($b).into_iter(), ($c).into_iter(), ($d).into_iter())
    };
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => {
        ::zip::Zip5(
            ($a).into_iter(),
            ($b).into_iter(),
            ($c).into_iter(),
            ($d).into_iter(),
            ($e).into_iter(),
        )
    };
    ($($x:expr),+,) => { zip!($($x),+) }
}
