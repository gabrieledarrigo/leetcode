// Write a function to find the longest common prefix string amongst an array of strings.
// If there is no common prefix, return an empty string "".

// Example 1:
// Input: strs = ["flower","flow","flight"]
// Output: "fl"

// Example 2:
// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

use std::string;

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::from("");
        }

        for (i, current_char) in strs[0].chars().enumerate() {
            for current_str in strs.iter().skip(1) {
                if
                // We've reached the end of the current string
                i == current_str.chars().count()

                // The current character does not match the current character in the current string 
                || current_str.chars().nth(i).unwrap() != current_char
                {
                    // Return the current prefix
                    return strs[0][..i].to_string();
                }
            }
        }

        // If we've reached this point, the first string is the longest common prefix
        return strs[0].clone();
    }

    pub fn longest_common_prefix_alternative_version(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::from("");
        }

        // Initialize prefix to first string
        let mut prefix = strs[0].clone();

        // Loop through remaining strings
        for string in strs.iter().skip(1) {
            // Shorten prefix until it matches the current string
            while string.starts_with(&prefix) == false {
                prefix.pop();
            }
        }

        return prefix;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(Solution::longest_common_prefix(vec![]), "".to_string());

        assert_eq!(
            Solution::longest_common_prefix(vec!["dog".to_string()]),
            "dog".to_string()
        );

        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl".to_string()
        );

        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            "".to_string()
        );
    }
}
