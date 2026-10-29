use std::collections::VecDeque;

// LeetCode 1700, easy. Tags: array, queue, simulation, counting.
pub struct Solution;

impl Solution {
    /// Counts preferences instead of rotating students. O(n) time, O(1) extra space.
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut remaining = [0_i32; 2];
        for student in students {
            remaining[student as usize] += 1;
        }

        for sandwich in sandwiches {
            let sandwich = sandwich as usize;
            if remaining[sandwich] == 0 {
                return remaining[1 - sandwich];
            }
            remaining[sandwich] -= 1;
        }

        0
    }

    /// Directly simulates queue rotations. O(n^2) time, O(n) extra space.
    pub fn count_students_queue(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut queue: VecDeque<i32> = students.into();
        let mut sandwich_index = 0;
        let mut rotations_without_eating = 0;

        while !queue.is_empty() && rotations_without_eating < queue.len() {
            if queue.front() == sandwiches.get(sandwich_index) {
                queue.pop_front();
                sandwich_index += 1;
                rotations_without_eating = 0;
            } else {
                let student = queue.pop_front().unwrap();
                queue.push_back(student);
                rotations_without_eating += 1;
            }
        }

        queue.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    fn assert_both(students: Vec<i32>, sandwiches: Vec<i32>, expected: i32) {
        assert_eq!(
            Solution::count_students(students.clone(), sandwiches.clone()),
            expected
        );
        assert_eq!(
            Solution::count_students_queue(students, sandwiches),
            expected
        );
    }

    #[test]
    fn examples() {
        assert_both(vec![1, 1, 0, 0], vec![0, 1, 0, 1], 0);
        assert_both(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1], 3);
    }

    #[test]
    fn every_student_eats_same_type() {
        assert_both(vec![0, 0, 0], vec![0, 0, 0], 0);
        assert_both(vec![1, 1], vec![1, 1], 0);
    }

    #[test]
    fn first_sandwich_has_no_matching_student() {
        assert_both(vec![0, 0, 0], vec![1, 0, 0], 3);
    }

    #[test]
    fn late_exhaustion_leaves_remaining_students() {
        assert_both(vec![0, 1, 0, 1], vec![0, 1, 1, 1], 1);
    }

    #[test]
    fn single_student() {
        assert_both(vec![0], vec![0], 0);
        assert_both(vec![0], vec![1], 1);
    }

    #[test]
    fn rotations_are_required_before_a_match() {
        assert_both(vec![1, 1, 0], vec![0, 1, 1], 0);
    }
}
