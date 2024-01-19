// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']',
// determine if the input string is valid.

// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
// Every close bracket has a corresponding open bracket of the same type.

// Example 1:
// Input: s = "()"
// Output: true

// Example 2:
// Input: s = "()[]{}"
// Output: true

// Example 3:
// Input: s = "(]"
// Output: false

struct Solution {}

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                }
                _ => {
                    let to_match = stack.pop();

                    if to_match.is_none()
                        || c == ')' && to_match.unwrap() != '('
                        || c == ']' && to_match.unwrap() != '['
                        || c == '}' && to_match.unwrap() != '{'
                    {
                        return false;
                    }
                }
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_is_valid() {
        assert_eq!(Solution::is_valid("()".to_string()), true);
        assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
        assert_eq!(Solution::is_valid("(]".to_string()), false);
        assert_eq!(Solution::is_valid("([)]".to_string()), false);
        assert_eq!(Solution::is_valid("{[]}".to_string()), true);
        assert_eq!(Solution::is_valid("]".to_string()), false);
        assert_eq!(Solution::is_valid("(){}}{".to_string()), false);
        assert_eq!(Solution::is_valid("({{{{}}}))".to_string()), false);
    }
}
