// You are given the heads of two sorted linked lists list1 and list2.
// Merge the two lists into one sorted list.
// The list should be made by splicing together the nodes of the first two lists.
// Return the head of the merged linked list.

// Example 1:
// Input: list1 = [1,2,4], list2 = [1,3,4]
// Output: [1,1,2,3,4,4]

// Example 2:
// Input: list1 = [], list2 = []
// Output: []

// Example 3:
// Input: list1 = [], list2 = [0]
// Output: [0]

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

    fn list_to_vector(head: Option<Box<ListNode>>) -> Vec<i32> {
        // Create a new vector
        let mut stack: Vec<i32> = Vec::new();
        // Create a reference to the head
        let mut current = &head;

        while let Some(node) = current {
            stack.push(node.val);
            current = &node.next;
        }

        stack
    }

    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut vector1 = Self::list_to_vector(list1);
        let mut vector2 = Self::list_to_vector(list2);

        vector1.append(&mut vector2);
        vector1.sort();

        Self::vector_to_list(vector1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_to_list() {
        assert_eq!(
            Solution::vector_to_list(vec![1, 2, 3]),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            }))
        );
    }

    #[test]
    fn test_list_to_vector() {
        assert_eq!(
            Solution::list_to_vector(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode { val: 3, next: None }))
                }))
            }))),
            vec![1, 2, 3]
        );
    }

    #[test]
    fn test_merge_two_lists() {
        assert_eq!(
            Solution::merge_two_lists(
                Solution::vector_to_list(vec![1, 2, 4]),
                Solution::vector_to_list(vec![1, 3, 4])
            ),
            Solution::vector_to_list(vec![1, 1, 2, 3, 4, 4])
        );

        assert_eq!(
            Solution::merge_two_lists(
                Solution::vector_to_list(vec![]),
                Solution::vector_to_list(vec![])
            ),
            Solution::vector_to_list(vec![])
        );

        assert_eq!(
            Solution::merge_two_lists(
                Solution::vector_to_list(vec![]),
                Solution::vector_to_list(vec![0])
            ),
            Solution::vector_to_list(vec![0])
        );
    }
}
