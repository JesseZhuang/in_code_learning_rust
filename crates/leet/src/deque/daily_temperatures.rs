// lc 739

pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut res = vec![0i32; n];
        let mut stack: Vec<usize> = Vec::new(); // monotonic decreasing stack of indices
        for i in 0..n { // O(n)
            while let Some(&j) = stack.last() { // each index pushed/popped at most once, O(n) total
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
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![1, 1, 4, 2, 1, 1, 0, 0],
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73])
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec![1, 1, 1, 0],
            Solution::daily_temperatures(vec![30, 40, 50, 60])
        );
    }

    #[test]
    fn test_example3() {
        assert_eq!(
            vec![1, 1, 0],
            Solution::daily_temperatures(vec![30, 60, 90])
        );
    }

    #[test]
    fn test_single() {
        assert_eq!(vec![0], Solution::daily_temperatures(vec![50]));
    }

    #[test]
    fn test_decreasing() {
        assert_eq!(
            vec![0, 0, 0, 0],
            Solution::daily_temperatures(vec![90, 80, 70, 60])
        );
    }

    #[test]
    fn test_all_same() {
        assert_eq!(
            vec![0, 0, 0],
            Solution::daily_temperatures(vec![70, 70, 70])
        );
    }
}
