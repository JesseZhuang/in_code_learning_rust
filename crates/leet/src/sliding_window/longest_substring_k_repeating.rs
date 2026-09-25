// LeetCode 395 — Longest Substring with At Least K Repeating Characters

pub struct Solution;

impl Solution {
    /// Solution 1: Divide and Conquer.
    /// Count char frequencies. Find any char with freq < k. Split string on that
    /// char, recursively solve each part. If all chars meet threshold, return string length.
    /// Time O(26n) = O(n), Space O(26) = O(1).
    pub fn longest_substring(s: String, k: i32) -> i32 {
        Self::dc(s.as_bytes(), k as usize)
    }

    fn dc(s: &[u8], k: usize) -> i32 {
        if s.is_empty() {
            return 0;
        }

        // Count frequencies — O(n) scan, O(26) space
        let mut freq = [0usize; 26];
        for &b in s {
            freq[(b - b'a') as usize] += 1;
        }

        // Find a char that appears but fewer than k times — O(26) scan
        if let Some(&split_char) = s.iter().find(|&&b| {
            let f = freq[(b - b'a') as usize];
            f > 0 && f < k
        }) {
            // Split on that char and recurse on each segment
            s.split(|&b| b == split_char)
                .map(|part| Self::dc(part, k))
                .max()
                .unwrap_or(0)
        } else {
            // All chars in s appear >= k times
            s.len() as i32
        }
    }

    /// Solution 2: Sliding Window with unique count enumeration.
    /// For each target unique count 1..=total_unique, use two pointers.
    /// Expand when unique <= target, shrink otherwise. Track at_least_k count.
    /// Time O(26n) = O(n), Space O(26) = O(1).
    pub fn longest_substring2(s: String, k: i32) -> i32 {
        let bytes = s.as_bytes();
        if bytes.is_empty() {
            return 0;
        }
        let k = k as usize;

        // Count total unique chars — O(n) scan
        let mut seen = [false; 26];
        for &b in bytes {
            seen[(b - b'a') as usize] = true;
        }
        let total_unique = seen.iter().filter(|&&v| v).count();

        let mut result = 0;

        // Enumerate target unique count — 26 iterations max
        for target in 1..=total_unique {
            let mut freq = [0usize; 26];
            let mut left = 0usize;
            let mut unique = 0usize; // distinct chars in window
            let mut at_least_k = 0usize; // chars with freq >= k in window

            for right in 0..bytes.len() {
                // Expand: add bytes[right]
                let ri = (bytes[right] - b'a') as usize;
                if freq[ri] == 0 {
                    unique += 1;
                }
                freq[ri] += 1;
                if freq[ri] == k {
                    at_least_k += 1;
                }

                // Shrink while too many unique chars
                while unique > target {
                    let li = (bytes[left] - b'a') as usize;
                    if freq[li] == k {
                        at_least_k -= 1;
                    }
                    freq[li] -= 1;
                    if freq[li] == 0 {
                        unique -= 1;
                    }
                    left += 1;
                }

                // All unique chars in window appear >= k times
                if unique == target && at_least_k == target {
                    result = result.max(right - left + 1);
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dc() {
        assert_eq!(Solution::longest_substring("aaabb".to_string(), 3), 3);
        assert_eq!(Solution::longest_substring("ababbc".to_string(), 2), 5);
        assert_eq!(Solution::longest_substring("a".to_string(), 1), 1);
        assert_eq!(Solution::longest_substring("abc".to_string(), 4), 0);
        assert_eq!(Solution::longest_substring("aaaaa".to_string(), 2), 5);
        assert_eq!(Solution::longest_substring("abcdef".to_string(), 2), 0);
        assert_eq!(Solution::longest_substring("aabbcc".to_string(), 2), 6);
        assert_eq!(Solution::longest_substring("aaabbbdcccc".to_string(), 2), 6);
        assert_eq!(Solution::longest_substring("abcdefg".to_string(), 1), 7);
        assert_eq!(Solution::longest_substring("cbcbbaaa".to_string(), 3), 3);
    }

    #[test]
    fn test_sliding_window() {
        assert_eq!(Solution::longest_substring2("aaabb".to_string(), 3), 3);
        assert_eq!(Solution::longest_substring2("ababbc".to_string(), 2), 5);
        assert_eq!(Solution::longest_substring2("a".to_string(), 1), 1);
        assert_eq!(Solution::longest_substring2("abc".to_string(), 4), 0);
        assert_eq!(Solution::longest_substring2("aaaaa".to_string(), 2), 5);
        assert_eq!(Solution::longest_substring2("abcdef".to_string(), 2), 0);
        assert_eq!(Solution::longest_substring2("aabbcc".to_string(), 2), 6);
        assert_eq!(Solution::longest_substring2("aaabbbdcccc".to_string(), 2), 6);
        assert_eq!(Solution::longest_substring2("abcdefg".to_string(), 1), 7);
        assert_eq!(Solution::longest_substring2("cbcbbaaa".to_string(), 3), 3);
    }
}
