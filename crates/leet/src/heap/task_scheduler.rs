// lc 621 - Task Scheduler

pub struct Solution;

impl Solution {
    /// Math/greedy approach.
    /// Time O(n), Space O(1) — 26-letter frequency array.
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut count = [0i32; 26];
        let mut max_freq = 0i32;
        let mut max_cnt = 0i32;
        for &t in &tasks { // O(n)
            let i = (t as u8 - b'A') as usize;
            count[i] += 1;
            if count[i] > max_freq {
                max_freq = count[i];
                max_cnt = 1;
            } else if count[i] == max_freq {
                max_cnt += 1;
            }
        }
        let n_gaps = max_freq - 1;
        let gap_len = n + 1;
        (tasks.len() as i32).max(n_gaps * gap_len + max_cnt)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(8, Solution::least_interval(vec!['A','A','A','B','B','B'], 2));
    }

    #[test]
    fn example2() {
        assert_eq!(6, Solution::least_interval(vec!['A','C','A','B','D','B'], 1));
    }

    #[test]
    fn example3() {
        assert_eq!(10, Solution::least_interval(vec!['A','A','A','B','B','B'], 3));
    }

    #[test]
    fn no_cooling() {
        assert_eq!(3, Solution::least_interval(vec!['A','B','C'], 0));
    }

    #[test]
    fn single_task() {
        assert_eq!(1, Solution::least_interval(vec!['A'], 2));
    }

    #[test]
    fn all_same_task() {
        assert_eq!(7, Solution::least_interval(vec!['A','A','A'], 2));
    }

    #[test]
    fn many_unique() {
        let tasks: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        assert_eq!(26, Solution::least_interval(tasks, 100));
    }

    #[test]
    fn high_cooling_two_tasks() {
        assert_eq!(5, Solution::least_interval(vec!['A','A','B'], 3));
    }
}
