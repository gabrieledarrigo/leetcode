// Given the head of a singly linked list, reverse the list, and return the reversed list.

// Example 1:
// Input: head = [1,2,3,4,5]
// Output: [5,4,3,2,1]

// Example 2:
// Input: head = [1,2]
// Output: [2,1]

// Example 3:
// Input: head = []
// Output: []

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
}

pub struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev: Option<Box<ListNode>> = None;
        let mut current = head;

        while let Some(mut node) = current {
            current = node.next.take();
            node.next = prev;
            prev = Some(node);
        }

        prev
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_list() {
        fn vector_to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
            // Create a new list with a dummy head
            let mut head = Box::new(ListNode::new(0));
            // Create a reference to the head
            let mut current = &mut head;

            for values in vector {
                current.next = Some(Box::new(ListNode::new(values)));
                current = current.next.as_mut().unwrap();
            }

            head.next
        }

        assert_eq!(
            super::Solution::reverse_list(vector_to_list(vec![1, 2, 3, 4, 5])),
            vector_to_list(vec![5, 4, 3, 2, 1])
        );

        assert_eq!(
            super::Solution::reverse_list(vector_to_list(vec![1, 2])),
            vector_to_list(vec![2, 1])
        );

        assert_eq!(
            super::Solution::reverse_list(vector_to_list(vec![])),
            vector_to_list(vec![])
        );
    }
}
