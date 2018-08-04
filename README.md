# bloom

A basic implementation of a bloom filter in Rust.

This project is mostly just an excuse to play around in Rust. It does not aim to be an optimal
implementation of a bloom filter.

Uses n hash function generated from two FNV hash function with different seeds. Based on the
paper "Less Hashing, Same Performance: Building a Better Bloom Filter".
## Examples

```rust
use bloom::BloomFilter;
let mut bf = BloomFilter::new();
assert_eq!(bf.contains(1), false);
bf.insert(2);
assert_eq!(bf.contains(2), true);
```

## License

Licensed under the MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)


This readme was generated using [cargo-readme](https://github.com/livioribeiro/cargo-readme).
