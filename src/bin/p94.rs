fn main() {
    println!("94. Binary Tree Inorder Traversal")
}

type RcTree = Rc<RefCell<TreeNode>>;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<RcTree>,
    pub right: Option<RcTree>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32, left: Option<RcTree>, right: Option<RcTree>) -> Option<RcTree> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }
}

use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut paths: Vec<i32> = Vec::with_capacity(100);

        fn traversal(node: Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<i32>) {
            if let Some(n) = node {
                traversal(n.borrow().left.clone(), paths);
                paths.push(n.borrow().val);
                traversal(n.borrow().right.clone(), paths);
            }
        }

        traversal(root, &mut paths);
        paths
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            Solution::inorder_traversal(TreeNode::new(1, None, None)),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test2() {
        assert_eq!(Solution::inorder_traversal(3), vec![]);
    }

    #[test]
    fn test3() {
        assert_eq!(Solution::inorder_traversal(4), vec![1]);
    }
}
