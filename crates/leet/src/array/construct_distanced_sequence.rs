/// leet 1718
pub struct Solution;

impl Solution {
    pub fn construct_distanced_sequence(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut sequence = vec![0; 2 * n - 1];
        let mut used = vec![false; n + 1];
        Self::search(0, &mut sequence, &mut used);
        sequence
    }

    /// Conservative worst case: O(n * n!) time, O(n) auxiliary space.
    /// Each level places one unused value; scanning slots and candidates costs O(n).
    /// Descending choices at the leftmost empty slot make the first solution largest.
    fn search(mut index: usize, sequence: &mut [i32], used: &mut [bool]) -> bool {
        while index < sequence.len() && sequence[index] != 0 {
            index += 1;
        }
        if index == sequence.len() {
            return true;
        }

        for value in (1..used.len()).rev() {
            if used[value] {
                continue;
            }
            let second = if value == 1 { index } else { index + value };
            if second >= sequence.len() || sequence[second] != 0 {
                continue;
            }

            used[value] = true;
            sequence[index] = value as i32;
            sequence[second] = value as i32;
            if Self::search(index + 1, sequence, used) {
                return true;
            }
            sequence[index] = 0;
            sequence[second] = 0;
            used[value] = false;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn single_number() {
        assert_eq!(Solution::construct_distanced_sequence(1), vec![1]);
    }

    #[test]
    fn official_examples_are_lexicographically_largest() {
        assert_eq!(
            Solution::construct_distanced_sequence(3),
            vec![3, 1, 2, 3, 2]
        );
        assert_eq!(
            Solution::construct_distanced_sequence(5),
            vec![5, 3, 1, 4, 3, 5, 2, 4, 2]
        );
    }

    #[test]
    fn maximum_input_has_valid_counts_and_distances() {
        let n = 20;
        let sequence = Solution::construct_distanced_sequence(n as i32);
        assert_eq!(sequence.len(), 2 * n - 1);
        let mut positions = vec![Vec::new(); n + 1];
        for (index, &value) in sequence.iter().enumerate() {
            assert!((1..=n as i32).contains(&value));
            positions[value as usize].push(index);
        }
        assert_eq!(positions[1].len(), 1);
        for (value, indices) in positions.iter().enumerate().skip(2) {
            assert_eq!(indices.len(), 2, "count for {value}");
            assert_eq!(indices[1] - indices[0], value, "distance for {value}");
        }
    }
}
