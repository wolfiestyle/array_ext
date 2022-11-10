# array_ext

Extra functionality for Rust arrays.

[Documentation](https://docs.rs/array_ext)

## Examples

The trait `Array` provides fixed-size array generics:

```rust
use array_ext::Array;

fn average<T: Array<f32>>(arr: T) -> f32
{
    let n = arr.len() as f32;
    arr.foldl(0.0, |acc, val| acc + val) / n
}

assert!((average([8.96, 3.14, 17.9]) - 10.0).abs() < f32::EPSILON);
```
Some methods, like `zip_with`, are provided by the sized `ArrayN` trait that allows doing full
`[T; N] -> [U; N]` mapping. The base `Array` trait can only do `[T; N] -> [T; N]` mapping.

This was originally made as workaround for the lack of const generics, but since v0.4 it's 
implemented using const generics.
