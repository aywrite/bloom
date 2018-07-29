extern crate fnv;

use fnv::FnvHasher;
use std::hash::Hash;
use std::hash::Hasher;

struct BloomFilter {
    data: Vec<bool>,
    size: usize,
}

fn bloom_filter() -> BloomFilter {
    let size = 4;
    BloomFilter {
        data: vec![false; size],
        size: size,
    }
}

impl BloomFilter {
    pub fn insert<T: Hash>(&mut self, value: T) {
        let index = self.hash(value) % self.size;
        self.data[index] = true;
    }

    pub fn contains<T: Hash>(&mut self, value: T) -> bool {
        let index = self.hash(value) % self.size;
        println!("{}", index);
        self.data[index] == true
    }

    fn hash<T: Hash>(&mut self, value: T) -> usize {
        let mut hasher = FnvHasher::with_key(32);
        value.hash(&mut hasher);
        hasher.finish() as usize
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
