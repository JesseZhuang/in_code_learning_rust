// LeetCode 1041, medium, tags: math, string, simulation.

pub struct Solution;

impl Solution {
    /// O(n) time, O(1) space.
    pub fn is_robot_bounded(instructions: &str) -> bool {
        let dirs: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (mut x, mut y, mut d) = (0i32, 0i32, 0usize);
        for c in instructions.chars() {
            match c {
                'G' => {
                    x += dirs[d].0;
                    y += dirs[d].1;
                }
                'R' => d = (d + 1) % 4,
                'L' => d = (d + 3) % 4,
                _ => {}
            }
        }
        (x == 0 && y == 0) || d > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert!(Solution::is_robot_bounded("GGLLGG"));
    }

    #[test]
    fn test_example2() {
        assert!(!Solution::is_robot_bounded("GG"));
    }

    #[test]
    fn test_example3() {
        assert!(Solution::is_robot_bounded("GL"));
    }
}
