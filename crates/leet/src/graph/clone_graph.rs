use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc::Rc;

type NodeRef = Rc<RefCell<GNode>>;

#[derive(Debug)]
pub struct GNode {
    pub val: i32,
    pub neighbors: Vec<NodeRef>,
}

impl GNode {
    pub fn new(val: i32) -> Self {
        GNode {
            val,
            neighbors: vec![],
        }
    }
}

pub struct Solution;

impl Solution {
    /// DFS clone - O(V+E) time, O(V) space
    pub fn clone_graph_dfs(node: Option<NodeRef>) -> Option<NodeRef> {
        let node = node?;
        let mut visited: HashMap<i32, NodeRef> = HashMap::new();
        Some(Self::dfs(&node, &mut visited))
    }

    fn dfs(node: &NodeRef, visited: &mut HashMap<i32, NodeRef>) -> NodeRef {
        let val = node.borrow().val;
        if let Some(cloned) = visited.get(&val) {
            return Rc::clone(cloned);
        }

        let clone = Rc::new(RefCell::new(GNode::new(val)));
        visited.insert(val, Rc::clone(&clone));

        let neighbors: Vec<NodeRef> = node.borrow().neighbors.clone();
        for neighbor in &neighbors {
            let cloned_neighbor = Self::dfs(neighbor, visited);
            clone.borrow_mut().neighbors.push(cloned_neighbor);
        }

        clone
    }

    /// BFS clone - O(V+E) time, O(V) space
    pub fn clone_graph_bfs(node: Option<NodeRef>) -> Option<NodeRef> {
        let node = node?;
        let mut visited: HashMap<i32, NodeRef> = HashMap::new();
        let mut queue = VecDeque::new();

        let val = node.borrow().val;
        let clone = Rc::new(RefCell::new(GNode::new(val)));
        visited.insert(val, Rc::clone(&clone));
        queue.push_back(Rc::clone(&node));

        while let Some(current) = queue.pop_front() {
            let current_val = current.borrow().val;
            let neighbors: Vec<NodeRef> = current.borrow().neighbors.clone();

            for neighbor in &neighbors {
                let neighbor_val = neighbor.borrow().val;
                if !visited.contains_key(&neighbor_val) {
                    let neighbor_clone = Rc::new(RefCell::new(GNode::new(neighbor_val)));
                    visited.insert(neighbor_val, Rc::clone(&neighbor_clone));
                    queue.push_back(Rc::clone(neighbor));
                }
                let cloned_neighbor = Rc::clone(visited.get(&neighbor_val).unwrap());
                visited
                    .get(&current_val)
                    .unwrap()
                    .borrow_mut()
                    .neighbors
                    .push(cloned_neighbor);
            }
        }

        Some(clone)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a graph from adjacency list (1-indexed).
    /// adj_list[i] contains the neighbors of node i+1.
    fn build_graph(adj_list: &[Vec<i32>]) -> Option<NodeRef> {
        if adj_list.is_empty() {
            return None;
        }

        let nodes: Vec<NodeRef> = (0..adj_list.len())
            .map(|i| Rc::new(RefCell::new(GNode::new(i as i32 + 1))))
            .collect();

        for (i, neighbors) in adj_list.iter().enumerate() {
            for &n in neighbors {
                nodes[i]
                    .borrow_mut()
                    .neighbors
                    .push(Rc::clone(&nodes[n as usize - 1]));
            }
        }

        Some(Rc::clone(&nodes[0]))
    }

    /// Convert graph back to adjacency list (1-indexed) via BFS.
    fn graph_to_adj_list(node: Option<NodeRef>) -> Vec<Vec<i32>> {
        let node = match node {
            Some(n) => n,
            None => return vec![],
        };

        let mut visited: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back(Rc::clone(&node));
        visited.insert(node.borrow().val, vec![]);

        while let Some(current) = queue.pop_front() {
            let val = current.borrow().val;
            let neighbors: Vec<NodeRef> = current.borrow().neighbors.clone();
            let mut neighbor_vals = vec![];
            for neighbor in &neighbors {
                let nval = neighbor.borrow().val;
                neighbor_vals.push(nval);
                if !visited.contains_key(&nval) {
                    visited.insert(nval, vec![]);
                    queue.push_back(Rc::clone(neighbor));
                }
            }
            visited.insert(val, neighbor_vals);
        }

        let max_val = *visited.keys().max().unwrap_or(&0);
        let mut result = vec![vec![]; max_val as usize];
        for (val, neighbors) in &visited {
            result[*val as usize - 1] = neighbors.clone();
        }
        result
    }

    /// Verify that cloned graph is a deep copy (different Rc pointers).
    fn verify_deep_copy(original: &Option<NodeRef>, cloned: &Option<NodeRef>) {
        if original.is_none() && cloned.is_none() {
            return;
        }
        let orig = original.as_ref().unwrap();
        let clone = cloned.as_ref().unwrap();
        assert!(!Rc::ptr_eq(orig, clone));
    }

    #[test]
    fn test_four_node_cycle_dfs() {
        let adj = vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]];
        let graph = build_graph(&adj);
        let cloned = Solution::clone_graph_dfs(graph.clone());
        verify_deep_copy(&graph, &cloned);
        let mut result = graph_to_adj_list(cloned);
        for neighbors in &mut result {
            neighbors.sort();
        }
        assert_eq!(result, vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]]);
    }

    #[test]
    fn test_four_node_cycle_bfs() {
        let adj = vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]];
        let graph = build_graph(&adj);
        let cloned = Solution::clone_graph_bfs(graph.clone());
        verify_deep_copy(&graph, &cloned);
        let mut result = graph_to_adj_list(cloned);
        for neighbors in &mut result {
            neighbors.sort();
        }
        assert_eq!(result, vec![vec![2, 4], vec![1, 3], vec![2, 4], vec![1, 3]]);
    }

    #[test]
    fn test_single_node_dfs() {
        let adj = vec![vec![]];
        let graph = build_graph(&adj);
        let cloned = Solution::clone_graph_dfs(graph.clone());
        verify_deep_copy(&graph, &cloned);
        let result = graph_to_adj_list(cloned);
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_single_node_bfs() {
        let adj = vec![vec![]];
        let graph = build_graph(&adj);
        let cloned = Solution::clone_graph_bfs(graph.clone());
        verify_deep_copy(&graph, &cloned);
        let result = graph_to_adj_list(cloned);
        assert_eq!(result, vec![vec![]]);
    }

    #[test]
    fn test_empty_graph_dfs() {
        let cloned = Solution::clone_graph_dfs(None);
        assert!(cloned.is_none());
    }

    #[test]
    fn test_empty_graph_bfs() {
        let cloned = Solution::clone_graph_bfs(None);
        assert!(cloned.is_none());
    }

    #[test]
    fn test_two_nodes_dfs() {
        let adj = vec![vec![2], vec![1]];
        let graph = build_graph(&adj);
        let cloned = Solution::clone_graph_dfs(graph.clone());
        verify_deep_copy(&graph, &cloned);
        let mut result = graph_to_adj_list(cloned);
        for neighbors in &mut result {
            neighbors.sort();
        }
        assert_eq!(result, vec![vec![2], vec![1]]);
    }

    #[test]
    fn test_two_nodes_bfs() {
        let adj = vec![vec![2], vec![1]];
        let graph = build_graph(&adj);
        let cloned = Solution::clone_graph_bfs(graph.clone());
        verify_deep_copy(&graph, &cloned);
        let mut result = graph_to_adj_list(cloned);
        for neighbors in &mut result {
            neighbors.sort();
        }
        assert_eq!(result, vec![vec![2], vec![1]]);
    }
}
