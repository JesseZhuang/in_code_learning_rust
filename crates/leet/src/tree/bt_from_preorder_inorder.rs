/// LeetCode 105: Construct Binary Tree from Preorder and Inorder Traversal

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::structs::tree_node::TreeNode;

/// Solution 1: Recursive DFS with HashMap
/// Time: O(n), Space: O(n)
pub struct Solution;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_map: HashMap<i32, usize> = inorder.iter().enumerate().map(|(i, &v)| (v, i)).collect();
        Self::helper(&preorder, &mut 0, 0, inorder.len() as i32 - 1, &inorder_map)
    }

    fn helper(
        preorder: &[i32],
        pre_idx: &mut usize,
        in_left: i32,
        in_right: i32,
        inorder_map: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if in_left > in_right {
            return None;
        }
        let root_val = preorder[*pre_idx];
        *pre_idx += 1;
        let in_idx = *inorder_map.get(&root_val).unwrap() as i32;

        let left = Self::helper(preorder, pre_idx, in_left, in_idx - 1, inorder_map);
        let right = Self::helper(preorder, pre_idx, in_idx + 1, in_right, inorder_map);

        Some(Rc::new(RefCell::new(TreeNode {
            val: root_val,
            left,
            right,
        })))
    }
}

/// Solution 2: Iterative with stack
/// Time: O(n), Space: O(n)
pub struct Solution2;

impl Solution2 {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }

        let root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
        let mut stack: Vec<Rc<RefCell<TreeNode>>> = vec![Rc::clone(&root)];
        let mut in_idx = 0;

        for i in 1..preorder.len() {
            let node = Rc::new(RefCell::new(TreeNode::new(preorder[i])));
            let mut parent = Rc::clone(stack.last().unwrap());

            if stack.last().unwrap().borrow().val != inorder[in_idx] {
                // Attach as left child
                parent.borrow_mut().left = Some(Rc::clone(&node));
            } else {
                // Pop until we find the parent for the right child
                while !stack.is_empty() && stack.last().unwrap().borrow().val == inorder[in_idx] {
                    parent = Rc::clone(stack.last().unwrap());
                    stack.pop();
                    in_idx += 1;
                }
                parent.borrow_mut().right = Some(Rc::clone(&node));
            }
            stack.push(node);
        }

        Some(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn val(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        node.as_ref().unwrap().borrow().val
    }

    #[test]
    fn test_example1() {
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];

        let tree1 = Solution::build_tree(preorder.clone(), inorder.clone());
        assert_eq!(val(&tree1), 3);
        assert_eq!(val(&tree1.as_ref().unwrap().borrow().left), 9);
        assert_eq!(val(&tree1.as_ref().unwrap().borrow().right), 20);

        let tree2 = Solution2::build_tree(preorder, inorder);
        assert_eq!(val(&tree2), 3);
        assert_eq!(val(&tree2.as_ref().unwrap().borrow().left), 9);
        assert_eq!(val(&tree2.as_ref().unwrap().borrow().right), 20);
    }

    #[test]
    fn test_single() {
        let preorder = vec![-1];
        let inorder = vec![-1];

        let tree1 = Solution::build_tree(preorder.clone(), inorder.clone());
        assert_eq!(val(&tree1), -1);

        let tree2 = Solution2::build_tree(preorder, inorder);
        assert_eq!(val(&tree2), -1);
    }

    #[test]
    fn test_left_skewed() {
        let preorder = vec![1, 2, 3];
        let inorder = vec![3, 2, 1];

        let tree1 = Solution::build_tree(preorder.clone(), inorder.clone());
        assert_eq!(val(&tree1), 1);
        assert_eq!(val(&tree1.as_ref().unwrap().borrow().left), 2);
        assert!(tree1.as_ref().unwrap().borrow().right.is_none());

        let tree2 = Solution2::build_tree(preorder, inorder);
        assert_eq!(val(&tree2), 1);
        assert_eq!(val(&tree2.as_ref().unwrap().borrow().left), 2);
        assert!(tree2.as_ref().unwrap().borrow().right.is_none());
    }

    #[test]
    fn test_right_skewed() {
        let preorder = vec![1, 2, 3];
        let inorder = vec![1, 2, 3];

        let tree1 = Solution::build_tree(preorder.clone(), inorder.clone());
        assert_eq!(val(&tree1), 1);
        assert!(tree1.as_ref().unwrap().borrow().left.is_none());
        assert_eq!(val(&tree1.as_ref().unwrap().borrow().right), 2);

        let tree2 = Solution2::build_tree(preorder, inorder);
        assert_eq!(val(&tree2), 1);
        assert!(tree2.as_ref().unwrap().borrow().left.is_none());
        assert_eq!(val(&tree2.as_ref().unwrap().borrow().right), 2);
    }

    #[test]
    fn test_empty() {
        let tree1 = Solution::build_tree(vec![], vec![]);
        assert!(tree1.is_none());

        let tree2 = Solution2::build_tree(vec![], vec![]);
        assert!(tree2.is_none());
    }
}
