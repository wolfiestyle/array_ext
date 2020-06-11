//! Extra functionality for Rust arrays.
use seq_macro::seq;

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

// for arrays with 1+ elements
macro_rules! impl_array {
    (@do_impl $count:expr , $($var:ident $idx:expr),+) => {
        impl<T> $crate::Array<T> for [T; $count] {
            #[inline]
            fn len(&self) -> usize { $count }

            #[inline]
            fn is_empty(&self) -> bool { false }

            #[inline]
            fn first(&self) -> Option<&T> { Some(&self[0]) }

            #[inline]
            fn first_mut(&mut self) -> Option<&mut T> { Some(&mut self[0]) }

            #[inline]
            fn last(&self) -> Option<&T> { Some(&self[$count - 1]) }

            #[inline]
            fn last_mut(&mut self) -> Option<&mut T> { Some(&mut self[$count - 1]) }

            #[inline]
            fn get(&self, index: usize) -> Option<&T> {
                if index < $count { Some(&self[index]) } else { None }
            }

            #[inline]
            fn get_mut(&mut self, index: usize) -> Option<&mut T> {
                if index < $count { Some(&mut self[index]) } else { None }
            }

            #[inline]
            fn as_ptr(&self) -> *const T { &self[0] }

            #[inline]
            fn as_mut_ptr(&mut self) -> *mut T { &mut self[0] }

            #[inline]
            fn as_slice(&self) -> &[T] { self }

            #[inline]
            fn as_mut_slice(&mut self) -> &mut [T] { self }

            #[inline]
            fn map_<F>(self, mut f: F) -> Self
            where
                F: FnMut(T) -> T
            {
                let [$($var),*] = self;
                [$( f($var) ),+]
            }

            #[inline]
            fn foldl<A, F>(self, mut acc: A, mut f: F) -> A
            where
                F: FnMut(A, T) -> A
            {
                let [$($var),*] = self;
                $( acc = f(acc, $var); )+
                acc
            }

            #[inline]
            fn foldr<A, F>(self, mut acc: A, mut f: F) -> A
            where
                F: FnMut(A, T) -> A
            {
                let [$($var),*] = self;
                impl_array!(@foldr acc, f; $($var),*;);
                acc
            }

            #[inline]
            fn from_fn<F>(mut f: F) -> Self
            where
                F: FnMut(usize) -> T
            {
                [$( f($idx) ),+]
            }

            #[inline]
            fn from_iter(mut iter: impl Iterator<Item = T>) -> Option<Self> {
                Some([$(impl_array!(@replace $idx, iter.next()?) ),+])
            }
        }
    };

    ($var:ident $idx:expr , $($tvar:ident $tidx:expr ,)* ; $($avar:ident $aidx:expr),*) => {
        impl_array!(@do_impl $idx, $($avar $aidx),*);
        impl_array!($($tvar $tidx,)* ; $($avar $aidx,)* $var $idx);
    };

    (; $($avar:ident $aidx:expr),*) => ();

    (@replace $a:expr, $sub:expr) => ($sub);

    (@foldr $acc:ident , $f:ident ; $head:expr $(, $tail:expr)* ; $($reversed:expr)*) => {
        impl_array!(@foldr $acc, $f; $($tail),*; $head $($reversed)*);
    };

    (@foldr $acc:ident , $f:ident ; ; $($reversed:expr)*) => {
        $($acc = $f($acc, $reversed);)*
    }
}

// implement sizes from 1 to 32
seq!(N in 1..=32 {
    impl_array!(#(a#N N,)* ; a0 0);
});

// special case for the empty array
impl<T> Array<T> for [T; 0] {
    #[inline]
    fn len(&self) -> usize {
        0
    }

    #[inline]
    fn is_empty(&self) -> bool {
        true
    }

    #[inline]
    fn first(&self) -> Option<&T> {
        None
    }

    #[inline]
    fn first_mut(&mut self) -> Option<&mut T> {
        None
    }

    #[inline]
    fn last(&self) -> Option<&T> {
        None
    }

    #[inline]
    fn last_mut(&mut self) -> Option<&mut T> {
        None
    }

    #[inline]
    fn get(&self, _index: usize) -> Option<&T> {
        None
    }

    #[inline]
    fn get_mut(&mut self, _index: usize) -> Option<&mut T> {
        None
    }

    #[inline]
    fn as_ptr(&self) -> *const T {
        self as _
    }

    #[inline]
    fn as_mut_ptr(&mut self) -> *mut T {
        get_mut_ptr(self)
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
    fn map_<F>(self, _f: F) -> Self
    where
        F: FnMut(T) -> T,
    {
        self
    }

    #[inline]
    fn foldl<A, F>(self, acc: A, _f: F) -> A
    where
        F: FnMut(A, T) -> A,
    {
        acc
    }

    #[inline]
    fn foldr<A, F>(self, acc: A, _f: F) -> A
    where
        F: FnMut(A, T) -> A,
    {
        acc
    }

    #[inline]
    fn from_fn<F>(_f: F) -> Self
    where
        F: FnMut(usize) -> T,
    {
        []
    }

    #[inline]
    fn from_iter(_iter: impl Iterator<Item = T>) -> Option<Self> {
        Some([])
    }
}

// workaround to not being able to cast `&mut [T; 0]` to `*mut T` directly
#[inline]
fn get_mut_ptr<T>(a: &mut [T; 0]) -> *mut T {
    a.as_mut_ptr()
}

pub mod sized;
pub use sized::*;

#[cfg(test)]
mod tests;
