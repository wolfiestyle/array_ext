//! Traits for individual Array sizes.
use seq_macro::seq;

macro_rules! impl_arrayn {
    (@do_impl $name:ident , $n:expr , $($var1:ident $var2:ident $idx:expr)*) => {
        pub trait $name<T>: $crate::Array<T> {
            /// Takes a `FnMut(T) -> U` closure and creates a new array by calling that closure on each element.
            fn map<U, F>(self, f: F) -> [U; $n]
            where
                F: FnMut(T) -> U;

            /// Merges elements with another array by calling a `FnMut(T, U) -> V` closure for each pair.
            fn zip<U, V, F>(self, other: [U; $n], f: F) -> [V; $n]
            where
                F: FnMut(T, U) -> V;
        }

        #[allow(unused_variables, unused_mut)]
        impl<T> $name<T> for [T; $n] {
            #[inline]
            fn map<U, F>(self, mut f: F) -> [U; $n]
            where
                F: FnMut(T) -> U
            {
                let [$($var1),*] = self;
                [$(f($var1)),*]
            }

            #[inline]
            fn zip<U, V, F>(self, other: [U; $n], mut f: F) -> [V; $n]
            where
                F: FnMut(T, U) -> V
            {
                let [$($var1),*] = self;
                let [$($var2),*] = other;
                [$(f($var1, $var2)),*]
            }
        }
    };

    ($name:ident $var1:ident $var2:ident $n:expr , $($tname:ident $tvar1:ident $tvar2:ident $tn:expr ,)* ; $($avar1:ident $avar2:ident $an:expr)*) => {
        impl_arrayn!(@do_impl $name, $n, $($avar1 $avar2 $an)*);
        impl_arrayn!($($tname $tvar1 $tvar2 $tn,)* ; $($avar1 $avar2 $an)* $var1 $var2 $n);
    };

    (; $($avar1:ident $avar2:ident $an:expr)*) => ();
}

seq!(N in 0..=32 {
    impl_arrayn!{
        #(
            Array#N a#N b#N N,
        )*;
    }
});
