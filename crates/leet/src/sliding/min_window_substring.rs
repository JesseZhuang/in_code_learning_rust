/// leet 76, hard, tags: hash table, string, sliding window.

/// sliding window with count tracking. O(m+n) time, O(128) space.
impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let mut cnt = [0i32; 128];
        for &b in t.as_bytes() {
            cnt[b as usize] += 1;
        }
        let mut not_found = t.len();
        let (mut l, mut start, mut min_len) = (0, 0, usize::MAX);
        for r in 0..s.len() {
            let rc = s[r] as usize;
            if cnt[rc] > 0 {
                not_found -= 1;
            }
            cnt[rc] -= 1;
            while not_found == 0 {
                if r - l + 1 < min_len {
                    min_len = r - l + 1;
                    start = l;
                }
                let lc = s[l] as usize;
                cnt[lc] += 1;
                if cnt[lc] > 0 {
                    not_found += 1;
                }
                l += 1;
            }
        }
        if min_len == usize::MAX {
            String::new()
        } else {
            String::from_utf8_lossy(&s[start..start + min_len]).into_owned()
        }
    }
}

struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        let cases = vec![
            ("ADOBECODEBANC", "ABC", "BANC"),
            ("a", "a", "a"),
            ("a", "aa", ""),
            ("a", "b", ""),
            ("ab", "a", "a"),
            ("ab", "b", "b"),
            ("abc", "ac", "abc"),
            ("bdab", "ab", "ab"),
            ("aaflslflsldkalskaaa", "aaa", "aaa"),
            ("cabwefgewcwaefgcf", "cae", "cwae"),
        ];
        for (s, t, expected) in cases {
            assert_eq!(
                Solution::min_window(s.to_string(), t.to_string()),
                expected,
                "s={s}, t={t}"
            );
        }
    }
}
