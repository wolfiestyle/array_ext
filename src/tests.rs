use super::*;
use std::fmt::Debug;
use std::ops::Add;

#[test]
fn methods_empty() {
    let mut arr: [i32; 0] = [];

    assert_eq!(arr.len(), 0);
    assert_eq!(arr.is_empty(), true);
    assert_eq!(arr.first(), None);
    assert_eq!(arr.first_mut(), None);
    assert_eq!(arr.last(), None);
    assert_eq!(arr.last_mut(), None);
    assert_eq!(arr.get(0), None);
    assert_eq!(arr.get_mut(0), None);
    assert_eq!(arr.map_(|a| a * 2), []);
    assert_eq!(arr.foldl(0, |a, n| a + n), 0);
    assert_eq!(arr.foldr(0, |a, n| a + n), 0);
}

fn test_arr<T: Array<V>, V: PartialEq + Debug>(mut arr: T, len: usize, mut first: V, mut last: V, mut second: V) {
    assert_eq!(arr.len(), len);
    assert_eq!(arr.is_empty(), false);
    assert_eq!(arr.first(), Some(&first));
    assert_eq!(arr.first_mut(), Some(&mut first));
    assert_eq!(arr.last(), Some(&last));
    assert_eq!(arr.last_mut(), Some(&mut last));
    assert_eq!(arr.get(1), Some(&second));
    assert_eq!(arr.get(42), None);
    assert_eq!(arr.get_mut(1), Some(&mut second));
    assert_eq!(arr.get_mut(42), None);
}

#[test]
fn methods() {
    let arr = [1, 2, 3, 4];
    test_arr(arr, 4, 1, 4, 2);
    assert_eq!(arr.map_(|a| a * 2), [2, 4, 6, 8]);
    assert_eq!(arr.foldl(0, |a, n| a + n), 10);

    let arr = ['c', 'd', 'e', 'f', 'g', 'a', 'b'];
    test_arr(arr, 7, 'c', 'b', 'd');
    assert_eq!(
        arr.map_(|a| a.to_uppercase().next().unwrap()),
        ['C', 'D', 'E', 'F', 'G', 'A', 'B']
    );
    assert_eq!(
        arr.foldl(String::new(), |mut a, c| {
            a.push(c);
            a
        }),
        "cdefgab"
    );
    assert_eq!(
        arr.foldr(String::new(), |mut a, c| {
            a.push(c);
            a
        }),
        "bagfedc"
    );
}

fn sum<T, V>(arr: T, val: V) -> T
where
    T: Array<V>,
    V: Add<Output = V> + Copy,
{
    arr.map_(|n| n + val)
}

fn avg<T: Array<f32>>(arr: T) -> f32 {
    let n = arr.len() as f32;
    arr.foldl(0.0, |a, n| a + n) / n
}

fn join_str<T: ArrayN<i32, N>, const N: usize>(arr: T) -> String {
    arr.downcast()
        .map(|n| n.to_string())
        .foldl(String::new(), |a, s| a + &s)
}

#[test]
fn generics() {
    assert_eq!(sum([], 1), []);
    assert_eq!(sum([1], 10), [11]);
    assert_eq!(sum([1, 2], 20), [21, 22]);
    assert_eq!(sum([1, 2, 3], 30), [31, 32, 33]);
    assert_eq!(sum([1, 2, 3, 4], 40), [41, 42, 43, 44]);

    assert_eq!(avg([3.5, 4.1, 7.0, 1.5]), 4.025);
    assert_eq!(avg([1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0]), 102.3);

    assert_eq!(join_str([1, 2, 42, 69]), "124269");
}

#[test]
fn slice() {
    assert_eq!([1, 2, 3].as_slice(), &[1, 2, 3]);
    assert_eq!([4, 5, 6].as_mut_slice(), &mut [4, 5, 6]);

    let mut arr = [1, -5, 8, 42, 33];
    arr.as_mut_slice().sort();
    assert_eq!(arr, [-5, 1, 8, 33, 42]);

    let mut v = Vec::new();
    v.extend_from_slice(arr.as_slice());
    assert_eq!(arr, &v[..]);
}

#[test]
fn constructors() {
    let arr: [usize; 5] = Array::from_iter(1..).unwrap();
    assert_eq!(arr, [1, 2, 3, 4, 5]);

    let iter = (1..).filter(|n| n % 2 == 0).zip("foobar".chars());
    let arr: [(i32, char); 6] = Array::from_iter(iter).unwrap();
    assert_eq!(arr, [(2, 'f'), (4, 'o'), (6, 'o'), (8, 'b'), (10, 'a'), (12, 'r')]);
}

#[test]
fn sized() {
    let arr = [1, 2, 3];
    assert_eq!(arr.map(|a| a as f32 / 2.0), [0.5, 1.0, 1.5]);
    assert_eq!(arr.zip_with([30, 20, 10], |a, b| a + b), [31, 22, 13]);
    assert_eq!(
        arr.zip_with(['a', 'b', 'c'], |a, b| (a, b)),
        [(1, 'a'), (2, 'b'), (3, 'c')]
    );

    let arr = ["foo", "asdf", "a", "very long string"];
    assert_eq!(arr.map(|s| s.len()), [3, 4, 1, 16]);
}

#[test]
fn object_safe() {
    let arr: Box<dyn Array<i32>> = Box::new([42, 69]);
    assert_eq!(arr.len(), 2);
    assert_eq!(arr.get(0), Some(&42));
    assert_eq!(arr.last(), Some(&69));

    let arrn: Box<dyn ArrayN<i32, 2>> = Box::new([42, 69]);
    assert_eq!(arrn.len(), 2);
    assert_eq!(arrn.get(0), Some(&42));
    assert_eq!(arrn.last(), Some(&69));
    assert_eq!(arrn.downcast_ref().map(|n| n + 1), [43, 70]);
}

#[test]
fn non_copy() {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Test(i32);

    let arr = [Test(1), Test(2), Test(3)];
    assert_eq!(arr.clone().map_(|a| Test(a.0 * 2)), [Test(2), Test(4), Test(6)]);
    assert_eq!(arr.clone().map(|a| a.0 * 2), [2, 4, 6]);
    assert_eq!(arr.clone().foldl(0, |a, n| a + n.0), 6);

    let arr2 = [Test(10), Test(20), Test(30)];
    assert_eq!(arr.clone().zip_with(arr2.clone(), |a, b| a.0 + b.0), [11, 22, 33]);

    let arr3 = [Test(100), Test(200), Test(300)];
    assert_eq!(
        arr.clone()
            .zip3_with(arr2.clone(), arr3.clone(), |a, b, c| a.0 + b.0 + c.0),
        [111, 222, 333]
    );

    let arr4 = [Test(1000), Test(2000), Test(3000)];
    assert_eq!(
        arr.clone()
            .zip4_with(arr2.clone(), arr3.clone(), arr4.clone(), |a, b, c, d| a.0
                + b.0
                + c.0
                + d.0),
        [1111, 2222, 3333]
    );

    let arr5 = [Test(10000), Test(20000), Test(30000)];
    assert_eq!(
        arr.zip5_with(arr2, arr3, arr4, arr5, |a, b, c, d, e| a.0 + b.0 + c.0 + d.0 + e.0),
        [11111, 22222, 33333]
    );
}

#[test]
fn resize() {
    let arr = [1, 2, 3];

    assert_eq!(arr.resize(42), [1, 2, 3, 42, 42]);
    assert_eq!(arr.resize(42), [1, 2]);
    assert_eq!(arr.resize_with(|i| i + 1), [1, 2, 3, 4, 5, 6]);
    assert_eq!(arr.resize::<0>(1), []);
    assert_eq!([].resize::<3>(0), [0; 3]);
}

#[cfg(feature = "nightly")]
#[test]
fn concat() {
    let a = [11, 22, 33];
    let b = [40, 50];

    assert_eq!(a.concat(b), [11, 22, 33, 40, 50]);
    assert_eq!(a.concat([]), a);
    assert_eq!([].concat(b), b);
}

#[cfg(feature = "nightly")]
#[test]
fn split() {
    let arr = [11, 22, 33, 40, 50];

    assert_eq!(arr.split(), ([11, 22, 33], [40, 50]));
    assert_eq!(arr.split::<0>(), ([], arr));
    assert_eq!(arr.split::<5>(), (arr, []));
}
