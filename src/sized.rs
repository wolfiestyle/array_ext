//! Traits for individual Array sizes.
use seq_macro::seq;

macro_rules! impl_arrayn
{
    (@do_impl $name:ident , $n:expr , $($idx:expr)*) => {
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

    ($name:ident $n:expr , $($tname:ident $tn:expr ,)* ; $($accum:expr)*) => {
        impl_arrayn!(@do_impl $name, $n, $($accum)*);
        impl_arrayn!($($tname $tn,)* ; $($accum)* $n);
    };

    (; $($accum:expr)*) => ();
}

seq!(N in 0..=32 {
    impl_arrayn!{
        #(
            Array#N N,
        )*;
    }
});

