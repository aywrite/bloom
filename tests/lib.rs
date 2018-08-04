extern crate bloom;

use bloom::BloomFilter;

#[test]
fn test_empty_contains_is_false() {
    let mut bf = BloomFilter::new();
    assert_eq!(bf.contains("foo"), false);
    assert_eq!(bf.contains("bar"), false);
}

#[test]
fn test_insert_and_contains() {
    let mut bf = BloomFilter::new();
    bf.insert("foo");
    bf.insert("bar");
    assert_eq!(bf.contains("foo"), true);
    assert_eq!(bf.contains("bar"), true);
}
