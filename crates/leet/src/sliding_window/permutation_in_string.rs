pub struct Solution;

impl Solution {
    /// Sliding window with match count — O(l2) time, O(1) space.
    /// Track how many of the 26 character positions have matching frequency
    /// between s1 and the current window in s2.
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        if s1.len() > s2.len() {
            return false;
        }
        let mut count = [0i32; 26];
        for &b in s1.iter() {
            count[(b - b'a') as usize] += 1;
        }

        // matches = number of indices where count[i] == 0
        let mut matches = 0usize;
        for i in 0..26 {
            if count[i] == 0 {
                matches += 1;
            }
        }

        // Build initial window of size s1.len()
        for i in 0..s1.len() {
            let idx = (s2[i] - b'a') as usize;
            count[idx] -= 1;
            if count[idx] == 0 {
                matches += 1;
            } else if count[idx] == -1 {
                matches -= 1;
            }
        }
        if matches == 26 {
            return true;
        }

        // Slide window
        for i in s1.len()..s2.len() {
            // Add right character
            let idx = (s2[i] - b'a') as usize;
            count[idx] -= 1;
            if count[idx] == 0 {
                matches += 1;
            } else if count[idx] == -1 {
                matches -= 1;
            }
            // Remove left character
            let idx = (s2[i - s1.len()] - b'a') as usize;
            count[idx] += 1;
            if count[idx] == 0 {
                matches += 1;
            } else if count[idx] == 1 {
                matches -= 1;
            }
            if matches == 26 {
                return true;
            }
        }
        false
    }

    /// Sorted comparison approach — O((l2 - l1) * l1 * log(l1)) time, O(l1) space.
    /// For each window of size l1 in s2, sort it and compare with sorted s1.
    pub fn check_inclusion_sorted(s1: String, s2: String) -> bool {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        if s1.len() > s2.len() {
            return false;
        }
        let mut sorted_s1 = s1.to_vec();
        sorted_s1.sort_unstable();

        for i in 0..=(s2.len() - s1.len()) {
            let mut window = s2[i..i + s1.len()].to_vec();
            window.sort_unstable();
            if window == sorted_s1 {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn check_both(s1: &str, s2: &str) -> (bool, bool) {
        (
            Solution::check_inclusion(s1.to_string(), s2.to_string()),
            Solution::check_inclusion_sorted(s1.to_string(), s2.to_string()),
        )
    }

    #[test]
    fn test_basic_true() {
        let (v1, v2) = check_both("ab", "eidbaooo");
        assert!(v1);
        assert!(v2);
    }

    #[test]
    fn test_basic_false() {
        let (v1, v2) = check_both("ab", "eidboaoo");
        assert!(!v1);
        assert!(!v2);
    }

    #[test]
    fn test_s1_longer_than_s2() {
        let (v1, v2) = check_both("abcdef", "abc");
        assert!(!v1);
        assert!(!v2);
    }

    #[test]
    fn test_exact_match() {
        let (v1, v2) = check_both("abc", "cba");
        assert!(v1);
        assert!(v2);
    }

    #[test]
    fn test_single_char() {
        let (v1, v2) = check_both("a", "a");
        assert!(v1);
        assert!(v2);

        let (v1, v2) = check_both("a", "b");
        assert!(!v1);
        assert!(!v2);
    }

    #[test]
    fn test_permutation_at_end() {
        let (v1, v2) = check_both("abc", "xyzacb");
        assert!(v1);
        assert!(v2);
    }

    #[test]
    fn test_all_same_chars() {
        let (v1, v2) = check_both("aaa", "aaaaaaa");
        assert!(v1);
        assert!(v2);

        let (v1, v2) = check_both("aaa", "aab");
        assert!(!v1);
        assert!(!v2);
    }
}
