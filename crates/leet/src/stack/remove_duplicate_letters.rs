pub struct Solution;

impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let bytes = s.as_bytes();
        let mut last_occurrence = [0usize; 26];
        for (i, &b) in bytes.iter().enumerate() {
            last_occurrence[(b - b'a') as usize] = i;
        }

        let mut in_stack = [false; 26];
        let mut stack: Vec<u8> = Vec::new();

        for (i, &b) in bytes.iter().enumerate() {
            let idx = (b - b'a') as usize;
            if in_stack[idx] {
                continue;
            }
            while let Some(&top) = stack.last() {
                if top > b && last_occurrence[(top - b'a') as usize] > i {
                    stack.pop();
                    in_stack[(top - b'a') as usize] = false;
                } else {
                    break;
                }
            }
            stack.push(b);
            in_stack[idx] = true;
        }

        String::from_utf8(stack).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(Solution::remove_duplicate_letters("bcabc".into()), "abc");
        assert_eq!(Solution::remove_duplicate_letters("cbacdcbc".into()), "acdb");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(Solution::remove_duplicate_letters("a".into()), "a");
        assert_eq!(Solution::remove_duplicate_letters("abc".into()), "abc");
        assert_eq!(Solution::remove_duplicate_letters("aaaa".into()), "a");
        assert_eq!(Solution::remove_duplicate_letters("bab".into()), "ab");
    }
}
