/// leet 5

pub struct Solution;

impl Solution {
    /// Expand from center. O(n^2) time, O(1) space.
    pub fn longest_palindrome(s: String) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        if n == 0 {
            return String::new();
        }
        let mut start = 0usize;
        let mut max_len = 1usize;

        let expand = |mut l: i32, mut r: i32| -> (usize, usize) {
            while l >= 0 && (r as usize) < n && bytes[l as usize] == bytes[r as usize] {
                l -= 1;
                r += 1;
            }
            ((l + 1) as usize, (r - l - 1) as usize)
        };

        for i in 0..n {
            // O(n) iterations, each expand O(n) worst case
            let (s1, l1) = expand(i as i32, i as i32);
            let (s2, l2) = expand(i as i32, (i + 1) as i32);
            if l1 > max_len {
                start = s1;
                max_len = l1;
            }
            if l2 > max_len {
                start = s2;
                max_len = l2;
            }
        }
        s[start..start + max_len].to_string()
    }

    /// Manacher's algorithm. O(n) time, O(n) space.
    pub fn longest_palindrome_manacher(s: String) -> String {
        let n = s.len();
        if n == 0 {
            return String::new();
        }
        let bytes = s.as_bytes();
        // Use i32 to avoid unsigned underflow
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

        let mut max_len = 0i32;
        let mut max_center = 0i32;
        for i in 1..m - 1 {
            if p[i as usize] > max_len {
                max_len = p[i as usize];
                max_center = i;
            }
        }

        let orig_start = ((max_center - 1 - max_len) / 2) as usize;
        s[orig_start..orig_start + max_len as usize].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_expand() {
        let res = Solution::longest_palindrome("babad".to_string());
        assert!(res == "bab" || res == "aba");
        assert_eq!(Solution::longest_palindrome("cbbd".to_string()), "bb");
        assert_eq!(Solution::longest_palindrome("a".to_string()), "a");
        assert_eq!(Solution::longest_palindrome("aa".to_string()), "aa");
        assert_eq!(Solution::longest_palindrome("racecar".to_string()), "racecar");
        assert_eq!(Solution::longest_palindrome("xabcbay".to_string()), "abcba");
    }

    #[test]
    fn test_manacher() {
        let res = Solution::longest_palindrome_manacher("babad".to_string());
        assert!(res == "bab" || res == "aba");
        assert_eq!(Solution::longest_palindrome_manacher("cbbd".to_string()), "bb");
        assert_eq!(Solution::longest_palindrome_manacher("a".to_string()), "a");
        assert_eq!(Solution::longest_palindrome_manacher("aa".to_string()), "aa");
        assert_eq!(Solution::longest_palindrome_manacher("racecar".to_string()), "racecar");
        assert_eq!(Solution::longest_palindrome_manacher("xabcbay".to_string()), "abcba");
    }
}
