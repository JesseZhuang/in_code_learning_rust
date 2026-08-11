// LeetCode 332 - Reconstruct Itinerary
// Hierholzer's algorithm for Eulerian path.
// Time: O(E log E) for sorting adjacency lists; Space: O(E) for graph + result.

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    /// Recursive DFS (Hierholzer's). Build adjacency list sorted in reverse so we can pop
    /// the lexicographically smallest neighbor. Post-order collection, then reverse.
    pub fn find_itinerary(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        for ticket in &tickets {
            graph.entry(ticket[0].as_str()).or_default().push(ticket[1].as_str());
        }
        // Sort each adjacency list in reverse lexicographic order so pop gives smallest.
        for dests in graph.values_mut() {
            dests.sort_unstable_by(|a, b| b.cmp(a));
        }

        let mut route: Vec<&str> = Vec::with_capacity(tickets.len() + 1);

        fn dfs<'a>(node: &'a str, graph: &mut HashMap<&'a str, Vec<&'a str>>, route: &mut Vec<&'a str>) {
            while let Some(next) = graph.get_mut(node).and_then(Vec::pop) {
                dfs(next, graph, route);
            }
            route.push(node);
        }

        dfs("JFK", &mut graph, &mut route);
        route.reverse();
        route.into_iter().map(String::from).collect()
    }

    /// Iterative stack version of Hierholzer's algorithm.
    pub fn find_itinerary_iterative(tickets: Vec<Vec<String>>) -> Vec<String> {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        for ticket in &tickets {
            graph.entry(ticket[0].as_str()).or_default().push(ticket[1].as_str());
        }
        for dests in graph.values_mut() {
            dests.sort_unstable_by(|a, b| b.cmp(a));
        }

        let mut stack: Vec<&str> = vec!["JFK"];
        let mut route: Vec<&str> = Vec::with_capacity(tickets.len() + 1);

        while let Some(&node) = stack.last() {
            if graph.get(node).map_or(true, |v| v.is_empty()) {
                route.push(stack.pop().unwrap());
            } else {
                let next = graph.get_mut(node).unwrap().pop().unwrap();
                stack.push(next);
            }
        }

        route.reverse();
        route.into_iter().map(String::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_tickets(input: &[&[&str; 2]]) -> Vec<Vec<String>> {
        input.iter().map(|t| vec![t[0].to_string(), t[1].to_string()]).collect()
    }

    fn to_strings(input: &[&str]) -> Vec<String> {
        input.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_basic() {
        let tickets = to_tickets(&[&["MUC","LHR"],&["JFK","MUC"],&["SFO","SJC"],&["LHR","SFO"]]);
        let expected = to_strings(&["JFK","MUC","LHR","SFO","SJC"]);
        assert_eq!(Solution::find_itinerary(tickets.clone()), expected);
        assert_eq!(Solution::find_itinerary_iterative(tickets), expected);
    }

    #[test]
    fn test_multiple_from_same() {
        let tickets = to_tickets(&[&["JFK","SFO"],&["JFK","ATL"],&["SFO","ATL"],&["ATL","JFK"],&["ATL","SFO"]]);
        let expected = to_strings(&["JFK","ATL","JFK","SFO","ATL","SFO"]);
        assert_eq!(Solution::find_itinerary(tickets.clone()), expected);
        assert_eq!(Solution::find_itinerary_iterative(tickets), expected);
    }

    #[test]
    fn test_single_ticket() {
        let tickets = to_tickets(&[&["JFK","A"]]);
        let expected = to_strings(&["JFK","A"]);
        assert_eq!(Solution::find_itinerary(tickets.clone()), expected);
        assert_eq!(Solution::find_itinerary_iterative(tickets), expected);
    }

    #[test]
    fn test_lexicographic_choice() {
        let tickets = to_tickets(&[&["JFK","KUL"],&["JFK","NRT"],&["NRT","JFK"]]);
        let expected = to_strings(&["JFK","NRT","JFK","KUL"]);
        assert_eq!(Solution::find_itinerary(tickets.clone()), expected);
        assert_eq!(Solution::find_itinerary_iterative(tickets), expected);
    }

    #[test]
    fn test_duplicate_tickets() {
        let tickets = to_tickets(&[&["JFK","A"],&["A","JFK"],&["JFK","A"]]);
        let expected = to_strings(&["JFK","A","JFK","A"]);
        assert_eq!(Solution::find_itinerary(tickets.clone()), expected);
        assert_eq!(Solution::find_itinerary_iterative(tickets), expected);
    }
}
