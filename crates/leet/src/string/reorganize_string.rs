// todo lc767

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn reorganize_string(s: String) -> String {
        let (mut max_c, mut max_v) = (&0u8, 0);
        let mut counts = HashMap::new();
        for b in s.as_bytes() {
            counts.entry(b).and_modify(|v| *v += 1).or_insert(1);
            if counts[b] > max_v { (max_c, max_v) = (b, counts[b]); }
        }
        if max_v > (s.len() + 1) / 2 { return String::new(); }
        // Vec::with_capacity still 0 length until push
        let (mut i, mut res) = (0, vec![0u8; s.len()]);
        while counts[max_c] > 0 {
            res[i] = *max_c;
            i += 2;
            counts.entry(max_c).and_modify(|v| *v -= 1);
            // counts.insert(max_c, counts[max_c] - 1);
        }
        for (k, v) in counts.iter_mut() {
            while *v > 0 { // counts[k] > 0 not ok, mutable borrow then immutable
                if i > s.len() - 1 { i = 1; }
                res[i] = **k;
                i += 2;
                *v -= 1;
            }
        }
        String::from_utf8(res).expect("invalid utf8")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_valid(result: &str, s: &str) -> bool {
        let mut r: Vec<u8> = result.bytes().collect();
        let mut o: Vec<u8> = s.bytes().collect();
        r.sort();
        o.sort();
        if r != o { return false; }
        let rb = result.as_bytes();
        for i in 1..rb.len() {
            if rb[i] == rb[i - 1] { return false; }
        }
        true
    }

    #[test]
    fn test_possible() {
        for s in ["aab", "a", "ab", "aabb", "aaabb", "vvvlo"] {
            let res = Solution::reorganize_string(s.to_string());
            assert!(is_valid(&res, s), "Invalid for input: {}, got: {}", s, res);
        }
    }

    #[test]
    fn test_impossible() {
        for s in ["aaab", "aa", "aaaa"] {
            assert_eq!(Solution::reorganize_string(s.to_string()), "");
        }
    }
}