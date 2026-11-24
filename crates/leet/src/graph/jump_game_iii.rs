use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    /// Breadth-first search takes O(n) time and O(n) space.
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let indices = 0..arr.len() as i32;
        let mut visited = vec![false; arr.len()];
        let mut queue = VecDeque::from([start]);
        visited[start as usize] = true;

        while let Some(index) = queue.pop_front() {
            let jump = arr[index as usize];
            if jump == 0 {
                return true;
            }
            for next in [index - jump, index + jump] {
                if indices.contains(&next) && !visited[next as usize] {
                    visited[next as usize] = true;
                    queue.push_back(next);
                }
            }
        }
        false
    }
}

pub struct Solution2;

impl Solution2 {
    /// Iterative depth-first search takes O(n) time and O(n) space.
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let indices = 0..arr.len() as i32;
        let mut visited = vec![false; arr.len()];
        let mut stack = vec![start];
        visited[start as usize] = true;

        while let Some(index) = stack.pop() {
            let jump = arr[index as usize];
            if jump == 0 {
                return true;
            }
            for next in [index - jump, index + jump] {
                if indices.contains(&next) && !visited[next as usize] {
                    visited[next as usize] = true;
                    stack.push(next);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{Solution, Solution2};

    fn check(arr: &[i32], start: i32, expected: bool) {
        assert_eq!(Solution::can_reach(arr.to_vec(), start), expected);
        assert_eq!(Solution2::can_reach(arr.to_vec(), start), expected);
    }

    #[test]
    fn reaches_zero_from_five() {
        check(&[4, 2, 3, 0, 3, 1, 2], 5, true);
    }

    #[test]
    fn reaches_zero_from_first_index() {
        check(&[4, 2, 3, 0, 3, 1, 2], 0, true);
    }

    #[test]
    fn cannot_reach_existing_zero() {
        check(&[3, 0, 2, 1, 2], 2, false);
    }

    #[test]
    fn starts_on_zero() {
        check(&[0], 0, true);
    }

    #[test]
    fn reaches_zero_by_jumping_right() {
        check(&[2, 1, 0], 0, true);
    }

    #[test]
    fn reaches_zero_by_jumping_left() {
        check(&[1, 0, 2], 2, true);
    }

    #[test]
    fn both_jumps_are_out_of_bounds() {
        check(&[2, 2, 2], 1, false);
    }

    #[test]
    fn terminates_on_cycle() {
        check(&[1, 1], 0, false);
    }
}
