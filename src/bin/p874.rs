fn main() {}

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32, next: Option<Box<ListNode>>) -> Self {
        ListNode { next, val }
    }
}

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = head.clone();
        let mut slow = head;

        while let Some(node) = fast {
            match node.next {
                Some(next_node) => {
                    fast = next_node.next;
                    slow = slow.unwrap().next;
                }
                None => break,
            }
        }

        slow
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // 0..=0 -> 0
    // 0..=1 -> 1
    // 0..=2 -> 1
    // 0..=3 -> 2
    // 0..=4 -> 2
    // 0..=5 -> 3
    // 0..=6 -> 3
    // 0..=7 -> 4

    #[test]
    fn test1() {
        let input = ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                2,
                Some(Box::new(ListNode::new(
                    3,
                    Some(Box::new(ListNode::new(
                        4,
                        Some(Box::new(ListNode::new(5, None))),
                    ))),
                ))),
            ))),
        );
        let expected = ListNode::new(
            3,
            Some(Box::new(ListNode::new(
                4,
                Some(Box::new(ListNode::new(5, None))),
            ))),
        );
        assert_eq!(
            Solution::middle_node(Some(Box::new(input))),
            Some(Box::new(expected))
        );
    }

    #[test]
    fn test2() {
        let input = ListNode::new(
            1,
            Some(Box::new(ListNode::new(
                2,
                Some(Box::new(ListNode::new(
                    3,
                    Some(Box::new(ListNode::new(
                        4,
                        Some(Box::new(ListNode::new(
                            5,
                            Some(Box::new(ListNode::new(6, None))),
                        ))),
                    ))),
                ))),
            ))),
        );
        let expected = ListNode::new(
            4,
            Some(Box::new(ListNode::new(
                5,
                Some(Box::new(ListNode::new(6, None))),
            ))),
        );
        assert_eq!(
            Solution::middle_node(Some(Box::new(input))),
            Some(Box::new(expected))
        );
    }
}
