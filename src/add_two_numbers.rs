// You are given two non-empty linked lists representing two non-negative integers.
// The digits are stored in reverse order, and each of their nodes contains a single digit.
// Add the two numbers and return the sum as a linked list.

// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

// Example 1:

// Input: l1 = [2,4,3], l2 = [5,6,4]
// Output: [7,0,8]
// Explanation: 342 + 465 = 807.

// Example 2:

// Input: l1 = [0], l2 = [0]
// Output: [0]

// Example 3:
// Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
// Output: [8,9,9,9,0,0,0,1]

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

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = Box::new(ListNode::new(0));
    let mut current = &mut head;

    let mut carry = 0;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let a = match l1 {
            Some(node) => {
                l1 = node.next;
                node.val
            }
            None => 0,
        };

        let b = match l2 {
            Some(node) => {
                l2 = node.next;
                node.val
            }
            None => 0,
        };

        let sum = a + b + carry;
        carry = sum / 10;

        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
    }

    head.next
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vector_to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(0));
        let mut current = &mut head;

        for value in vector {
            current.next = Some(Box::new(ListNode::new(value)));
            current = current.next.as_mut().unwrap();
        }

        head.next
    }

    #[test]
    fn test_add_two_numbers() {
        // Test case 1: Example from the problem description
        assert_eq!(
            add_two_numbers(vector_to_list(vec![2, 4, 3]), vector_to_list(vec![5, 6, 4])),
            vector_to_list(vec![7, 0, 8])
        );

        // Test case 2: Adding two empty lists
        assert_eq!(
            add_two_numbers(vector_to_list(vec![0]), vector_to_list(vec![0])),
            vector_to_list(vec![0])
        );

        // Test case 3: Adding an empty list with a non-empty list
        assert_eq!(
            add_two_numbers(None, vector_to_list(vec![1])),
            vector_to_list(vec![1])
        );

        // // Test case 4: Adding two lists with different lengths
        assert_eq!(
            add_two_numbers(vector_to_list(vec![1, 2]), vector_to_list(vec![3])),
            vector_to_list(vec![4, 2])
        );

        // Test case 5: Adding two list where one is of a very big number
        assert_eq!(
            add_two_numbers(
                vector_to_list(vec![1, 9, 9, 9, 9, 9, 9, 9, 9, 9]),
                vector_to_list(vec![9])
            ),
            vector_to_list(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1])
        );
    }
}
