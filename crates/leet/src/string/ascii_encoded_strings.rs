//! HackerRank ASCII Encoded Strings (related to LeetCode 91, LintCode 512).
//!
//! Encoding: replace each character with its decimal ASCII value, then reverse
//! the whole digit string.
//!
//! Decoding: assume the alphabet is `[A-Za-z ]` (codes 32, 65..=90, 97..=122),
//! so in the reversed digit stream a leading digit '3'..'9' starts a 2-digit
//! code (range 32..=99) and '1' or '2' starts a 3-digit code (100..=126).

pub struct Solution;

impl Solution {
    pub fn encode(s: &str) -> String {
        let mut buf = String::new();
        for c in s.chars() {
            buf.push_str(&(c as u32).to_string());
        }
        buf.chars().rev().collect()
    }

    pub fn decode(encoded: &str) -> String {
        let reversed: Vec<char> = encoded.chars().rev().collect();
        let mut res = String::new();
        let mut i = 0;
        while i < reversed.len() {
            let take = if matches!(reversed[i], '3'..='9') { 2 } else { 3 };
            let chunk: String = reversed[i..i + take].iter().collect();
            let code: u32 = chunk.parse().unwrap();
            res.push(char::from_u32(code).unwrap());
            i += take;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    const CASES: &[(&str, &str)] = &[
        ("hello world", "00180141111191123111801801101401"),
        ("HackerRank",  "7010117928411101701997927"),
        ("Go VMWare",   "101411797877682311117"),
        ("Truth Always Wins ", "23511011501782351112179911801562340161171141148"),
    ];

    #[test]
    fn encode_cases() {
        for (plain, encoded) in CASES {
            assert_eq!(&Solution::encode(plain), encoded, "encode {}", plain);
        }
    }

    #[test]
    fn decode_cases() {
        for (plain, encoded) in CASES {
            assert_eq!(&Solution::decode(encoded), plain, "decode {}", encoded);
        }
    }

    #[test]
    fn round_trip() {
        for (plain, _) in CASES {
            assert_eq!(&Solution::decode(&Solution::encode(plain)), plain);
        }
    }

    #[test]
    fn single_char() {
        assert_eq!(Solution::encode("A"), "56");
        assert_eq!(Solution::decode("56"), "A");
    }

    #[test]
    fn lowercase_only() {
        assert_eq!(Solution::encode("abc"), "998979");
        assert_eq!(Solution::decode("998979"), "abc");
    }

    #[test]
    fn space_only() {
        assert_eq!(Solution::encode(" "), "23");
        assert_eq!(Solution::decode("23"), " ");
    }
}
