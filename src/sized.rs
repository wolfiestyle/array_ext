//! Traits for individual Array sizes.
use crate::Array;

/// Array with size information on the type.
pub trait ArrayN<T, const N: usize>: Array<T> {
    /// Merges elements with another array by calling a `FnMut(T, U) -> V` closure for each pair.
    fn zip_with<U, V, F>(self, other: [U; N], f: F) -> [V; N]
    where
        F: FnMut(T, U) -> V;
}

impl<T, const N: usize> ArrayN<T, N> for [T; N] {
    #[inline]
    fn zip_with<U, V, F>(self, other: [U; N], mut f: F) -> [V; N]
    where
        F: FnMut(T, U) -> V,
    {
        let mut b = other.into_iter();
        self.map(|a| f(a, b.next().unwrap()))
    }
}
