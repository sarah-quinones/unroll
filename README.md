`unroll`
======

An generic unroll function that takes a callable and calls it `N` times.

```toml
[dependencies]
unroll = "0.1"
```

```rust
use unroll::unroll;

fn main() {
    let arr = [1, 10, 100];
    let mut sum = 0;

    // Expands to:
    //
    //     sum += arr[0];
    //     sum += arr[1];
    //     sum += arr[2];
    //
    unroll::<3, _>(|i| sum += arr[i]);
    assert_eq!(sum, 111);
}
```
