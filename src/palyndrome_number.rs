// Given an integer x, return true if x is a
// palindrome, and false otherwise.

// Example 1:
// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.

// Example 2:
// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-.
// Therefore it is not a palindrome.

// Example 3:
// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

fn reverse(to_reverse: &str) -> String {
    to_reverse.chars().rev().collect()
}

pub fn is_number_palindrome(x: i32) -> bool {
    let number_string = x.to_string();

    if number_string == reverse(&number_string) {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::is_number_palindrome;

    #[test]
    fn test_is_number_palindrome() {
        assert_eq!(is_number_palindrome(121), true);
        assert_eq!(is_number_palindrome(-121), false);
        assert_eq!(is_number_palindrome(10), false);
    }
}
