use super::Array;
use std::fmt::Debug;
use std::ops::Add;

fn test_empty<T: Array<i32>>(mut arr: T)
{
    assert_eq!(arr.len(), 0);
    assert_eq!(arr.is_empty(), true);
    assert_eq!(arr.first(), None);
    assert_eq!(arr.first_mut(), None);
    assert_eq!(arr.last(), None);
    assert_eq!(arr.last_mut(), None);
    assert_eq!(arr.get(1), None);
    assert_eq!(arr.get_mut(1), None);
}

#[test]
fn methods_empty()
{
    let arr: [i32; 0] = [];
    test_empty(arr);
    assert_eq!(arr.map(|a| a * 2), []);
    assert_eq!(arr.fold(0, |a, n| a + n), 0);
}

fn test_arr<T: Array<V>, V: PartialEq + Debug>(mut arr: T, len: usize, mut first: V, mut last: V, mut second: V)
{
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
fn methods()
{
    let arr = [1, 2, 3, 4];
    test_arr(arr, 4, 1, 4, 2);
    assert_eq!(arr.map(|a| a * 2), [2, 4, 6, 8]);
    assert_eq!(arr.fold(0, |a, n| a + n), 10);

    let arr = ['c', 'd', 'e', 'f', 'g', 'a', 'b'];
    test_arr(arr, 7, 'c', 'b', 'd');
    assert_eq!(arr.map(|a| a.to_uppercase().next().unwrap()), ['C', 'D', 'E', 'F', 'G', 'A', 'B']);
    assert_eq!(arr.fold(String::new(), |mut a, c| { a.push(c); a }), "cdefgab");
}

fn sum<T, V>(arr: T, val: V) -> T
    where T: Array<V>, V: Add<Output=V> + Copy
{
    arr.map(|n| n + val)
}

fn avg<T: Array<f32>>(arr: T) -> f32
{
    let n = arr.len() as f32;
    arr.fold(0.0, |a, n| a + n) / n
}

#[test]
fn generics()
{
    assert_eq!(sum([], 1), []);
    assert_eq!(sum([1], 10), [11]);
    assert_eq!(sum([1, 2], 20), [21, 22]);
    assert_eq!(sum([1, 2, 3], 30), [31, 32, 33]);
    assert_eq!(sum([1, 2, 3, 4], 40), [41, 42, 43, 44]);

    assert_eq!(avg([3.5, 4.1, 7.0, 1.5]), 4.025);
    assert_eq!(avg([1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0, 512.0]), 102.3);
}

#[test]
fn slice()
{
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
fn constructors()
{
    let arr: [usize; 5] = Array::from_fn(|i| i);
    assert_eq!(arr, [0, 1, 2, 3, 4]);

    let mut n = 1;
    let arr: [usize; 5] = Array::from_fn(|i| { n *= 2; i + n });
    assert_eq!(arr, [2, 5, 10, 19, 36]);
}