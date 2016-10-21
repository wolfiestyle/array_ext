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

assert_eq!(average([8.96, 3.14, 17.9]), 10.0);
```

Currently implemented for [T; N] where N: 0 to 32. It's meant to be a workaround for the lack of integer generics.
