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

Currently implemented for array sizes from 0 to 32, hardcoded as workaround for the lack of integer generics.

The `map` and `zip` methods are provided by the sized `Array1, Array2, ...` types. The base `Array` type only
provides a `map_` method that can do `T -> T` mapping.
