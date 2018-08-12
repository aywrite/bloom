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

#[test]
fn test_new_is_empty() {
    let mut bf = BloomFilter::new();
    assert_eq!(bf.is_empty(), true);
}

#[test]
fn test_not_is_empty_after_insert() {
    let mut bf = BloomFilter::new();
    bf.insert("foo");
    assert_eq!(bf.is_empty(), false);
}

#[test]
fn test_is_empty_after_clear() {
    let mut bf = BloomFilter::new();
    bf.insert("foo");
    bf.clear();
    assert_eq!(bf.is_empty(), true);
}
