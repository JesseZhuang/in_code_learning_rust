pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res = vec![0i32; n]; // O(n) space
        let mut stack: Vec<usize> = Vec::new(); // monotonic decreasing stack of indices, O(n) space
        for i in 0..n { // O(n)
            while let Some(&j) = stack.last() { // O(n) total pops
                if temperatures[j] < temperatures[i] {
                    stack.pop();
                    res[j] = (i - j) as i32;
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        res // Time O(n), Space O(n)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_examples() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(Solution::daily_temperatures(vec![50]), vec![0]);
        assert_eq!(
            Solution::daily_temperatures(vec![90, 80, 70, 60]),
            vec![0, 0, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![70, 70, 70]),
            vec![0, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 30, 30, 30, 31]),
            vec![4, 3, 2, 1, 0]
        );
    }
}
