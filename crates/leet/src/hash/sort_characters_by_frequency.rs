use std::collections::HashMap;

/// leet 451
pub struct Solution;

impl Solution {
    /// HashMap + Sort: count frequencies, sort characters by frequency descending.
    /// Time O(n + k log k) where k = unique chars. Space O(n).
    pub fn frequency_sort(s: String) -> String {
        let mut count: HashMap<char, usize> = HashMap::new();
        for c in s.chars() { // O(n)
            *count.entry(c).or_default() += 1;
        }
        let mut chars: Vec<char> = count.keys().copied().collect();
        chars.sort_unstable_by(|a, b| count[b].cmp(&count[a])); // O(k log k)
        let mut result = String::with_capacity(s.len());
        for c in chars { // O(n) total
            for _ in 0..count[&c] {
                result.push(c);
            }
        }
        result
    }

    /// Bucket Sort: use frequency as bucket index, iterate from highest bucket.
    /// Time O(n), Space O(n).
    pub fn frequency_sort_bucket(s: String) -> String {
        let mut count: HashMap<char, usize> = HashMap::new();
        for c in s.chars() { // O(n)
            *count.entry(c).or_default() += 1;
        }
        let max_freq = *count.values().max().unwrap_or(&0); // O(k)
        let mut buckets: Vec<Vec<char>> = vec![vec![]; max_freq + 1]; // O(n)
        for (&c, &freq) in &count { // O(k)
            buckets[freq].push(c);
        }
        let mut result = String::with_capacity(s.len());
        for freq in (1..=max_freq).rev() { // O(n) total
            for &c in &buckets[freq] {
                for _ in 0..freq {
                    result.push(c);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_frequency_sorted(original: &str, result: &str) {
        let mut orig_count: HashMap<char, usize> = HashMap::new();
        for c in original.chars() { *orig_count.entry(c).or_default() += 1; }
        let mut res_count: HashMap<char, usize> = HashMap::new();
        for c in result.chars() { *res_count.entry(c).or_default() += 1; }
        assert_eq!(orig_count, res_count);
        let mut prev_freq = usize::MAX;
        let chars: Vec<char> = result.chars().collect();
        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            let mut run_len = 0;
            while i < chars.len() && chars[i] == c { run_len += 1; i += 1; }
            assert_eq!(orig_count[&c], run_len);
            assert!(run_len <= prev_freq);
            prev_freq = run_len;
        }
    }

    fn verify(s: &str) {
        assert_frequency_sorted(s, &Solution::frequency_sort(s.to_string()));
        assert_frequency_sorted(s, &Solution::frequency_sort_bucket(s.to_string()));
    }

    #[test] fn test_example1() { verify("tree"); }
    #[test] fn test_example2() { verify("cccaaa"); }
    #[test] fn test_example3() { verify("Aabb"); }
    #[test] fn test_single_char() { verify("z"); }
    #[test] fn test_all_same() { verify("aaaa"); }
    #[test] fn test_all_unique() { verify("abc"); }
    #[test] fn test_digits_and_letters() { verify("2a554442f544asfasssffffasss"); }
    #[test] fn test_case_sensitive() { verify("aAaA"); }
    #[test] fn test_two_chars() { verify("ab"); }
    #[test] fn test_long_repeated() { verify("bbbcccaaa"); }
}
