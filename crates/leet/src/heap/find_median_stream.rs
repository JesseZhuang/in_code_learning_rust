use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

/// Solution 1: Two Heaps
/// - max_heap (left half): stores the smaller half of numbers
/// - min_heap (right half): stores the larger half of numbers
/// Invariant: max_heap.len() == min_heap.len() or max_heap.len() == min_heap.len() + 1
pub struct MedianFinder {
    max_heap: BinaryHeap<i32>,           // left half (max-heap)
    min_heap: BinaryHeap<Reverse<i32>>,  // right half (min-heap)
}

impl MedianFinder {
    pub fn new() -> Self {
        Self {
            max_heap: BinaryHeap::new(),
            min_heap: BinaryHeap::new(),
        }
    }

    pub fn add_num(&mut self, num: i32) {
        self.max_heap.push(num);
        // Move max of left to right
        self.min_heap.push(Reverse(self.max_heap.pop().unwrap()));
        // Rebalance: left should be >= right in size
        if self.min_heap.len() > self.max_heap.len() {
            self.max_heap.push(self.min_heap.pop().unwrap().0);
        }
    }

    pub fn find_median(&self) -> f64 {
        if self.max_heap.len() > self.min_heap.len() {
            *self.max_heap.peek().unwrap() as f64
        } else {
            (*self.max_heap.peek().unwrap() as f64 + self.min_heap.peek().unwrap().0 as f64) / 2.0
        }
    }
}

/// Solution 2: BTreeMap-based approach
/// Maintains sorted counts and tracks median position.
pub struct MedianFinderBTree {
    map: BTreeMap<i32, usize>,
    len: usize,
}

impl MedianFinderBTree {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            len: 0,
        }
    }

    pub fn add_num(&mut self, num: i32) {
        *self.map.entry(num).or_insert(0) += 1;
        self.len += 1;
    }

    pub fn find_median(&self) -> f64 {
        let mid1 = (self.len - 1) / 2; // 0-indexed position of first middle
        let mid2 = self.len / 2;        // 0-indexed position of second middle

        let mut count = 0;
        let mut v1 = 0;
        let mut v2 = 0;
        for (&key, &cnt) in &self.map {
            count += cnt;
            if v1 == 0 && count > mid1 {
                v1 = key;
            }
            if count > mid2 {
                v2 = key;
                break;
            }
        }
        (v1 as f64 + v2 as f64) / 2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_example() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }

    #[test]
    fn test_single_element() {
        let mut mf = MedianFinder::new();
        mf.add_num(5);
        assert_eq!(mf.find_median(), 5.0);
    }

    #[test]
    fn test_negative_numbers() {
        let mut mf = MedianFinder::new();
        mf.add_num(-3);
        mf.add_num(-1);
        mf.add_num(-2);
        assert_eq!(mf.find_median(), -2.0);
    }

    #[test]
    fn test_duplicates() {
        let mut mf = MedianFinder::new();
        mf.add_num(1);
        mf.add_num(1);
        mf.add_num(1);
        assert_eq!(mf.find_median(), 1.0);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.0);
    }

    // BTreeMap solution tests
    #[test]
    fn test_btree_basic_example() {
        let mut mf = MedianFinderBTree::new();
        mf.add_num(1);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
    }

    #[test]
    fn test_btree_single_element() {
        let mut mf = MedianFinderBTree::new();
        mf.add_num(5);
        assert_eq!(mf.find_median(), 5.0);
    }

    #[test]
    fn test_btree_negative_numbers() {
        let mut mf = MedianFinderBTree::new();
        mf.add_num(-3);
        mf.add_num(-1);
        mf.add_num(-2);
        assert_eq!(mf.find_median(), -2.0);
    }

    #[test]
    fn test_btree_duplicates() {
        let mut mf = MedianFinderBTree::new();
        mf.add_num(1);
        mf.add_num(1);
        mf.add_num(1);
        assert_eq!(mf.find_median(), 1.0);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.0);
    }
}
