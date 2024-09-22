struct Solution;

use std::rc::Rc;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(values: Vec<i32>) -> Option<Box<Self>> {
        let mut head = None;
        for val in values.into_iter().rev() {
            let mut node = Self::new(val);
            node.next = head;
            head = Some(Box::new(node));
        }
        head
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(list1_node), Some(list2_node)) => {
                if list1_node.val <= list2_node.val {
                    Some(Box::new(ListNode {
                        val: list1_node.val,
                        next: Self::merge_two_lists(list1_node.next, Some(list2_node)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: list2_node.val,
                        next: Self::merge_two_lists(Some(list1_node), list2_node.next),
                    }))
                }
            }
            (None, Some(node)) | (Some(node), None) => Some(node),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        assert_eq!(
            Solution::merge_two_lists(
                ListNode::from_vec(vec![1, 2, 4]),
                ListNode::from_vec(vec![1, 3, 4]),
            ),
            ListNode::from_vec(vec![1, 1, 2, 3, 4, 4])
        );
    }
    #[test]
    fn test2() {
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(vec![]), ListNode::from_vec(vec![]),),
            ListNode::from_vec(vec![])
        );
    }

    #[test]
    fn test3() {
        assert_eq!(
            Solution::merge_two_lists(ListNode::from_vec(vec![]), ListNode::from_vec(vec![0]),),
            ListNode::from_vec(vec![0])
        );
    }
}
