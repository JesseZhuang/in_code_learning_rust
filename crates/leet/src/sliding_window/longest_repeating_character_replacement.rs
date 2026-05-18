// LeetCode 424 — Longest Repeating Character Replacement

pub struct Solution;

impl Solution {
    /// Solution 1: Sliding window — O(n) time, O(1) space (26-char freq array).
    /// Key insight: we never shrink max_freq because the answer only improves
    /// when max_freq increases; stale max_freq just means the window won't grow.
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let mut freq = [0i32; 26]; // O(1) space — fixed 26 letters
        let mut max_freq = 0i32; // tracks highest frequency seen in any window position
        let mut left = 0usize;
        let mut result = 0i32;

        for right in 0..bytes.len() {
            // O(n) — right pointer advances once per iteration
            let idx = (bytes[right] - b'A') as usize;
            freq[idx] += 1;
            max_freq = max_freq.max(freq[idx]); // O(1) update of running max

            let window_len = (right - left + 1) as i32;
            // If replacements needed exceed k, shrink window from left
            if window_len - max_freq > k {
                // O(1) per shrink — left moves at most n times total
                let left_idx = (bytes[left] - b'A') as usize;
                freq[left_idx] -= 1;
                left += 1;
            }

            result = result.max((right - left + 1) as i32); // O(1) answer update
        }

        result
    }

    /// Solution 2: Binary search on answer length + sliding window check.
    /// O(n log n) time, O(1) space.
    pub fn character_replacement_binary_search(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n == 0 {
            return 0;
        }

        // Binary search: can we find a valid window of length `mid`?
        let mut lo = 1usize; // O(log n) iterations of binary search
        let mut hi = n;

        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2; // check if window of size `mid` is feasible
            if Self::can_make_window(bytes, k, mid) {
                lo = mid; // feasible — try larger
            } else {
                hi = mid - 1; // not feasible — try smaller
            }
        }

        lo as i32
    }

    /// Check if any window of exactly `win_len` has (win_len - max_freq) <= k.
    /// O(n) scan with fixed-size freq array.
    fn can_make_window(bytes: &[u8], k: i32, win_len: usize) -> bool {
        let mut freq = [0i32; 26]; // O(1) space
        let mut max_freq = 0i32;

        for i in 0..bytes.len() {
            // O(n) — slide the fixed-size window across the string
            let idx = (bytes[i] - b'A') as usize;
            freq[idx] += 1;
            max_freq = max_freq.max(freq[idx]);

            // Once window exceeds win_len, remove leftmost element
            if i >= win_len {
                let left_idx = (bytes[i - win_len] - b'A') as usize;
                freq[left_idx] -= 1;
                // Recompute max_freq after removal — O(26) = O(1)
                max_freq = *freq.iter().max().unwrap();
            }

            // Check if current window of size win_len is valid
            if i + 1 >= win_len && (win_len as i32 - max_freq) <= k {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
    }

    #[test]
    fn test_single_char() {
        assert_eq!(Solution::character_replacement("A".to_string(), 0), 1);
    }

    #[test]
    fn test_all_same() {
        assert_eq!(Solution::character_replacement("AAAA".to_string(), 2), 4);
    }

    #[test]
    fn test_no_replacement() {
        assert_eq!(Solution::character_replacement("AABBC".to_string(), 0), 2);
    }

    #[test]
    fn test_all_different() {
        assert_eq!(Solution::character_replacement("ABCDE".to_string(), 4), 5);
    }

    #[test]
    fn test_alternating() {
        assert_eq!(Solution::character_replacement("ABABAB".to_string(), 2), 5);
    }

    #[test]
    fn test_trailing_same() {
        assert_eq!(Solution::character_replacement("ABCAA".to_string(), 2), 5);
    }

    // Binary search solution tests
    #[test]
    fn test_binary_search_basic() {
        assert_eq!(Solution::character_replacement_binary_search("ABAB".to_string(), 2), 4);
        assert_eq!(Solution::character_replacement_binary_search("AABABBA".to_string(), 1), 4);
    }

    #[test]
    fn test_binary_search_single_char() {
        assert_eq!(Solution::character_replacement_binary_search("A".to_string(), 0), 1);
    }

    #[test]
    fn test_binary_search_all_same() {
        assert_eq!(Solution::character_replacement_binary_search("AAAA".to_string(), 2), 4);
    }

    #[test]
    fn test_binary_search_no_replacement() {
        assert_eq!(Solution::character_replacement_binary_search("AABBC".to_string(), 0), 2);
    }

    #[test]
    fn test_binary_search_all_different() {
        assert_eq!(Solution::character_replacement_binary_search("ABCDE".to_string(), 4), 5);
    }

    #[test]
    fn test_binary_search_alternating() {
        assert_eq!(Solution::character_replacement_binary_search("ABABAB".to_string(), 2), 5);
    }

    #[test]
    fn test_binary_search_trailing_same() {
        assert_eq!(Solution::character_replacement_binary_search("ABCAA".to_string(), 2), 5);
    }
}
