//! Reverse of LeetCode 38 follow-up. Tags: math, string, simulation.

pub struct Solution;

impl Solution {
    /// Expand RLE pairs (count, digit) back to the previous Count-and-Say term.
    /// Time O(n), Space O(n) for the output buffer.
    /// Returns Err on odd-length input or non-digit count characters.
    pub fn reverse_count_say(s: &str) -> Result<String, &'static str> {
        if s.is_empty() {
            return Ok(String::new());
        }
        let bytes = s.as_bytes();
        if bytes.len() % 2 != 0 {
            return Err("odd length input");
        }
        let mut out = String::with_capacity(bytes.len() * 4);
        let mut i = 0;
        while i < bytes.len() {
            let count_byte = bytes[i];
            if !count_byte.is_ascii_digit() {
                return Err("non-digit count");
            }
            let cnt = (count_byte - b'0') as usize;
            let digit_ch = bytes[i + 1] as char;
            for _ in 0..cnt {
                out.push(digit_ch);
            }
            i += 2;
        }
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    use crate::string::count_and_say::Solution as CAS;

    #[test]
    fn empty() {
        assert_eq!(Solution::reverse_count_say("").unwrap(), "");
    }

    #[test]
    fn example() {
        assert_eq!(Solution::reverse_count_say("111221").unwrap(), "1211");
    }

    #[test]
    fn single_pair() {
        assert_eq!(Solution::reverse_count_say("13").unwrap(), "3");
        assert_eq!(Solution::reverse_count_say("21").unwrap(), "11");
        assert_eq!(Solution::reverse_count_say("32").unwrap(), "222");
    }

    #[test]
    fn count_nine() {
        assert_eq!(Solution::reverse_count_say("97").unwrap(), "777777777");
    }

    #[test]
    fn zero_count() {
        assert_eq!(Solution::reverse_count_say("0415").unwrap(), "5");
    }

    #[test]
    fn multiple_pairs() {
        assert_eq!(Solution::reverse_count_say("112131").unwrap(), "111111");
    }

    #[test]
    fn round_trip_with_count_and_say() {
        for n in 2..=10 {
            let forward = CAS::count_and_say(n);
            let back = Solution::reverse_count_say(&forward).unwrap();
            assert_eq!(back, CAS::count_and_say(n - 1), "n={}", n);
        }
    }

    #[test]
    fn odd_length_errors() {
        assert!(Solution::reverse_count_say("123").is_err());
    }
}
