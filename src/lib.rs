//! Extra functionality for Rust arrays.
#![cfg_attr(feature = "nightly", feature(generic_const_exprs, array_try_from_fn))]

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

    /// Resizes the array, filling new spaces at the end with the specified element.
    fn resize<const S: usize>(self, elem: T) -> [T; S]
    where
        T: Clone,
        Self: Sized;

    /// Resizes the array, filling new spaces at the end with the values generated from a function.
    fn resize_with<F, const S: usize>(self, f: F) -> [T; S]
    where
        F: FnMut(usize) -> T,
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
    fn resize<const S: usize>(self, elem: T) -> [T; S]
    where
        T: Clone,
    {
        self.resize_with(|_| elem.clone())
    }

    #[inline]
    fn resize_with<F, const S: usize>(self, mut f: F) -> [T; S]
    where
        F: FnMut(usize) -> T,
    {
        let mut a = self.into_iter();
        std::array::from_fn(|i| if i < N { a.next().unwrap() } else { f(i) })
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
        #[cfg(feature = "nightly")]
        {
            std::array::try_from_fn(|_| iter.next())
        }
        #[cfg(not(feature = "nightly"))]
        {
            let mut arr = std::array::from_fn(|_| None);
            for item in &mut arr {
                *item = Some(iter.next()?);
            }
            Some(arr.map(Option::unwrap))
        }
    }
}

/// Array with size information on the type.
pub trait ArrayN<T, const N: usize>: Array<T> {
    /// Merges elements with another array by calling a `FnMut(T, U) -> Output` closure for each pair.
    fn zip_with<U, Output, F>(self, other: [U; N], f: F) -> [Output; N]
    where
        F: FnMut(T, U) -> Output,
        Self: Sized;

    /// Merges elements with another two arrays by calling a `FnMut(T, U, V) -> Output` closure for each tuple.
    fn zip3_with<U, V, Output, F>(self, other1: [U; N], other2: [V; N], f: F) -> [Output; N]
    where
        F: FnMut(T, U, V) -> Output,
        Self: Sized;

    /// Merges elements with another three arrays by calling a `FnMut(T, U, V, W) -> Output` closure for each tuple.
    fn zip4_with<U, V, W, Output, F>(self, other1: [U; N], other2: [V; N], other3: [W; N], f: F) -> [Output; N]
    where
        F: FnMut(T, U, V, W) -> Output,
        Self: Sized;

    /// Merges elements with another four arrays by calling a `FnMut(T, U, V, W, X) -> Output` closure for each tuple.
    fn zip5_with<U, V, W, X, Output, F>(
        self, other1: [U; N], other2: [V; N], other3: [W; N], other4: [X; N], f: F,
    ) -> [Output; N]
    where
        F: FnMut(T, U, V, W, X) -> Output,
        Self: Sized;

    /// Converts this object into it's concrete array type.
    fn downcast(self) -> [T; N];

    /// Gets a reference to this object's concrete array type.
    fn downcast_ref(&self) -> &[T; N];

    /// Gets a mutable reference to this object's concrete array type.
    fn downcast_mut(&mut self) -> &mut [T; N];

    /// Concatenates two arrays together.
    #[cfg(feature = "nightly")]
    fn concat<const M: usize>(self, other: [T; M]) -> [T; N + M]
    where
        Self: Sized;

    /// Splits an array into two sub-arrays.
    #[cfg(feature = "nightly")]
    fn split<const P: usize>(self) -> ([T; P], [T; N - P])
    where
        Self: Sized;
}

impl<T, const N: usize> ArrayN<T, N> for [T; N] {
    #[inline]
    fn zip_with<U, Output, F>(self, other: [U; N], mut f: F) -> [Output; N]
    where
        F: FnMut(T, U) -> Output,
    {
        let mut b = other.into_iter();
        self.map(|a| f(a, b.next().unwrap()))
    }

    #[inline]
    fn zip3_with<U, V, Output, F>(self, other1: [U; N], other2: [V; N], mut f: F) -> [Output; N]
    where
        F: FnMut(T, U, V) -> Output,
    {
        let mut it1 = other1.into_iter();
        let mut it2 = other2.into_iter();
        self.map(|x0| f(x0, it1.next().unwrap(), it2.next().unwrap()))
    }

    #[inline]
    fn zip4_with<U, V, W, Output, F>(self, other1: [U; N], other2: [V; N], other3: [W; N], mut f: F) -> [Output; N]
    where
        F: FnMut(T, U, V, W) -> Output,
    {
        let mut it1 = other1.into_iter();
        let mut it2 = other2.into_iter();
        let mut it3 = other3.into_iter();
        self.map(|x0| f(x0, it1.next().unwrap(), it2.next().unwrap(), it3.next().unwrap()))
    }

    #[inline]
    fn zip5_with<U, V, W, X, Output, F>(
        self, other1: [U; N], other2: [V; N], other3: [W; N], other4: [X; N], mut f: F,
    ) -> [Output; N]
    where
        F: FnMut(T, U, V, W, X) -> Output,
    {
        let mut it1 = other1.into_iter();
        let mut it2 = other2.into_iter();
        let mut it3 = other3.into_iter();
        let mut it4 = other4.into_iter();
        self.map(|x0| {
            f(
                x0,
                it1.next().unwrap(),
                it2.next().unwrap(),
                it3.next().unwrap(),
                it4.next().unwrap(),
            )
        })
    }

    #[inline]
    fn downcast(self) -> [T; N] {
        self
    }

    #[inline]
    fn downcast_ref(&self) -> &[T; N] {
        self
    }

    #[inline]
    fn downcast_mut(&mut self) -> &mut [T; N] {
        self
    }

    #[cfg(feature = "nightly")]
    fn concat<const M: usize>(self, other: [T; M]) -> [T; N + M] {
        let mut a = self.into_iter();
        let mut b = other.into_iter();
        std::array::from_fn(|i| if i < N { a.next() } else { b.next() }.unwrap())
    }

    #[cfg(feature = "nightly")]
    fn split<const P: usize>(self) -> ([T; P], [T; N - P]) {
        let mut a = self.into_iter();
        let l = [(); P].map(|_| a.next().unwrap());
        let r = [(); N - P].map(|_| a.next().unwrap());
        (l, r)
    }
}

#[cfg(test)]
mod tests;
