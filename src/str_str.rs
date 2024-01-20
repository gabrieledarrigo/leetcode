// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

// Example 1:
// Input: haystack = "sadbutsad", needle = "sad"
// Output: 0
// Explanation: "sad" occurs at index 0 and 6. The first occurrence is at index 0, so we return 0.

// Example 2:
// Input: haystack = "leetcode", needle = "leeto"
// Output: -1
// Explanation: "leeto" did not occur in "leetcode", so we return -1.

pub struct Solution {}

impl Solution {
    /// Returns the index of the first occurrence of the `needle` string in the `haystack` string.
    /// If the `needle` string is empty, returns 0.
    /// If the `needle` string is not found in the `haystack` string, returns -1.
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.chars().collect::<Vec<char>>();
        let needle = needle.chars().collect::<Vec<char>>();

        if needle.is_empty() {
            return 0;
        }

        for (i, _) in haystack.iter().enumerate() {
            for (j, _) in needle.iter().enumerate() {
                // If the needle is longer than the remaining haystack, return -1.
                if i + j == haystack.len() {
                    return -1;
                }

                // If the current haystack character does not match the current needle character, break and move on to the next haystack character.
                if haystack[i + j] != needle[j] {
                    break;
                }

                // If the current haystack character matches the current needle character and we are at the end of the needle,
                //  return the index of the haystack character.
                if j == needle.len() - 1 {
                    return i as i32;
                }
            }
        }

        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
    }
}
