// leet code 394

pub struct Solution;

impl Solution {
    /// Stack-based approach. Time O(n * max_k), Space O(n).
    pub fn decode_string(s: String) -> String {
        let mut stack: Vec<(String, usize)> = Vec::new();
        let mut current = String::new();
        let mut k: usize = 0;

        for ch in s.chars() {
            match ch {
                '0'..='9' => {
                    k = k * 10 + (ch as usize - '0' as usize);
                }
                '[' => {
                    stack.push((current.clone(), k));
                    current.clear();
                    k = 0;
                }
                ']' => {
                    let (prev, repeat) = stack.pop().unwrap();
                    let repeated = current.repeat(repeat);
                    current = prev + &repeated;
                }
                _ => {
                    current.push(ch);
                }
            }
        }
        current
    }

    /// Recursive descent approach. Time O(n * max_k), Space O(n).
    pub fn decode_string_v2(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let mut i = 0;
        Self::decode_recursive(&chars, &mut i)
    }

    fn decode_recursive(chars: &[char], i: &mut usize) -> String {
        let mut result = String::new();
        while *i < chars.len() && chars[*i] != ']' {
            if chars[*i].is_ascii_digit() {
                let mut k: usize = 0;
                while *i < chars.len() && chars[*i].is_ascii_digit() {
                    k = k * 10 + (chars[*i] as usize - '0' as usize);
                    *i += 1;
                }
                // skip '['
                *i += 1;
                let inner = Self::decode_recursive(chars, i);
                // skip ']'
                *i += 1;
                for _ in 0..k {
                    result.push_str(&inner);
                }
            } else {
                result.push(chars[*i]);
                *i += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn run(input: &str, expected: &str) {
        assert_eq!(Solution::decode_string(input.to_string()), expected);
        assert_eq!(Solution::decode_string_v2(input.to_string()), expected);
    }

    #[test]
    fn test_basic() {
        run("3[a]2[bc]", "aaabcbc");
    }

    #[test]
    fn test_nested() {
        run("3[a2[c]]", "accaccacc");
    }

    #[test]
    fn test_multiple_groups() {
        run("2[abc]3[cd]ef", "abcabccdcdcdef");
    }

    #[test]
    fn test_plain() {
        run("abc", "abc");
    }

    #[test]
    fn test_single_repeat() {
        run("1[a]", "a");
    }

    #[test]
    fn test_double_digit() {
        run("10[a]", "aaaaaaaaaa");
    }

    #[test]
    fn test_deeply_nested() {
        run("2[a2[b3[c]]]", "abcccbcccabcccbccc");
    }

    #[test]
    fn test_empty() {
        run("", "");
    }

    #[test]
    fn test_adjacent_groups() {
        run("2[a]2[b]", "aabb");
    }
}
