//! Traits for individual Array sizes.
use crate::Array;
use seq_macro::seq;

/// Array with size information on the type.
pub trait ArrayN<T, const N: usize>: Array<T> {
    /// Merges elements with another array by calling a `FnMut(T, U) -> V` closure for each pair.
    fn zip_with<U, V, F>(self, other: [U; N], f: F) -> [V; N]
    where
        F: FnMut(T, U) -> V;
}

macro_rules! impl_arrayn {
    (@do_impl $n:expr , $($var1:ident $var2:ident $idx:expr)*) => {
        #[allow(unused_variables, unused_mut)]
        impl<T> ArrayN<T, $n> for [T; $n] {
            #[inline]
            fn zip_with<U, V, F>(self, other: [U; $n], mut f: F) -> [V; $n]
            where
                F: FnMut(T, U) -> V
            {
                let [$($var1),*] = self;
                let [$($var2),*] = other;
                [$(f($var1, $var2)),*]
            }
        }
    };

    ($var1:ident $var2:ident $n:expr , $($tvar1:ident $tvar2:ident $tn:expr ,)* ; $($avar1:ident $avar2:ident $an:expr)*) => {
        impl_arrayn!(@do_impl $n, $($avar1 $avar2 $an)*);
        impl_arrayn!($($tvar1 $tvar2 $tn,)* ; $($avar1 $avar2 $an)* $var1 $var2 $n);
    };

    (; $($avar1:ident $avar2:ident $an:expr)*) => ();
}

seq!(N in 0..=32 {
    impl_arrayn!{
        #(
            a #N b #N N,
        )*;
    }
});
