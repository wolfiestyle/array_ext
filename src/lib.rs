//! Extra functionality for Rust arrays.

/// Generic array type.
///
/// This trait allows passing arrays by value in a generic way without turning them into slices,
/// so the functions get monomorphized for a specific size.
///
/// # Examples
/// ```
/// use array_ext::Array;
///
/// fn average<T: Array<f32>>(arr: T) -> f32
/// {
///     let n = arr.len() as f32;
///     arr.foldl(0.0, |acc, val| acc + val) / n
/// }
///
/// assert!((average([8.96, 3.14, 17.9]) - 10.0).abs() < f32::EPSILON);
/// ```
pub trait Array<T> {
    /// Returns the number of elements in the array.
    fn len(&self) -> usize;

    /// Returns true if the array has a length of 0
    fn is_empty(&self) -> bool;

    /// Returns the first element of the array, or `None` if it is empty.
    fn first(&self) -> Option<&T>;

    /// Returns a mutable pointer to the first element of the array, or `None` if it is empty.
    fn first_mut(&mut self) -> Option<&mut T>;

    /// Returns the last element of the array, or `None` if it is empty.
    fn last(&self) -> Option<&T>;

    /// Returns a mutable pointer to the last element of the array, or `None` if it is empty.
    fn last_mut(&mut self) -> Option<&mut T>;

    /// Returns the element of an array at the given index, or `None` if the index is out of bounds.
    fn get(&self, index: usize) -> Option<&T>;

    /// Returns a mutable reference to the element at the given index, or `None` if the index is out of bounds.
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Extracts a slice containing the entire array.
    fn as_slice(&self) -> &[T];

    /// Extracts a mutable slice of the entire array.
    fn as_mut_slice(&mut self) -> &mut [T];

    #[deprecated(since = "0.4.0", note = "use .map() instead")]
    /// Takes a `FnMut(T) -> T` closure and creates a new array by calling that closure on each element.
    fn map_<F>(self, f: F) -> Self
    where
        F: FnMut(T) -> T,
        Self: Sized;

    /// Applies a function over the entire array, producing a single final value.
    fn foldl<A, F>(self, acc: A, f: F) -> A
    where
        F: FnMut(A, T) -> A,
        Self: Sized;

    /// Applies a function over the entire array (in reverse order), producing a single final value.
    fn foldr<A, F>(self, acc: A, f: F) -> A
    where
        F: FnMut(A, T) -> A,
        Self: Sized;

    #[deprecated(since = "0.4.0", note = "use std::array::from_fn instead")]
    /// Creates a new array using the provided closure.
    fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> T,
        Self: Sized;

    /// Creates an array by extracting elements from the provided iterator.
    fn from_iter(iter: impl Iterator<Item = T>) -> Option<Self>
    where
        Self: Sized;
}

impl<T, const N: usize> Array<T> for [T; N] {
    #[inline]
    fn len(&self) -> usize {
        N
    }

    #[inline]
    fn is_empty(&self) -> bool {
        N == 0
    }

    #[inline]
    fn first(&self) -> Option<&T> {
        if N > 0 {
            Some(&self[0])
        } else {
            None
        }
    }

    #[inline]
    fn first_mut(&mut self) -> Option<&mut T> {
        if N > 0 {
            Some(&mut self[0])
        } else {
            None
        }
    }

    #[inline]
    fn last(&self) -> Option<&T> {
        if N > 0 {
            Some(&self[N - 1])
        } else {
            None
        }
    }

    #[inline]
    fn last_mut(&mut self) -> Option<&mut T> {
        if N > 0 {
            Some(&mut self[N - 1])
        } else {
            None
        }
    }

    #[inline]
    fn get(&self, index: usize) -> Option<&T> {
        if index < N {
            Some(&self[index])
        } else {
            None
        }
    }

    #[inline]
    fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < N {
            Some(&mut self[index])
        } else {
            None
        }
    }

    #[inline]
    fn as_slice(&self) -> &[T] {
        self
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    #[inline]
    fn map_<F>(self, f: F) -> Self
    where
        F: FnMut(T) -> T,
    {
        self.map(f)
    }

    #[inline]
    fn foldl<A, F>(self, mut acc: A, mut f: F) -> A
    where
        F: FnMut(A, T) -> A,
    {
        for val in self {
            acc = f(acc, val);
        }
        acc
    }

    #[inline]
    fn foldr<A, F>(self, mut acc: A, mut f: F) -> A
    where
        F: FnMut(A, T) -> A,
    {
        for val in self.into_iter().rev() {
            acc = f(acc, val);
        }
        acc
    }

    #[inline]
    fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        std::array::from_fn(f)
    }

    #[inline]
    fn from_iter(mut iter: impl Iterator<Item = T>) -> Option<Self> {
        let mut v = Vec::with_capacity(N); //FIXME: use try_map when it's stable
        for _ in 0..N {
            v.push(iter.next()?);
        }
        v.try_into().ok()
    }
}

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

#[cfg(test)]
mod tests;
