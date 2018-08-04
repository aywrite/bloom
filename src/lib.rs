//! A basic implementation of a bloom filter in Rust.
//!
//! This project is mostly just an excuse to play around in Rust. It does not aim to be an optimal
//! implementation of a bloom filter.
//!
//! Uses n hash function generated from two FNV hash function with different seeds. Based on the
//! paper "Less Hashing, Same Performance: Building a Better Bloom Filter".
//! # Examples
//!
//! ```
//! use bloom::BloomFilter;
//! let mut bf = BloomFilter::new();
//! assert_eq!(bf.contains(1), false);
//! bf.insert(2);
//! assert_eq!(bf.contains(2), true);
//! ```
extern crate fnv;

use fnv::FnvHasher;
use std::hash::Hash;
use std::hash::Hasher;

/// A bloom filter is a probabilistic structure for encoding membership somewhat similar to a set.
///
/// It is more space efficient than a conventional hash set at the cost of some percentage of false
/// positives when testing for membership. A bloom filter does not store the items in a which are
/// inserted, only their hash. Once inserted items cannot be removed.
///
/// Elements must implement the [`Hash`] trait. Since the bloom filter may produce a false positive
/// when testing for membership elements do not need to implement the [`Eq`] trait, unlike elements
/// for the standard library [`HashMap`] or [`HashSet`] types.
///
/// As with the standard library [`HashSet`] it is a logic error for an item to be modified in such
/// a way that the item's hash, as determined by the [`Hash`] trait changes while the bloom filter
/// is in scope.
///
/// # Examples
///
/// ```
/// use bloom::BloomFilter;
///
/// let mut books = BloomFilter::new();
///
/// // Add some books
/// books.insert("A Dance With Dragons");
/// books.insert("To Kill a Mockingbird");
/// books.insert("The Odyssey");
/// books.insert("The Great Gatsby");
///
/// // Check for a specific one.
/// // Unlike a conventional set this may return true or false. If contains returns false
/// // we can be sure that it isn't in the set.
/// if !books.contains("The Winds of Winter") {
///     println!("We have some books, but The Winds of Winter ain't one.");
/// }
///
/// ```
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct BloomFilter {
    data: Vec<bool>,
    size: usize,
    num_hashers: usize,
}

impl Default for BloomFilter {
    fn default() -> BloomFilter {
        let size = 100;
        BloomFilter {
            data: vec![false; size],
            size,
            num_hashers: 3,
        }
    }
}

//TODO implement:
// - is_empty
// - clear
// - with_size_and_num_hashers
// - use better hash seeds
// - support a vec of hashers?
// - input validation (e.g. don't create bloom filter of size 0)
// - dedicated bit vector type
impl BloomFilter {
    pub fn new() -> BloomFilter {
        let size = 100;
        BloomFilter {
            data: vec![false; size],
            size,
            num_hashers: 3,
        }
    }

    /// Creates an empty `BloomFilter` with the specified size.
    ///
    /// The bloom filter will be created with exactly the specified size, unlike other set
    /// datatypes it cannot be re-sized after allocation
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    /// let mut bf = BloomFilter::with_size(10);
    /// bf.insert(2);
    /// ```
    pub fn with_size(size: usize) -> BloomFilter {
        BloomFilter {
            data: vec![false; size],
            size,
            num_hashers: 3,
        }
    }

    /// Creates an empty `BloomFilter` with the specified number of hashers.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    /// let mut bf = BloomFilter::with_num_hashers(3);
    /// bf.insert(2);
    /// ```
    pub fn with_num_hashers(hashers: u32) -> BloomFilter {
        let size = 100;
        BloomFilter {
            data: vec![false; size],
            size,
            num_hashers: hashers as usize,
        }
    }

    /// Adds a value to the bloom filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    /// let mut bf = BloomFilter::new();
    /// bf.insert(2);
    /// assert_eq!(bf.contains(2), true);
    /// ```
    ///
    pub fn insert<T: Hash>(&mut self, value: T) {
        for i in 0..self.num_hashers {
            let hash = self.hash(&value, i + 1);
            self.data[hash] = true;
        }
    }

    /// Returns `false` if the bloom filter **definitely** does not contain a value.
    ///
    /// A return value of true indicates that the value **may** be in the filter.
    ///
    /// # Examples
    ///
    /// ```
    /// use bloom::BloomFilter;
    /// let mut bf = BloomFilter::new();
    /// assert_eq!(bf.contains(1), false);
    /// bf.insert(2);
    /// assert_eq!(bf.contains(2), true);
    /// bf.insert(3);
    /// assert_eq!(bf.contains(&3), true);
    /// ```
    ///
    pub fn contains<T: Hash>(&mut self, value: T) -> bool {
        for i in 0..self.num_hashers {
            let hash = self.hash(&value, i + 1);
            if !self.data[hash] {
                return false;
            }
        }
        return true;
    }

    fn hash<T: Hash>(&mut self, value: T, i: usize) -> usize {
        let mut hasher_a = FnvHasher::with_key(32);
        let mut hasher_b = FnvHasher::with_key(123456);
        value.hash(&mut hasher_a);
        let hash_a = hasher_a.finish() as usize;
        value.hash(&mut hasher_b);
        let hash_b = hasher_b.finish() as usize;
        let hashed = hash_b.wrapping_mul(i).wrapping_add(hash_a);
        (hashed % self.size) as usize
    }
}
