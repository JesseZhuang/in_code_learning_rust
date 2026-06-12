use rand::Rng;
use std::collections::HashMap;

pub struct RandomizedSet {
    vals: Vec<i32>,
    val_index: HashMap<i32, usize>,
}

impl RandomizedSet {
    pub fn new() -> Self {
        Self {
            vals: Vec::new(),
            val_index: HashMap::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        if self.val_index.contains_key(&val) {
            return false;
        }
        self.val_index.insert(val, self.vals.len());
        self.vals.push(val);
        true
    }

    pub fn remove(&mut self, val: i32) -> bool {
        if let Some(&idx) = self.val_index.get(&val) {
            let last = *self.vals.last().unwrap();
            self.vals[idx] = last;
            self.val_index.insert(last, idx);
            self.vals.pop();
            self.val_index.remove(&val);
            true
        } else {
            false
        }
    }

    pub fn get_random(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..self.vals.len());
        self.vals[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(1));
        assert!(!set.remove(2));
        assert!(set.insert(2));
        let r = set.get_random();
        assert!(r == 1 || r == 2);
        assert!(set.remove(1));
        assert!(!set.insert(2));
        assert_eq!(set.get_random(), 2);
    }

    #[test]
    fn test_duplicates() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(5));
        assert!(!set.insert(5));
        assert_eq!(set.vals.len(), 1);
    }

    #[test]
    fn test_remove_nonexistent() {
        let mut set = RandomizedSet::new();
        assert!(!set.remove(99));
    }

    #[test]
    fn test_reinsert() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(10));
        assert!(set.remove(10));
        assert!(set.insert(10));
        assert_eq!(set.get_random(), 10);
    }

    #[test]
    fn test_single_element() {
        let mut set = RandomizedSet::new();
        assert!(set.insert(42));
        assert_eq!(set.get_random(), 42);
        assert!(set.remove(42));
        assert!(set.vals.is_empty());
    }

    #[test]
    fn test_get_random_distribution() {
        let mut set = RandomizedSet::new();
        set.insert(1);
        set.insert(2);
        set.insert(3);
        for _ in 0..100 {
            let r = set.get_random();
            assert!((1..=3).contains(&r));
        }
    }
}
