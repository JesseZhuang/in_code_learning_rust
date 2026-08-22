/// LeetCode 528 - Random Pick with Weight
///
/// Given an array `w` where `w[i]` is the weight of index `i`, implement
/// `pick_index()` which randomly picks an index in proportion to its weight.

use rand::Rng;

pub struct Solution;

/// Approach 1: Prefix sum + binary search.
/// O(n) space for prefix array; O(log n) per pick.
pub struct RandomPickWithWeight {
    prefix: Vec<i32>,
    total: i32,
}

impl RandomPickWithWeight {
    /// O(n) time, O(n) space
    pub fn new(w: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(w.len());
        let mut sum = 0;
        for &weight in &w {
            sum += weight;
            prefix.push(sum);
        }
        Self { prefix, total: sum }
    }

    /// O(log n) time — binary search for the target in prefix sums
    pub fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let target = rng.gen_range(1..=self.total);
        // Find the first index where prefix[i] >= target
        let mut lo = 0usize;
        let mut hi = self.prefix.len();
        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            if self.prefix[mid] < target {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo as i32
    }
}

/// Approach 2: Prefix sum + linear scan.
/// O(n) space; O(n) per pick.
pub struct RandomPickWithWeight2 {
    prefix: Vec<i32>,
    total: i32,
}

impl RandomPickWithWeight2 {
    /// O(n) time, O(n) space
    pub fn new(w: Vec<i32>) -> Self {
        let mut prefix = Vec::with_capacity(w.len());
        let mut sum = 0;
        for &weight in &w {
            sum += weight;
            prefix.push(sum);
        }
        Self { prefix, total: sum }
    }

    /// O(n) time — linear scan
    pub fn pick_index(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let target = rng.gen_range(1..=self.total);
        for (i, &p) in self.prefix.iter().enumerate() {
            if p >= target {
                return i as i32;
            }
        }
        (self.prefix.len() - 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_element() {
        let picker = RandomPickWithWeight::new(vec![5]);
        for _ in 0..100 {
            assert_eq!(picker.pick_index(), 0);
        }

        let picker2 = RandomPickWithWeight2::new(vec![5]);
        for _ in 0..100 {
            assert_eq!(picker2.pick_index(), 0);
        }
    }

    #[test]
    fn skewed_weights_binary_search() {
        let picker = RandomPickWithWeight::new(vec![1, 99]);
        let trials = 10_000;
        let mut counts = [0u32; 2];
        for _ in 0..trials {
            counts[picker.pick_index() as usize] += 1;
        }
        // Index 1 should be picked ~99% of the time (> 9000 out of 10000)
        assert!(
            counts[1] > 9000,
            "Expected index 1 picked > 9000 times, got {}",
            counts[1]
        );
    }

    #[test]
    fn skewed_weights_linear_scan() {
        let picker = RandomPickWithWeight2::new(vec![1, 99]);
        let trials = 10_000;
        let mut counts = [0u32; 2];
        for _ in 0..trials {
            counts[picker.pick_index() as usize] += 1;
        }
        assert!(
            counts[1] > 9000,
            "Expected index 1 picked > 9000 times, got {}",
            counts[1]
        );
    }

    #[test]
    fn valid_index_range() {
        let w = vec![3, 14, 1, 7];
        let picker = RandomPickWithWeight::new(w.clone());
        let picker2 = RandomPickWithWeight2::new(w);
        for _ in 0..1000 {
            let idx = picker.pick_index();
            assert!((0..4).contains(&idx), "Index out of range: {}", idx);
            let idx2 = picker2.pick_index();
            assert!((0..4).contains(&idx2), "Index out of range: {}", idx2);
        }
    }
}
