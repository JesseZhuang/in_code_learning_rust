pub struct Solution;

impl Solution {
    /// Sliding window with count array — check all 26 entries for zero.
    /// O(n) time (each char visited twice; inner check is O(26) = O(1)), O(1) space.
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        if s.len() < p.len() {
            return vec![];
        }
        let mut count = [0i32; 26];
        // Build initial window of size p.len()
        for i in 0..p.len() {
            count[(p[i] - b'a') as usize] -= 1;
            count[(s[i] - b'a') as usize] += 1;
        }
        let mut result = Vec::new();
        if count.iter().all(|&c| c == 0) {
            result.push(0);
        }
        // Slide the window: O(n) iterations, each O(26) check
        for i in p.len()..s.len() {
            count[(s[i] - b'a') as usize] += 1;
            count[(s[i - p.len()] - b'a') as usize] -= 1;
            if count.iter().all(|&c| c == 0) {
                result.push((i - p.len() + 1) as i32);
            }
        }
        result
    }

    /// Sliding window tracking number of matched characters.
    /// O(n) time (each char visited twice, O(1) work per char), O(1) space.
    pub fn find_anagrams_v2(s: String, p: String) -> Vec<i32> {
        let (s, p) = (s.as_bytes(), p.as_bytes());
        if s.len() < p.len() {
            return vec![];
        }
        let mut count = [0i32; 26];
        for &b in p.iter() {
            count[(b - b'a') as usize] += 1;
        }
        let mut matches = 0usize; // number of characters with count == 0
        for i in 0..26 {
            if count[i] == 0 {
                matches += 1;
            }
        }
        let mut result = Vec::new();

        // Build initial window
        for i in 0..p.len() {
            let idx = (s[i] - b'a') as usize;
            count[idx] -= 1;
            if count[idx] == 0 {
                matches += 1;
            } else if count[idx] == -1 {
                matches -= 1;
            }
        }
        if matches == 26 {
            result.push(0);
        }

        // Slide: O(n) iterations, O(1) per iteration
        for i in p.len()..s.len() {
            // Add right character
            let idx = (s[i] - b'a') as usize;
            count[idx] -= 1;
            if count[idx] == 0 {
                matches += 1;
            } else if count[idx] == -1 {
                matches -= 1;
            }
            // Remove left character
            let idx = (s[i - p.len()] - b'a') as usize;
            count[idx] += 1;
            if count[idx] == 0 {
                matches += 1;
            } else if count[idx] == 1 {
                matches -= 1;
            }
            if matches == 26 {
                result.push((i - p.len() + 1) as i32);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run_both(s: &str, p: &str) -> (Vec<i32>, Vec<i32>) {
        (
            Solution::find_anagrams(s.to_string(), p.to_string()),
            Solution::find_anagrams_v2(s.to_string(), p.to_string()),
        )
    }

    #[test]
    fn test_example1() {
        let (v1, v2) = run_both("cbaebabacd", "abc");
        assert_eq!(v1, vec![0, 6]);
        assert_eq!(v2, vec![0, 6]);
    }

    #[test]
    fn test_example2() {
        let (v1, v2) = run_both("abab", "ab");
        assert_eq!(v1, vec![0, 1, 2]);
        assert_eq!(v2, vec![0, 1, 2]);
    }

    #[test]
    fn test_no_match() {
        let (v1, v2) = run_both("abcdef", "xyz");
        assert_eq!(v1, Vec::<i32>::new());
        assert_eq!(v2, Vec::<i32>::new());
    }

    #[test]
    fn test_p_longer_than_s() {
        let (v1, v2) = run_both("ab", "abcd");
        assert_eq!(v1, Vec::<i32>::new());
        assert_eq!(v2, Vec::<i32>::new());
    }

    #[test]
    fn test_single_char_match() {
        let (v1, v2) = run_both("abcba", "a");
        assert_eq!(v1, vec![0, 4]);
        assert_eq!(v2, vec![0, 4]);
    }

    #[test]
    fn test_entire_string_anagram() {
        let (v1, v2) = run_both("bca", "abc");
        assert_eq!(v1, vec![0]);
        assert_eq!(v2, vec![0]);
    }

    #[test]
    fn test_repeated_pattern() {
        let (v1, v2) = run_both("aababaab", "aab");
        assert_eq!(v1, vec![0, 1, 3, 4, 5]);
        assert_eq!(v2, vec![0, 1, 3, 4, 5]);
    }

    #[test]
    fn test_empty_s() {
        let (v1, v2) = run_both("", "a");
        assert_eq!(v1, Vec::<i32>::new());
        assert_eq!(v2, Vec::<i32>::new());
    }
}
