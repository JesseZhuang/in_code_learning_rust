use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Count each string, then scan the original order to preserve distinct-string order.
    /// Expected time: O(n); space: O(n), where n is the array length.
    pub fn kth_distinct(arr: Vec<String>, mut k: i32) -> String {
        let mut counts = HashMap::with_capacity(arr.len());
        for item in &arr {
            *counts.entry(item.clone()).or_insert(0) += 1;
        }

        for item in arr {
            if counts.get(&item) == Some(&1) {
                k -= 1;
                if k == 0 {
                    return item;
                }
            }
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn returns_kth_distinct_string_from_official_example() {
        let arr = ["d", "b", "c", "b", "c", "a"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(Solution::kth_distinct(arr, 2), "a");
    }

    #[test]
    fn returns_first_and_last_distinct_strings_in_order() {
        let arr = ["a", "b", "c", "d"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>();

        assert_eq!(Solution::kth_distinct(arr.clone(), 1), "a");
        assert_eq!(Solution::kth_distinct(arr, 4), "d");
    }

    #[test]
    fn ignores_repeated_strings() {
        let arr = ["a", "a", "b", "c", "c", "d"]
            .into_iter()
            .map(String::from)
            .collect();

        assert_eq!(Solution::kth_distinct(arr, 2), "d");
    }

    #[test]
    fn returns_empty_when_fewer_than_k_distinct_strings_exist() {
        let arr = ["a", "b", "a"].into_iter().map(String::from).collect();

        assert_eq!(Solution::kth_distinct(arr, 3), "");
    }

    #[test]
    fn returns_single_element_when_it_is_distinct() {
        assert_eq!(
            Solution::kth_distinct(vec![String::from("only")], 1),
            "only"
        );
    }
}
