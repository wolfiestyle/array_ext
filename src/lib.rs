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
/// assert_eq!(average([8.96, 3.14, 17.9]), 10.0);
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

    /// Returns an raw pointer to the array's buffer.
    fn as_ptr(&self) -> *const T;

    /// Returns an unsafe mutable pointer to the array's buffer.
    fn as_mut_ptr(&mut self) -> *mut T;

    /// Extracts a slice containing the entire array.
    fn as_slice(&self) -> &[T];

    /// Extracts a mutable slice of the entire array.
    fn as_mut_slice(&mut self) -> &mut [T];

    /// Takes a `FnMut(T) -> T` closure and creates a new array by calling that closure on each element.
    fn map_<F>(self, f: F) -> Self
    where
        T: Copy,
        F: FnMut(T) -> T,
        Self: Sized;

    /// Applies a function over the entire array, producing a single final value.
    fn foldl<A, F>(self, acc: A, f: F) -> A
    where
        T: Copy,
        F: FnMut(A, T) -> A,
        Self: Sized;

    /// Applies a function over the entire array (in reverse order), producing a single final value.
    fn foldr<A, F>(self, acc: A, f: F) -> A
    where
        T: Copy,
        F: FnMut(A, T) -> A,
        Self: Sized;

    /// Creates a new array using the provided closure.
    fn from_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> T,
        Self: Sized;

    /// Creates an array by extracting elements from the provided iterator.
    fn from_iter<I: Iterator<Item = T>>(iter: I) -> Option<Self>
    where
        Self: Sized;
}

// for arrays with 1+ elements
macro_rules! impl_array {
    (@replace $a:expr, $sub:expr) => ($sub);

    (@do_impl $count:tt $($idx:tt)+) => {
        impl<T> $crate::Array<T> for [T; $count] {
            fn len(&self) -> usize { $count }

            fn is_empty(&self) -> bool { false }

            fn first(&self) -> Option<&T> { Some(&self[0]) }

            fn first_mut(&mut self) -> Option<&mut T> { Some(&mut self[0]) }

            fn last(&self) -> Option<&T> { Some(&self[$count - 1]) }

            fn last_mut(&mut self) -> Option<&mut T> { Some(&mut self[$count - 1]) }

            fn get(&self, index: usize) -> Option<&T> {
                if index < $count { Some(&self[index]) } else { None }
            }

            fn get_mut(&mut self, index: usize) -> Option<&mut T> {
                if index < $count { Some(&mut self[index]) } else { None }
            }

            fn as_ptr(&self) -> *const T { &self[0] }

            fn as_mut_ptr(&mut self) -> *mut T { &mut self[0] }

            fn as_slice(&self) -> &[T] { self }

            fn as_mut_slice(&mut self) -> &mut [T] { self }

            fn map_<F>(self, mut f: F) -> Self
            where
                T: Copy, F: FnMut(T) -> T
            {
                [$( f(self[$count - $idx - 1]) ),+]
            }

            fn foldl<A, F>(self, mut acc: A, mut f: F) -> A
            where
                T: Copy, F: FnMut(A, T) -> A
            {
                $( acc = f(acc, self[$count - $idx - 1]); )+ acc
            }

            fn foldr<A, F>(self, mut acc: A, mut f: F) -> A
            where
                T: Copy, F: FnMut(A, T) -> A
            {
                $( acc = f(acc, self[$idx]); )+ acc
            }

            fn from_fn<F>(mut f: F) -> Self
            where
                F: FnMut(usize) -> T
            {
                [$( f($count - $idx - 1) ),+]
            }

            fn from_iter<I: Iterator<Item=T>>(mut iter: I) -> Option<Self> {
                Some([$( impl_array!(@replace $idx, match iter.next() { Some(v) => v, None => return None }) ),+])
            }
        }
    };

    ($count:tt $idx:tt) => {
        impl_array!(@do_impl $count $idx);
    };

    ($head:tt $($tail:tt)+) => {
        impl_array!(@do_impl $head $($tail)+);
        impl_array!($($tail)+);
    };
}

// implement sizes from 32 to 1
impl_array!(32 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0);

// special case for the empty array
impl<T> Array<T> for [T; 0] {
    fn len(&self) -> usize {
        0
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn first(&self) -> Option<&T> {
        None
    }

    fn first_mut(&mut self) -> Option<&mut T> {
        None
    }

    fn last(&self) -> Option<&T> {
        None
    }

    fn last_mut(&mut self) -> Option<&mut T> {
        None
    }

    fn get(&self, _index: usize) -> Option<&T> {
        None
    }

    fn get_mut(&mut self, _index: usize) -> Option<&mut T> {
        None
    }

    fn as_ptr(&self) -> *const T {
        self as _
    }

    fn as_mut_ptr(&mut self) -> *mut T {
        get_mut_ptr(self)
    }

    fn as_slice(&self) -> &[T] {
        self
    }

    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }

    fn map_<F>(self, _f: F) -> Self
    where
        T: Copy,
        F: FnMut(T) -> T,
    {
        self
    }

    fn foldl<A, F>(self, acc: A, _f: F) -> A
    where
        T: Copy,
        F: FnMut(A, T) -> A,
    {
        acc
    }

    fn foldr<A, F>(self, acc: A, _f: F) -> A
    where
        T: Copy,
        F: FnMut(A, T) -> A,
    {
        acc
    }

    fn from_fn<F>(_f: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        []
    }

    fn from_iter<I: Iterator<Item = T>>(_iter: I) -> Option<Self> {
        Some([])
    }
}

// workaround to not being able to cast `&mut [T; 0]` to `*mut T` directly
fn get_mut_ptr<T>(a: &mut [T; 0]) -> *mut T {
    a.as_mut_ptr()
}

pub mod sized;
pub use sized::*;

#[cfg(test)]
mod tests;
