/// LeetCode 138 - Copy List with Random Pointer
///
/// Given a linked list where each node has a `next` and `random` pointer,
/// create a deep copy. We represent the list as Vec<(i32, Option<usize>)>
/// where each element is (value, random_index).

pub struct Solution;

impl Solution {
    /// Approach 1: Hash Map approach
    /// Time: O(n), Space: O(n)
    ///
    /// Simulates the hash map approach: iterate through the original list,
    /// create a copy of each node, and map old indices to new indices.
    /// Then set random pointers using the mapping.
    pub fn copy_random_list_hashmap(
        head: &[(i32, Option<usize>)],
    ) -> Vec<(i32, Option<usize>)> {
        use std::collections::HashMap;

        if head.is_empty() {
            return vec![];
        }

        // Map from original index to new index (identity in this representation,
        // but simulates the HashMap<*Node, *Node> approach)
        let mut index_map: HashMap<usize, usize> = HashMap::new();
        let mut result: Vec<(i32, Option<usize>)> = Vec::with_capacity(head.len());

        // First pass: copy all nodes (values), build index mapping
        for (i, &(val, _)) in head.iter().enumerate() {
            index_map.insert(i, i);
            result.push((val, None));
        }

        // Second pass: assign random pointers using the mapping
        for (i, &(_, random)) in head.iter().enumerate() {
            if let Some(rand_idx) = random {
                result[index_map[&i]].1 = Some(index_map[&rand_idx]);
            }
        }

        result
    }

    /// Approach 2: Direct index-based copy
    /// Time: O(n), Space: O(n)
    ///
    /// Since we represent the list with indices, we can directly copy
    /// values and random indices in a single pass. This mirrors the
    /// O(1) extra space interleaving approach conceptually but in Rust's
    /// index-based representation it simplifies to a direct clone with
    /// index remapping.
    pub fn copy_random_list_direct(
        head: &[(i32, Option<usize>)],
    ) -> Vec<(i32, Option<usize>)> {
        if head.is_empty() {
            return vec![];
        }

        // Direct copy: since indices are stable, we just copy each element
        head.iter()
            .map(|&(val, random)| (val, random))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_both(input: &[(i32, Option<usize>)], expected: &[(i32, Option<usize>)]) {
        let result1 = Solution::copy_random_list_hashmap(input);
        assert_eq!(result1, expected, "hashmap approach failed");

        let result2 = Solution::copy_random_list_direct(input);
        assert_eq!(result2, expected, "direct approach failed");
    }

    #[test]
    fn test_example_1() {
        // [[7,null],[13,0],[11,4],[10,2],[1,0]]
        let input = vec![
            (7, None),
            (13, Some(0)),
            (11, Some(4)),
            (10, Some(2)),
            (1, Some(0)),
        ];
        run_both(&input, &input);
    }

    #[test]
    fn test_example_2() {
        // [[1,1],[2,1]]
        let input = vec![(1, Some(1)), (2, Some(1))];
        run_both(&input, &input);
    }

    #[test]
    fn test_example_3() {
        // [[3,null],[3,0],[3,null]]
        let input = vec![(3, None), (3, Some(0)), (3, None)];
        run_both(&input, &input);
    }

    #[test]
    fn test_empty_list() {
        let input: Vec<(i32, Option<usize>)> = vec![];
        run_both(&input, &input);
    }

    #[test]
    fn test_single_node_no_random() {
        let input = vec![(1, None)];
        run_both(&input, &input);
    }

    #[test]
    fn test_single_node_self_random() {
        let input = vec![(1, Some(0))];
        run_both(&input, &input);
    }

    #[test]
    fn test_all_random_to_last() {
        let input = vec![
            (1, Some(3)),
            (2, Some(3)),
            (3, Some(3)),
            (4, Some(3)),
        ];
        run_both(&input, &input);
    }

    #[test]
    fn test_negative_values() {
        let input = vec![
            (-1, Some(2)),
            (-10000, None),
            (10000, Some(0)),
        ];
        run_both(&input, &input);
    }
}
