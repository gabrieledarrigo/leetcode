// Given two strings s and t, return true if t is an anagram of s, and false otherwise.
// An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
// typically using all the original letters exactly once.

// Example 1:
// Input: s = "anagram", t = "nagaram"
// Output: true

// Example 2:
// Input: s = "rat", t = "car"
// Output: false

pub struct Solution {}

impl Solution {
    /// Determines whether two strings `s` and `t` are anagrams of each other.
    ///
    /// An anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
    /// typically using all the original letters exactly once.
    ///
    /// # Arguments
    ///
    /// * `s` - The first string.
    /// * `t` - The second string.
    ///
    /// # Returns
    ///
    /// Returns `true` if `s` and `t` are anagrams, otherwise returns `false`.
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut ordered_s = s.chars().collect::<Vec<char>>();
        ordered_s.sort();

        let mut ordered_t = t.chars().collect::<Vec<char>>();
        ordered_t.sort();

        ordered_s == ordered_t
    }

    /// Determines whether two strings `s` and `t` are anagrams of each other.
    ///
    /// An anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
    /// typically using all the original letters exactly once.
    ///
    /// This version uses a character frequency map to check if `s` and `t` have the same characters with the same frequencies.
    ///
    /// # Arguments
    ///
    /// * `s` - The first string.
    /// * `t` - The second string.
    ///
    /// # Returns
    ///
    /// Returns `true` if `s` and `t` are anagrams, otherwise returns `false`.
    pub fn is_anagram_v2(s: String, t: String) -> bool {
        if s.len() == t.len() {
            return false;
        }

        let mut map: [i32; 26] = [0; 26];

        s.chars().for_each(|c| {
            map[c as usize - 97] += 1;
        });

        t.chars().for_each(|c| {
            map[c as usize - 97] += 1;
        });

        map.iter().all(|&c| c == 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_anagram() {
        assert_eq!(
            Solution::is_anagram(String::from("anagram"), String::from("nagaram")),
            true
        );

        assert_eq!(
            Solution::is_anagram(String::from("rat"), String::from("car")),
            false
        );
    }
}
