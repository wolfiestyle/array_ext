//! Traits for individual Array sizes.

macro_rules! impl_arrayn
{
    ($name:ident, $n:expr, $($idx:expr)*) => {
        pub trait $name<T>: $crate::Array<T>
        {
            /// Takes a `FnMut(T) -> U` closure and creates a new array by calling that closure on each element.
            fn map<U, F>(self, f: F) -> [U; $n] where T: Copy, F: FnMut(T) -> U;
            /// Merges elements with another array by calling a `FnMut(T, U) -> V` closure for each pair.
            fn zip<U, V, F>(self, other: [U; $n], f: F) -> [V; $n] where T: Copy, U: Copy, F: FnMut(T, U) -> V;
        }

        #[allow(unused_variables, unused_mut)]
        impl<T> $name<T> for [T; $n]
        {
            fn map<U, F>(self, mut f: F) -> [U; $n] where T: Copy, F: FnMut(T) -> U
            {
                [$(f(self[$idx])),*]
            }

            fn zip<U, V, F>(self, other: [U; $n], mut f: F) -> [V; $n] where T: Copy, U: Copy, F: FnMut(T, U) -> V
            {
                [$(f(self[$idx], other[$idx])),*]
            }
        }
    };
}

// ugly, but can't create identifiers from macros.. yet
impl_arrayn!( Array0,  0,);
impl_arrayn!( Array1,  1, 0);
impl_arrayn!( Array2,  2, 0 1);
impl_arrayn!( Array3,  3, 0 1 2);
impl_arrayn!( Array4,  4, 0 1 2 3);
impl_arrayn!( Array5,  5, 0 1 2 3 4);
impl_arrayn!( Array6,  6, 0 1 2 3 4 5);
impl_arrayn!( Array7,  7, 0 1 2 3 4 5 6);
impl_arrayn!( Array8,  8, 0 1 2 3 4 5 6 7);
impl_arrayn!( Array9,  9, 0 1 2 3 4 5 6 7 8);
impl_arrayn!(Array10, 10, 0 1 2 3 4 5 6 7 8 9);
impl_arrayn!(Array11, 11, 0 1 2 3 4 5 6 7 8 9 10);
impl_arrayn!(Array12, 12, 0 1 2 3 4 5 6 7 8 9 10 11);
impl_arrayn!(Array13, 13, 0 1 2 3 4 5 6 7 8 9 10 11 12);
impl_arrayn!(Array14, 14, 0 1 2 3 4 5 6 7 8 9 10 11 12 13);
impl_arrayn!(Array15, 15, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14);
impl_arrayn!(Array16, 16, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15);
impl_arrayn!(Array17, 17, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16);
impl_arrayn!(Array18, 18, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17);
impl_arrayn!(Array19, 19, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18);
impl_arrayn!(Array20, 20, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19);
impl_arrayn!(Array21, 21, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20);
impl_arrayn!(Array22, 22, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21);
impl_arrayn!(Array23, 23, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22);
impl_arrayn!(Array24, 24, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23);
impl_arrayn!(Array25, 25, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24);
impl_arrayn!(Array26, 26, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25);
impl_arrayn!(Array27, 27, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26);
impl_arrayn!(Array28, 28, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27);
impl_arrayn!(Array29, 29, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28);
impl_arrayn!(Array30, 30, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29);
impl_arrayn!(Array31, 31, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30);
impl_arrayn!(Array32, 32, 0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31);
