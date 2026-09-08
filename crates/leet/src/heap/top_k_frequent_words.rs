// lc 692 - Top K Frequent Words

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

pub struct Solution;

/// Custom wrapper for heap ordering: higher freq first, then lex order for ties.
#[derive(Eq, PartialEq)]
struct WordFreq {
    word: String,
    freq: usize,
}

impl Ord for WordFreq {
    fn cmp(&self, other: &Self) -> Ordering {
        // Primary: higher frequency wins
        // Secondary: lexicographically smaller word wins (reverse string comparison)
        self.freq
            .cmp(&other.freq)
            .then_with(|| other.word.cmp(&self.word))
    }
}

impl PartialOrd for WordFreq {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    /// BinaryHeap (max-heap) approach with custom ordering.
    /// Time O(n log n), Space O(n)
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let k = k as usize;
        let mut count: HashMap<String, usize> = HashMap::new();
        for word in &words { // O(n) count frequencies
            *count.entry(word.clone()).or_insert(0) += 1;
        }
        let mut heap: BinaryHeap<WordFreq> = BinaryHeap::new(); // max-heap by (freq, reverse-lex)
        for (word, freq) in count { // O(m log m) where m = unique words
            heap.push(WordFreq { word, freq });
        }
        let mut res = Vec::with_capacity(k);
        for _ in 0..k { // O(k log m) pop k times
            res.push(heap.pop().unwrap().word);
        }
        res
    }

    /// Bucket sort approach.
    /// Time O(n + m log m) where m = max bucket size, Space O(n)
    pub fn top_k_frequent_bucket(words: Vec<String>, k: i32) -> Vec<String> {
        let k = k as usize;
        let n = words.len();
        let mut count: HashMap<String, usize> = HashMap::new();
        for word in &words { // O(n) count frequencies
            *count.entry(word.clone()).or_insert(0) += 1;
        }
        let mut buckets: Vec<Vec<String>> = vec![vec![]; n + 1]; // index = frequency
        for (word, freq) in count { // O(m) distribute into buckets
            buckets[freq].push(word);
        }
        let mut res = Vec::with_capacity(k);
        for freq in (1..=n).rev() { // collect from highest freq bucket
            buckets[freq].sort(); // sort each bucket lexicographically
            for word in &buckets[freq] {
                res.push(word.clone());
                if res.len() == k {
                    return res;
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|&x| x.to_string()).collect()
    }

    #[test]
    fn test_heap_basic() {
        assert_eq!(
            Solution::top_k_frequent(s(&["i", "love", "leetcode", "i", "love", "coding"]), 2),
            s(&["i", "love"])
        );
    }

    #[test]
    fn test_heap_tie_breaking() {
        assert_eq!(
            Solution::top_k_frequent(
                s(&["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"]),
                4,
            ),
            s(&["the", "is", "sunny", "day"])
        );
    }

    #[test]
    fn test_heap_single_word() {
        assert_eq!(
            Solution::top_k_frequent(s(&["hello"]), 1),
            s(&["hello"])
        );
    }

    #[test]
    fn test_heap_all_same_freq() {
        // All words appear once; result should be lex order
        let res = Solution::top_k_frequent(s(&["b", "c", "a"]), 3);
        // all freq=1, so lex order
        assert_eq!(res, s(&["a", "b", "c"]));
    }

    #[test]
    fn test_heap_k_equals_unique() {
        assert_eq!(
            Solution::top_k_frequent(s(&["a", "a", "b", "b", "c"]), 3),
            s(&["a", "b", "c"])
        );
    }

    #[test]
    fn test_bucket_basic() {
        assert_eq!(
            Solution::top_k_frequent_bucket(s(&["i", "love", "leetcode", "i", "love", "coding"]), 2),
            s(&["i", "love"])
        );
    }

    #[test]
    fn test_bucket_tie_breaking() {
        assert_eq!(
            Solution::top_k_frequent_bucket(
                s(&["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"]),
                4,
            ),
            s(&["the", "is", "sunny", "day"])
        );
    }

    #[test]
    fn test_bucket_single_word() {
        assert_eq!(
            Solution::top_k_frequent_bucket(s(&["hello"]), 1),
            s(&["hello"])
        );
    }

    #[test]
    fn test_bucket_all_same_freq() {
        assert_eq!(
            Solution::top_k_frequent_bucket(s(&["b", "c", "a"]), 3),
            s(&["a", "b", "c"])
        );
    }

    #[test]
    fn test_bucket_k_equals_unique() {
        assert_eq!(
            Solution::top_k_frequent_bucket(s(&["a", "a", "b", "b", "c"]), 3),
            s(&["a", "b", "c"])
        );
    }
}
