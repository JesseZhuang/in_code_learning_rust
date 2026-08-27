// LeetCode 1268 - Search Suggestions System
// Time: O(n log n + m * L * log n) where n = products.len(), m = searchWord.len(), L = max product length
// Space: O(sort) — sorting space only; output excluded

pub struct Solution;

impl Solution {
    pub fn suggested_products(mut products: Vec<String>, search_word: String) -> Vec<Vec<String>> {
        products.sort();
        let mut result = Vec::with_capacity(search_word.len());

        for i in 1..=search_word.len() {
            let prefix = &search_word[..i];
            // Find first product >= prefix using binary search
            let start = products.partition_point(|p| p.as_str() < prefix);
            let mut suggestions = Vec::new();
            for j in start..products.len().min(start + 3) {
                if products[j].starts_with(prefix) {
                    suggestions.push(products[j].clone());
                } else {
                    break;
                }
            }
            result.push(suggestions);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let products = vec!["mobile", "mouse", "moneypot", "monitor", "mousepad"]
            .into_iter().map(String::from).collect();
        let result = Solution::suggested_products(products, "mouse".to_string());
        assert_eq!(
            result,
            vec![
                vec!["mobile", "moneypot", "monitor"],
                vec!["mobile", "moneypot", "monitor"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"],
            ]
        );
    }

    #[test]
    fn test_example2() {
        let products = vec!["havana"].into_iter().map(String::from).collect();
        let result = Solution::suggested_products(products, "havana".to_string());
        assert_eq!(
            result,
            vec![
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
                vec!["havana"],
            ]
        );
    }

    #[test]
    fn test_no_match() {
        let products = vec!["apple", "apricot"].into_iter().map(String::from).collect();
        let result = Solution::suggested_products(products, "bx".to_string());
        assert_eq!(result, vec![Vec::<String>::new(), Vec::<String>::new()]);
    }
}
