pub struct Solution;

impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut stack = Vec::new();
        let mut next_popped = 0;

        for value in pushed {
            stack.push(value);
            // O(n) total pops: each pushed value is removed at most once.
            while next_popped < popped.len() && stack.last() == Some(&popped[next_popped]) {
                stack.pop();
                next_popped += 1;
            }
        }

        next_popped == popped.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_official_examples() {
        assert!(Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 5, 3, 2, 1]
        ));
        assert!(!Solution::validate_stack_sequences(
            vec![1, 2, 3, 4, 5],
            vec![4, 3, 5, 1, 2]
        ));
    }

    #[test]
    fn test_singleton() {
        assert!(Solution::validate_stack_sequences(vec![0], vec![0]));
    }

    #[test]
    fn test_ascending() {
        assert!(Solution::validate_stack_sequences(
            vec![0, 1, 2, 3],
            vec![0, 1, 2, 3]
        ));
    }

    #[test]
    fn test_reversed() {
        assert!(Solution::validate_stack_sequences(
            vec![0, 1, 2, 3],
            vec![3, 2, 1, 0]
        ));
    }

    #[test]
    fn test_impossible_interleaving() {
        assert!(!Solution::validate_stack_sequences(
            vec![1, 2, 3, 4],
            vec![3, 1, 2, 4]
        ));
    }

    #[test]
    fn test_maximum_length() {
        let pushed: Vec<i32> = (0..1000).collect();
        let popped: Vec<i32> = (0..500).rev().chain((500..1000).rev()).collect();
        assert!(Solution::validate_stack_sequences(pushed, popped));
    }
}
