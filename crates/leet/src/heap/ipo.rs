/// leet 502

use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    /// Greedy + max-heap. O(n log n + k log n) time, O(n) space.
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let n = profits.len();
        let mut projects: Vec<(i32, i32)> = capital.into_iter().zip(profits).collect();
        projects.sort_unstable();
        let mut pq = BinaryHeap::new(); // max-heap of profits
        let mut i = 0;
        let mut w = w;
        for _ in 0..k {
            while i < n && projects[i].0 <= w {
                pq.push(projects[i].1);
                i += 1;
            }
            if let Some(profit) = pq.pop() {
                w += profit;
            } else {
                break;
            }
        }
        w
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]),
            4
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]),
            6
        );
    }

    #[test]
    fn k_larger_than_projects() {
        assert_eq!(
            Solution::find_maximized_capital(5, 0, vec![1, 2], vec![0, 0]),
            3
        );
    }

    #[test]
    fn no_affordable_projects() {
        assert_eq!(
            Solution::find_maximized_capital(2, 0, vec![5, 10], vec![3, 5]),
            0
        );
    }
}
