/// LeetCode 647, medium, tags: string, dynamic programming, two pointers.

pub struct Solution;

impl Solution {
    /// Expand around center. O(n^2) time, O(1) space.
    pub fn count_substrings(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let mut count = 0i32;
        for i in 0..n {
            // O(n) centers
            count += Self::expand(bytes, i as i32, i as i32);
            count += Self::expand(bytes, i as i32, (i + 1) as i32);
        }
        count
    }

    /// Each expand call is O(n) worst case
    fn expand(bytes: &[u8], mut l: i32, mut r: i32) -> i32 {
        let mut res = 0;
        let n = bytes.len() as i32;
        while l >= 0 && r < n && bytes[l as usize] == bytes[r as usize] {
            l -= 1;
            r += 1;
            res += 1;
        }
        res
    }

    /// Manacher's algorithm. O(n) time, O(n) space.
    pub fn count_substrings_manacher(s: String) -> i32 {
        let n = s.len();
        if n == 0 {
            return 0;
        }
        let bytes = s.as_bytes();
        let mut t: Vec<u8> = Vec::with_capacity(2 * n + 3);
        t.push(b'$');
        t.push(b'#');
        for &b in bytes {
            t.push(b);
            t.push(b'#');
        }
        t.push(b'@');

        let m = t.len() as i32;
        let mut p = vec![0i32; m as usize];
        let mut center: i32 = 0;
        let mut right: i32 = 0;

        for i in 1..m - 1 {
            let mirror = 2 * center - i;
            if right > i {
                p[i as usize] = p[mirror as usize].min(right - i);
            }
            while t[(i + p[i as usize] + 1) as usize] == t[(i - p[i as usize] - 1) as usize] {
                p[i as usize] += 1;
            }
            if i + p[i as usize] > right {
                center = i;
                right = i + p[i as usize];
            }
        }

        // Each p[i] = radius of longest palindrome centered at i in t
        // Number of palindromic substrings = sum of ceil(p[i]/2)
        let mut count = 0i32;
        for i in 1..(m - 1) as usize {
            count += (p[i] + 1) / 2;
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_expand() {
        assert_eq!(Solution::count_substrings("abc".to_string()), 3);
        assert_eq!(Solution::count_substrings("aaa".to_string()), 6);
        assert_eq!(Solution::count_substrings("a".to_string()), 1);
        assert_eq!(Solution::count_substrings("aa".to_string()), 3);
        assert_eq!(Solution::count_substrings("racecar".to_string()), 10);
        assert_eq!(Solution::count_substrings("aaaa".to_string()), 10);
        assert_eq!(Solution::count_substrings("abba".to_string()), 6);
    }

    #[test]
    fn test_manacher() {
        assert_eq!(Solution::count_substrings_manacher("abc".to_string()), 3);
        assert_eq!(Solution::count_substrings_manacher("aaa".to_string()), 6);
        assert_eq!(Solution::count_substrings_manacher("a".to_string()), 1);
        assert_eq!(Solution::count_substrings_manacher("aa".to_string()), 3);
        assert_eq!(Solution::count_substrings_manacher("racecar".to_string()), 10);
        assert_eq!(Solution::count_substrings_manacher("aaaa".to_string()), 10);
        assert_eq!(Solution::count_substrings_manacher("abba".to_string()), 6);
    }
}
